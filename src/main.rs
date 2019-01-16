pub mod schema;
pub mod models;
pub mod logic;

#[macro_use] extern crate diesel;

use logic::handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    simple_logger::init_with_level(log::Level::Debug).expect("main::simple_logger::init_with_level() panicked");

    lambda_runtime::lambda!(handler);

    Ok(())
}