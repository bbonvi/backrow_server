use crate::db;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
// use std::env;
use crate::env;

pub mod errors;
mod ws;

// /api/rooms
// /api/rooms/kekw
// /api/rooms/kekw/ws
// /api/rooms/kekw/ban/0001

/// "first line of request", "ip", "status code", "user-agent"
const LOGGER_FORMAT: &str = "\"%r\", \"%a\", \"%s\", \"%{User-Agent}i\"";

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    let pool = db::get_pool();
    let addr = env::APP_ADDR.clone();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::new(LOGGER_FORMAT))
            // blank route for dev purposes
            .service(web::scope("/blank").route("", web::get().to(|| HttpResponse::Ok())))
            .service(
                web::scope("/api").service(
                    web::scope("/rooms/{room_path}").route("/ws", web::get().to(ws::index)),
                ),
            )
    })
    .bind(addr)?
    .run()
    .await
}
