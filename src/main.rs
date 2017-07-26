#[macro_use]
extern crate clap;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate env_logger;
extern crate digitalocean;
extern crate prettytable;

mod component;
mod arg;

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


/// Checks in the following places, in order:
///   - The `--token` flag passed to the program
///   - The environment as `$DO_API_TOKEN`
///   - The config file at `$HOME/.config/ocean/config.toml`
fn fetch_api_token<'a>(matches: &'a ArgMatches) -> Option<String> {
    // TODO: Env/File
    if let Some(val) = matches.value_of("token") {
        Some(val.into())
    } else if let Ok(val) = env::var("DO_API_TOKEN") {
        Some(val)
    } else {
        None
    }
}


