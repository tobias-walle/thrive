use std::sync::{Arc, Mutex};

use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use thrive_core::state::State;

struct Websocket {
    state: web::Data<Mutex<State<'static>>>,
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
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<Mutex<State<'static>>>,
) -> Result<HttpResponse, Error> {
    dbg!(&state);
    let response = ws::start(Websocket { state }, &req, stream);
    dbg!(&response);
    response
}
