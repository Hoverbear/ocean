use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use arg;

pub fn app() -> App<'static, 'static> {
    App::new("droplet")
        .about("Interact with droplets.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("list")
                    .about("List.")
                    .arg(arg::limit()))
        .subcommand(SubCommand::with_name("get")
                    .about("Get.")
                    .arg(arg::id()))

}

pub fn command(client: DigitalOcean, arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        ("list", Some(arg_matches)) => println!("Got a list command"),
        _ => panic!("Unknown subcommand provided"),
    }
}
