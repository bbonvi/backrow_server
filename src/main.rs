#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;

pub mod db;
mod debug;

fn main() {
    dotenv().ok();

    debug::init();
    println!("Hello, world!");
}
