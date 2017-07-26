use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use arg;
use component::Component;

mod list;
pub use self::list::List;

pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new("droplet")
            .about("Interact with droplets.")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(List::app())
            .subcommand(SubCommand::with_name("get")
                        .about("Get.")
                        .arg(arg::id()))
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) {
        match arg_matches.subcommand() {
            ("list", Some(arg_matches)) => List::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }
    }
}
