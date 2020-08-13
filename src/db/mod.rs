use crate::env;
use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

mod audit_logs;
mod channels;
mod emotes;
mod errors;
mod files;
pub mod helpers;
mod messages;
mod restrains;
mod roles;
mod rooms;
mod users;
mod videos;

pub use audit_logs::*;
pub use channels::*;
pub use emotes::*;
pub use errors::*;
pub use files::*;
pub use messages::*;
pub use restrains::*;
pub use roles::*;
pub use rooms::*;
pub use users::*;
pub use videos::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn get_pool() -> DbPool {
    let db_url = env::DATABASE_URL.clone();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
