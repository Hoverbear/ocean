#[macro_use]
extern crate clap;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate env_logger;
extern crate digitalocean;
extern crate prettytable;
#[macro_use]
extern crate error_chain;


mod component;
mod arg;
mod error;

use error::{Result, Error, ErrorKind};
use std::env;
use clap::ArgMatches;
use digitalocean::prelude::*;
use component::Component;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let app = component::Root::app();

    let matches = app.get_matches();

    let api_token = fetch_api_token(&matches).expect("No API token found");

    let client = DigitalOcean::new(api_token).unwrap();

    component::Root::handle(client, &matches);
}


// Checks in the following places, in order:
//   - The `--token` flag passed to the program
//   - The environment as `$DO_API_TOKEN` (which may be loaded from `.env`)
fn fetch_api_token<'a>(matches: &'a ArgMatches) -> Option<String> {
    if let Some(val) = matches.value_of("token") {
        Some(val.into())
    } else if let Ok(val) = env::var("DO_API_TOKEN") {
        Some(val)
    } else {
        None
    }
}

// Most commands output a table, this is their common interface.
trait PrintTable {
    fn print_table(&self);
}

trait PrintDetail {
    fn print_detail(&self);
}
