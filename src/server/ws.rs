use super::asserts;
use super::States;
use crate::db;
use actix::{Actor, StreamHandler};
use actix_identity::Identity;
use actix_web::web::{Path, Payload};
use actix_web::HttpRequest;
use actix_web_actors::ws;
use serde::Deserialize;

struct WebSocket;

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Info {
    room_path: String,
}

pub async fn index(
    req: HttpRequest,
    states: States,
    stream: Payload,
    info: Path<Info>,
    _id: Identity,
) -> super::RouteResult {
    if !asserts::valid_origin(&req) {
        #[cfg(not(debug_assertions))]
        return Err(ResponseError::AccessError("Bad origin"));
    }

    let conn = states.pool.get().unwrap();
    let room_path = info.room_path.clone();
    let _room = db::Room::by_path(&room_path, &conn)?;

    ws::start(WebSocket {}, &req, stream).map_err(From::from)
}
