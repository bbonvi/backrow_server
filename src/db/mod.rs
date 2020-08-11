use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

mod channels;
mod emotes;
mod errors;
mod files;
mod messages;
mod roles;
mod rooms;
mod users;
mod videos;

pub use channels::*;
pub use emotes::*;
pub use errors::*;
pub use files::*;
pub use messages::*;
pub use roles::*;
pub use rooms::*;
pub use users::*;
pub use videos::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn get_pool() -> DbPool {
    dotenv().ok();
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
