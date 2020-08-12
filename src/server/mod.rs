use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use crate::db;
use std::env;

const LOGGER_FORMAT: &str = r#""%r", "%a", "%b", "%D", "%s", "%{User-Agent}i""#;

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    let pool = db::get_pool();
    let addr = env::var("APP_ADDR").expect("PORT must be set");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::new(LOGGER_FORMAT))
            .service(
                web::scope("/api")
                           .route("/test", web::get().to(|| HttpResponse::Ok()))
           )
    })
    .bind(addr)?
    .run()
    .await
}
