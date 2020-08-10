#[macro_use]
extern crate diesel;
extern crate dotenv;

mod debug;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    debug::init();
    println!("Hello, world!");
}
