use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use thrive_core::command::Command;

use crate::ServerState;

struct Websocket {
    state: web::Data<ServerState>,
}

impl Actor for Websocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Started WS")
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Stopped WS")
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Websocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        dbg!(&msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<Command>(&text) {
                    Ok(command) => {
                        self.state
                            .write()
                            .unwrap()
                            .apply_command(command)
                            .expect("Couldn't apply command.");
                    }
                    Err(err) => log::warn!("Couln't parse incoming message: {}", err),
                };
                ctx.text(text)
            }
            _ => (),
        }
    }
}

pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<ServerState>,
) -> Result<HttpResponse, Error> {
    let response = ws::start(Websocket { state }, &req, stream);
    dbg!(&response);
    response
}
