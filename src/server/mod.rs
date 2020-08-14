extern crate num_cpus;

use crate::db;
use crate::env;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};

pub mod asserts;
pub mod auth;
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

    const YEAR_IN_SECS: i64 = 60 * 60 * 24 * 365;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .max_age(YEAR_IN_SECS)
                    .secure(true),
            ))
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
                            .route("/signin/discord", web::get().to(users::sign_in_discord))
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
