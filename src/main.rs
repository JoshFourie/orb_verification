mod schema;
mod models;
mod logic;

// #[macro_use] is a feature that is being gradually deprecated in Rust 2018, see Diesel open_issue: https://github.com/diesel-rs/diesel/issues/1764.
#[macro_use] extern crate diesel;

// the next step will be a farirly lengthy matching algorithm for the various orbs we would want ot verify so there is a single exposed handler for convenience.
use crate::logic::exposed_handler;

// simple_logger logs errors for AWS lambda, and the lambda_runtime macro lambda! exposes the 'logic' mod to AWS Lambda.
fn main() -> Result<(), Box<dyn std::error::Error>> {

    simple_logger::init_with_level(log::Level::Debug).expect("main::simple_logger::init_with_level() panicked");

    lambda_runtime::lambda!(exposed_handler);

    Ok(())
}

