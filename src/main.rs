#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate env_logger;
extern crate digitalocean;
extern crate prettytable;
#[macro_use]
extern crate error_chain;
extern crate serde_json;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod component;
mod arg;
mod error;

use clap::ArgMatches;
use component::Component;
use digitalocean::prelude::*;
use std::{env, process};
use prettytable::Table;
use prettytable::cell::Cell;
use prettytable::row::Row;
use error_chain::ChainedError;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let app = component::Root::app();

    let matches = app.get_matches();

    let api_token = fetch_api_token(&matches).expect("No API token found");

    let client = DigitalOcean::new(api_token).unwrap();

    if let Err(e) = component::Root::handle(client, &matches) {
        println!("{}", e.display_chain());
        process::exit(1);
    }
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
pub trait AsTable {
    fn as_table(&self);
}

impl<'a> AsTable for Vec<&'a str> {
    fn as_table(&self) {
        let mut table = Table::new();

        table.set_format(
            *prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR,
        );
        table.set_titles(Row::new(vec![
            Cell::new("value"),
        ]));

        for row in self {
            table.add_row(Row::new(vec![
                Cell::new(&format!("{}", row)),
            ]));
        }

        table.printstd();
    }
}
