use crate::db;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
// use std::env;
use crate::env;

pub mod asserts;
pub mod errors;
pub mod helpers;
mod ws;

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
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/rooms")
                            .service(
                                web::scope("/{room_path}")
                                    .route("/ws", web::get().to(ws::index))
                                    .route("/action", web::get().to(HttpResponse::Ok)),
                            )
                            .route("", web::get().to(HttpResponse::Ok)),
                    )
                    .service(
                        web::scope("/users")
                            .route("/signin", web::get().to(HttpResponse::Ok))
                            .route("/signup", web::get().to(HttpResponse::Ok)),
                    ),
            )
    })
    .bind(addr)?
    .run()
    .await
}
