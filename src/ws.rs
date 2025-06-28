use actix::fut::{ActorFutureExt, wrap_future};
use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_session::{Session, SessionExt};
use actix_web::{HttpRequest, HttpResponse, error::ErrorUnauthorized, get, web};
use actix_web_actors::ws;
use argon2::password_hash::rand_core::le;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::MySqlPool;
use std::path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::auth;
use crate::db;
use crate::game::{Game, GameState, GameStatus, Player, SharedGameState};
use crate::models::{User, UserStatistics};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "msg_type")]
enum ClientMessage {
    #[serde(rename = "move")]
    Move {
        from: String,
        to: String,
        pgn: String,
    },
    #[serde(rename = "chat")]
    Chat { message: String },

    #[serde(other)]
    Unknown,
}
#[derive(Message)]
#[rtype(result = "()")]
struct IncomingMessage(String);

pub struct ChessSession {
    pub user_id: i32,
    pub username: String,
    pub elo: i32,
    pub game_state: SharedGameState,
    pub db_pool: MySqlPool,
}

impl Actor for ChessSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        println!(
            "Nowy gracz dołączył: user_id = {}, elo = {}\n",
            self.user_id, self.elo
        );

        let new_player = Player {
            user_id: self.user_id,
            username: self.username.clone(),
            elo: self.elo,
            addr: addr.clone(),
        };

        let game_state = self.game_state.clone();
        let db_pool = self.db_pool.clone();

        actix::spawn(async move {
            let mut state = game_state.lock().unwrap();

            if let Some(opponent) = state.queue.pop() {
                let game_id = match db::create_game(
                    &db_pool,
                    opponent.user_id,
                    new_player.user_id,
                    opponent.elo,
                    new_player.elo,
                )
                .await
                {
                    Ok(id) => id,
                    Err(e) => {
                        println!("Błąd przy zapisie gry do DB: {:?}", e);
                        return;
                    }
                };

                let game = Game::new(game_id, opponent.clone(), new_player.clone());
                state.games.push(game);

                let start_msg_white = serde_json::json!({
                    "msg_type": "game_status",
                    "status": "playing",
                    "color": "white",
                    "opponent_username": new_player.username,
                    "opponent_elo": new_player.elo,
                })
                .to_string();
                opponent.addr.do_send(IncomingMessage(start_msg_white));

                let start_msg_black = serde_json::json!({
                    "msg_type": "game_status",
                    "status": "playing",
                    "color": "black",
                    "opponent_username": opponent.username,
                    "opponent_elo": opponent.elo,
                })
                .to_string();
                addr.do_send(IncomingMessage(start_msg_black));
            } else {
                state.queue.push(new_player);

                let waiting_msg = serde_json::json!({
                    "msg_type": "game_status",
                    "status": "waiting",
                })
                .to_string();
                addr.do_send(IncomingMessage(waiting_msg));
            }
        });

        ctx.run_interval(Duration::from_secs(10), |_, ctx| {
            ctx.ping(b"");
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        let mut state = self.game_state.lock().unwrap();

        println!("Gracz {} rozłączył się", self.user_id);

        state.queue.retain(|p| p.user_id != self.user_id);

        if let Some(index) = state
            .games
            .iter()
            .position(|g| g.white.user_id == self.user_id || g.black.user_id == self.user_id)
        {
            let mut game = state.games.remove(index);

            if game.status == GameStatus::InProgress {
                if game.white.user_id == self.user_id {
                    game.status = GameStatus::BlackWin;
                    game.black.addr.do_send(IncomingMessage(
                        json!({ "msg_type": "game_status", "status": "win" }).to_string(),
                    ));
                } else {
                    game.status = GameStatus::WhiteWin;
                    game.white.addr.do_send(IncomingMessage(
                        json!({ "msg_type": "game_status", "status": "win" }).to_string(),
                    ));
                }
            }

            let db_pool = self.db_pool.clone();
            let game_id = game.game_id;
            let result = game.status.clone();
            let white_player_id = game.white.user_id;
            let black_player_id = game.black.user_id;

            actix::spawn(async move {
                if let Err(e) = db::end_game(&db_pool, game_id, &result).await {
                    eprintln!("Błąd kończenia gry w DB: {:?}", e);
                }

                if let Err(e) =
                    db::update_users_statistics(&db_pool, white_player_id, black_player_id, &result)
                        .await
                {
                    eprintln!("Błąd aktualizacji statystyk użytkowników: {:?}", e);
                }
            });

            println!("Gracz {} rozłączył się, gra zakończona", self.user_id);
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChessSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(m) => m,
            Err(_) => return,
        };

        match msg {
            ws::Message::Text(text) => {
                let parsed = serde_json::from_str::<ClientMessage>(&text);
                if parsed.is_err() {
                    ctx.text("Niepoprawny format JSON.");
                    return;
                }

                match parsed.unwrap() {
                    ClientMessage::Move { from, to, pgn } => {
                        let mut state = self.game_state.lock().unwrap();

                        if let Some(game) = state.games.iter_mut().find(|g| {
                            g.white.user_id == self.user_id || g.black.user_id == self.user_id
                        }) {
                            match game.validate_and_play_move(self.user_id, &from, &to) {
                                Ok(()) => {
                                    let opponent = if game.white.user_id == self.user_id {
                                        &game.black
                                    } else {
                                        &game.white
                                    };

                                    let msg = json!({
                                        "msg_type": "move",
                                        "from": from,
                                        "to": to,
                                    });

                                    opponent.addr.do_send(IncomingMessage(msg.to_string()));

                                    let player_color = if game.white.user_id == self.user_id {
                                        "white"
                                    } else {
                                        "black"
                                    };

                                    let db_pool = self.db_pool.clone();
                                    let game_id = game.game_id;
                                    let player_color_owned = player_color.to_string();
                                    let pgn = pgn.clone();

                                    let move_number = game.move_number;

                                    ctx.spawn(
                                        wrap_future(async move {
                                            db::add_move_to_game(
                                                &db_pool,
                                                game_id,
                                                &player_color_owned,
                                                move_number,
                                                &pgn,
                                            )
                                            .await
                                        })
                                        .then(
                                            |res, _actor, _ctx| {
                                                if let Err(e) = res {
                                                    eprintln!(
                                                        "Błąd dodawania ruchu do DB: {:?}",
                                                        e
                                                    );
                                                }
                                                actix::fut::ready(())
                                            },
                                        ),
                                    );

                                    if player_color == "black" {
                                        game.move_number += 1;
                                    }

                                    if game.status != GameStatus::InProgress {
                                        let db_pool = self.db_pool.clone();
                                        let game_id = game.game_id;
                                        let result = game.status.clone();
                                        let white_player_id = game.white.user_id;
                                        let black_player_id = game.black.user_id;

                                        // ctx.spawn(db::end_game(&db_pool, game_id, &result));
                                        // ctx.spawn(db::update_users_statistics(&db_pool, white_player_id, black_player_id, &game.status));

                                        ctx.spawn(
                                        wrap_future({
                                                async move {
                                                    if let Err(e) = db::end_game(&db_pool, game_id, &result).await {
                                                        eprintln!("Błąd kończenia gry w DB: {:?}", e);
                                                    }

                                                    if let Err(e) = db::update_users_statistics(&db_pool, white_player_id, black_player_id, &result).await {
                                                        eprintln!("Błąd aktualizacji statystyk użytkowników: {:?}", e);
                                                    }
                                                    Ok::<(), ()>(())
                                                }
                                            })
                                            .then(|_res, _actor, _ctx| {
                                                actix::fut::ready(())
                                            }),
                                        );

                                        let (white_msg, black_msg) = match game.status {
                                            GameStatus::WhiteWin => (
                                                json!({ "msg_type": "game_status", "status": "win" }),
                                                json!({ "msg_type": "game_status", "status": "lose" }),
                                            ),
                                            GameStatus::BlackWin => (
                                                json!({ "msg_type": "game_status", "status": "lose" }),
                                                json!({ "msg_type": "game_status", "status": "win" }),
                                            ),
                                            GameStatus::Draw => (
                                                json!({ "msg_type": "game_status", "status": "draw" }),
                                                json!({ "msg_type": "game_status", "status": "draw" }),
                                            ),
                                            _ => return,
                                        };

                                        game.white
                                            .addr
                                            .do_send(IncomingMessage(white_msg.to_string()));
                                        game.black
                                            .addr
                                            .do_send(IncomingMessage(black_msg.to_string()));
                                    }
                                }
                                Err((reason, fen)) => {
                                    ctx.text(
                                        json!({
                                            "msg_type": "invalid_move",
                                            "reason": reason,
                                            "fen": fen,
                                        })
                                        .to_string(),
                                    );
                                }
                            }
                        } else {
                            ctx.text("Nie znaleziono gry.");
                        }
                    }

                    ClientMessage::Chat { message } => {
                        let state = self.game_state.lock().unwrap();

                        if let Some(game) = state.games.iter().find(|g| {
                            g.white.user_id == self.user_id || g.black.user_id == self.user_id
                        }) {
                            let opponent = if game.white.user_id == self.user_id {
                                &game.black
                            } else {
                                &game.white
                            };

                            let db_pool = self.db_pool.clone();
                            let game_id = state
                                .games
                                .iter()
                                .find(|g| {
                                    g.white.user_id == self.user_id
                                        || g.black.user_id == self.user_id
                                })
                                .map(|g| g.game_id);
                            let chat_message = message.clone();
                            let sender_id = self.user_id;

                            ctx.spawn(
                                wrap_future(async move {
                                    db::add_chat_message(&db_pool, game_id.unwrap(), sender_id, &chat_message)
                                        .await
                                })
                                .then(|res, _actor, _ctx| {
                                    if let Err(e) = res {
                                        eprintln!("Błąd dodawania wiadomości do DB: {:?}", e);
                                    }
                                    actix::fut::ready(())
                                }),
                            );

                            let msg = json!({
                                "msg_type": "chat",
                                "message": message,
                            });

                            opponent.addr.do_send(IncomingMessage(msg.to_string()));
                        } else {
                            ctx.text("Nie znaleziono gry.");
                        }
                    }

                    ClientMessage::Unknown => {
                        ctx.text("Nieznany typ wiadomości.");
                    }
                }
            }

            ws::Message::Ping(msg) => ctx.pong(&msg),

            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }

            _ => {}
        }
    }
}

impl Handler<IncomingMessage> for ChessSession {
    type Result = ();
    fn handle(&mut self, msg: IncomingMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

#[get("/ws/{token}")]
async fn websocket_handler(
    req: HttpRequest,
    path: web::Path<String>,
    stream: web::Payload,
    db_pool: web::Data<MySqlPool>,
    data: web::Data<SharedGameState>,
) -> actix_web::Result<HttpResponse> {
    let token = path.into_inner();

    let user_id: i32;

    match auth::verify_jwt(&token) {
        Ok(token_data) => {
            user_id = token_data.claims.sub;
        }
        Err(err) => {
            println!("\n\n\nJWT błąd: {:?}", err);
            return Err(ErrorUnauthorized("Nieprawidłowy token"));
        }
    }

    let user_stats_result = db::get_user_statistics(&db_pool, user_id).await;
    let user = db::get_user_by_id(&db_pool, user_id).await;

    match user_stats_result {
        Ok(Some(stats)) => {
            println!("User stats retrieved: elo = {}", stats.elo);
            let ws = ChessSession {
                user_id,
                username: user.unwrap().unwrap().username,
                elo: stats.elo,
                game_state: data.get_ref().clone(),
                db_pool: db_pool.get_ref().clone(),
            };
            match ws::start(ws, &req, stream) {
                Ok(resp) => {
                    println!("WS Started: user_id = {}, elo = {}", user_id, stats.elo);
                    Ok(resp)
                }
                Err(e) => {
                    println!("Błąd przy ws::start: {:?}", e);
                    Err(e.into())
                }
            }
        }
        Ok(None) => {
            println!(
                "Nie znaleziono użytkownika w bazie danych: user_id = {}",
                user_id
            );
            Ok(HttpResponse::InternalServerError().finish())
        }
        Err(e) => {
            println!("Błąd zapytania do bazy danych: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
