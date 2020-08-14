#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;

#[macro_use]
extern crate log;

pub mod db;
mod debug;
pub mod env;
pub mod schema;
pub mod server;
pub mod vars;

fn main() {
    dotenv().ok();
    debug::init();
    server::run().unwrap();
}
