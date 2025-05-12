use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde_json;

#[derive(Default)]
pub struct GameState {
    white_player: Option<Addr<MyWebSocket>>,
    black_player: Option<Addr<MyWebSocket>>,
}

type SharedGameState = Arc<Mutex<GameState>>;


#[derive(Serialize, Deserialize, Debug)]
struct MessageMove{
    from: String,
    to: String
}

#[derive(Message)]
#[rtype(result = "()")]
struct IncomingMessage(String);

pub struct MyWebSocket {
    color: String,
    game_state: SharedGameState,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let mut state = self.game_state.lock().unwrap();

        match self.color.as_str() {
            "white" => {
                state.white_player = Some(addr.clone());

            }
            "black" => {
                state.black_player = Some(addr.clone());
            }
            _ => {
                ctx.text("Nieznany kolor – użyj white lub black.");
                ctx.close(None);
                ctx.stop();
            }
        }
        ctx.run_interval(Duration::from_secs(10), |_, ctx| {
            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let state = self.game_state.lock().unwrap();
        match msg {
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<MessageMove>(&text) {
                    Ok(parsed_msg) => {
                        // println!("Odebrano ruch: {:?}", parsed_msg);

                        let json_str = serde_json::to_string(&parsed_msg).unwrap();

                        if self.color == "white" {
                            if let Some(enemy) = &state.black_player {
                                enemy.do_send(IncomingMessage(json_str));
                            } else {
                                ctx.text("Brak przeciwnika.");
                            }
                        } else if self.color == "black" {
                            if let Some(enemy) = &state.white_player {
                                enemy.do_send(IncomingMessage(json_str));
                            } else {
                                ctx.text("Brak przeciwnika.");
                            }
                        }
                    }
                    Err(_) => {
                        ctx.text("Niepoprawny format JSON.");
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl Handler<IncomingMessage> for MyWebSocket {
    type Result = ();
    fn handle(&mut self, msg: IncomingMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

#[get("/ws/{color}")]
async fn websocket_handler(path: web::Path<String>, req: HttpRequest, stream: web::Payload, data: web::Data<SharedGameState>) -> actix_web::Result<HttpResponse> {
    // println!(">>> URUCHOMIONO handler WebSocket! <<<");

    let color = path.into_inner();
    // println!("WebSocket connection established with color: {}", color);
    let ws = MyWebSocket {
        color,
        game_state: data.get_ref().clone(),
    };
    match ws::start(ws, &req, stream) {
        Ok(resp) => Ok(resp),
        Err(e) => {
            println!("Błąd przy ws::start: {:?}", e);
            Err(e.into())
        }
    }
}