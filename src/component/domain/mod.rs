use clap::{App, Arg, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use arg;
use component::Component;

mod list;
pub use self::list::List;

pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new("domain")
            .about("Interact with domains.")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(List::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) {
        match arg_matches.subcommand() {
            ("list", Some(arg_matches)) => List::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }
    }
}

