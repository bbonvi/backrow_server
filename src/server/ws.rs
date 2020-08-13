use super::asserts;
use crate::db;
use crate::server::errors::ResponseError;
use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse};
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
    pool: web::Data<db::DbPool>,
    stream: web::Payload,
    info: web::Path<Info>,
) -> Result<HttpResponse, ResponseError> {
    if !asserts::valid_origin(&req) {
        #[cfg(not(debug_assertions))]
        return Err(ResponseError::AccessError("Bad origin"));
    }

    let conn = pool.get().unwrap();
    let room_path = info.room_path.clone();
    let _room = db::Room::by_path(&room_path, &conn)?;

    ws::start(WebSocket {}, &req, stream).map_err(From::from)
}
