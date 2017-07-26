#[macro_use]
extern crate clap;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate env_logger;
extern crate digitalocean;

mod subcommand;
mod arg;

use std::env;
use std::str::FromStr;
use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};
use digitalocean::prelude::*;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init().ok();

    let app = Root::app();

    let matches = app.get_matches();

    let api_token = fetch_api_token(&matches).expect("No API token provided");

    let client = DigitalOcean::new(api_token).unwrap();

    Root::handle(client, &matches);
}

trait Module {
    fn app() -> App<'static, 'static>;
    fn handle(client: DigitalOcean, arg_matches: &ArgMatches);
}

struct Root;

impl Module for Root {
    fn app() -> App<'static, 'static> {
        App::new(env!("CARGO_PKG_NAME"))
            .version(crate_version!())
            .author(crate_authors!())
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .global_settings(&[
                             AppSettings::ColoredHelp,
                             AppSettings::GlobalVersion,
                             AppSettings::InferSubcommands,
            ])
            .setting(AppSettings::SubcommandRequired)
            .arg(Arg::with_name("token")
                 .long("token")
                 .short("t")
                 .value_name("TOKEN")
                 .help("The DigitalOcean API key to use.")
                 .takes_value(true))
            .subcommand(subcommand::droplet::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) {
        match arg_matches.subcommand() {
            ("droplet", Some(arg_matches)) => subcommand::droplet::command(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }

    }
}

/// Checks in the following places, in order:
///   - The `--token` flag passed to the program
///   - The environment as `$DO_API_TOKEN`
///   - The config file at `$HOME/.config/ocean/config.toml`
fn fetch_api_token<'a>(matches: &'a ArgMatches) -> Option<&'a str> {
    // TODO: Env/File
    matches.value_of("token")
}

