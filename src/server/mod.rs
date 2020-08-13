extern crate num_cpus;

use crate::db;
use crate::env;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};

pub mod asserts;
pub mod errors;
pub mod helpers;
mod users;
mod ws;

/// "first line of request", "ip", "status code", "user-agent"
const LOGGER_FORMAT: &str = "\"%r\", \"%a\", \"%s\", \"%{User-Agent}i\"";

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    let pool = db::get_pool();
    let addr = env::APP_ADDR.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(true),
            ))
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
                            .route("", web::get().to(users::get))
                            .route("/signin", web::post().to(users::sign_in))
                            .route("/signup", web::post().to(users::sign_up))
                            .route("/logout", web::post().to(users::log_out)),
                    ),
            )
    })
    .workers(num_cpus::get() * 2)
    .bind(addr)?
    .run()
    .await
}
