use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{get, web, HttpRequest, HttpResponse,  error::ErrorUnauthorized};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use actix_session::{Session, SessionExt};
use std::path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sqlx::MySqlPool;
use serde_json::json;


use crate::models::{User, UserStatistics};
use crate::game::{Player, Game, GameState, SharedGameState, GameStatus};
use crate::db;
use crate::auth;




#[derive(Serialize, Deserialize, Debug)]
struct MessageMove{
    typ: String,
    from: String,
    to: String
}

#[derive(Message)]
#[rtype(result = "()")]
struct IncomingMessage(String);

pub struct ChessSession {
    pub user_id: i32,
    pub elo: i32,
    game_state: SharedGameState,
}   

impl Actor for ChessSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        println!("Nowy gracz dołączył: user_id = {}, elo = {}\n", self.user_id, self.elo);

        // Tworzymy nowego gracza
        let new_player = Player {
            user_id: self.user_id,
            elo: self.elo,
            addr: addr.clone(),
        };

        let mut state = self.game_state.lock().unwrap();

        // Czy ktoś już czeka?
        if let Some(opponent) = state.queue.pop() {
            // Stwórz nową grę
            let game = Game {
                white: opponent.clone(),
                black: new_player.clone(),
                status: GameStatus::InProgress,
            };

            state.games.push(game);

            // Wyślij obu graczom informację o starcie gry
            let start_msg = serde_json::json!({
                "type": "game_start",
                "color": "white"
            })
            .to_string();
            opponent.addr.do_send(IncomingMessage(start_msg));

            let start_msg = serde_json::json!({
                "type": "game_start",
                "color": "black"
            })
            .to_string();
            addr.do_send(IncomingMessage(start_msg));
        } else {
            // Nikt nie czeka — dodaj do kolejki
            state.queue.push(new_player);

            let waiting_msg = serde_json::json!({
                "type": "waiting_for_opponent"
            })
            .to_string();
            ctx.text(waiting_msg);
        }

        // Ping co 10 sek.
        ctx.run_interval(Duration::from_secs(10), |_, ctx| {
            ctx.ping(b"");
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        let mut state = self.game_state.lock().unwrap();

        println!("Gracz {} rozłączył się", self.user_id);

        // 1. Usuń z kolejki (jeśli tam był)
        state.queue.retain(|p| p.user_id != self.user_id);

        // 2. Znajdź grę, w której uczestniczył
        if let Some(index) = state.games.iter().position(|g| {
            g.white.user_id == self.user_id || g.black.user_id == self.user_id
        }) {
            let game = state.games.remove(index);

            // 3. Ustal kto był przeciwnikiem
            let opponent = if game.white.user_id == self.user_id {
                game.black
            } else {
                game.white
            };

            // 4. Wyślij przeciwnikowi wiadomość o zakończeniu gry
            let info = json!({
                "type": "game_ended",
                "reason": "opponent_disconnected"
            });

            if let Ok(msg_str) = serde_json::to_string(&info) {
                opponent.addr.do_send(IncomingMessage(msg_str));
            }

            // (opcjonalnie: log)
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
                // Spróbuj sparsować ruch
                let parsed = serde_json::from_str::<MessageMove>(&text);
                if parsed.is_err() {
                    ctx.text("Niepoprawny format JSON.");
                    return;
                }

                let move_msg = parsed.unwrap();
                let serialized = serde_json::to_string(&move_msg).unwrap();

                let state = self.game_state.lock().unwrap();

                // Znajdź grę, w której uczestniczy ten gracz
                if let Some(game) = state.games.iter().find(|g| {
                    g.white.user_id == self.user_id || g.black.user_id == self.user_id
                }) {
                    // Znajdź przeciwnika
                    let opponent = if game.white.user_id == self.user_id {
                        &game.black
                    } else {
                        &game.white
                    };

                    // Wyślij ruch do przeciwnika
                    opponent.addr.do_send(IncomingMessage(serialized));
                } else {
                    ctx.text("Nie znaleziono gry.");
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
async fn websocket_handler(req: HttpRequest, path: web::Path<String>, stream: web::Payload, db_pool: web::Data<MySqlPool>,  data: web::Data<SharedGameState>) -> actix_web::Result<HttpResponse> {
    print!("WebSocket connection request received\n");

    // let session = req.get_session();

    print!("HOW TO KILL MYSELF");


    // if session.get::<i32>("user_id").unwrap().is_none() {
        // print!("RUCHAM CI MATKETY KURWA JEBANEA");
        // return Ok(HttpResponse::Unauthorized().finish());
    // }

    print!("CHUJJJJJJJJJJJJJJJJJJJJ");

    // let user_id = session.get::<i32>("user_id").unwrap().unwrap();
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

    match user_stats_result {
        Ok(Some(stats)) => {
            println!("User stats retrieved: elo = {}", stats.elo);
            let ws = ChessSession {
                user_id,
                elo: stats.elo,
                game_state: data.get_ref().clone(),
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
            println!("Nie znaleziono użytkownika w bazie danych: user_id = {}", user_id);
            Ok(HttpResponse::InternalServerError().finish())
        }
        Err(e) => {
            println!("Błąd zapytania do bazy danych: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}