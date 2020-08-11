use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

mod channels;
mod errors;
mod rooms;
mod users;
mod messages;

pub use channels::*;
pub use errors::*;
pub use rooms::*;
pub use users::*;
pub use messages::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn get_pool() -> DbPool {
    dotenv().ok();
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
