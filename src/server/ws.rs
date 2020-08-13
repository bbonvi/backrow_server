use crate::db;
use crate::env;
use crate::server::errors::ServerError;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde::Deserialize;
use uuid::Uuid;

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
) -> Result<HttpResponse, ServerError> {
    let app_origin = env::APP_ORIGIN.clone();

    let origin = req
        .headers()
        .get("origin")
        .map(|o| o.to_str().unwrap_or_default())
        .unwrap_or_default();

    if String::from(origin).contains(&app_origin) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let conn = pool.get().unwrap();

    debug!("{:?}", origin);
    let room_path = info.room_path.clone();

    let room = db::Room::by_path(&room_path, &conn)?;
    debug!("{:?}", room);

    ws::start(WebSocket {}, &req, stream).map_err(From::from)
}
