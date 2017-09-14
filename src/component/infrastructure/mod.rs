use AsTable;
use clap::{App, AppSettings, ArgMatches};
use component::Component;
use digitalocean::prelude::*;
use error::Result;

mod get;
pub use self::get::Get;
mod apply;
pub use self::apply::Apply;

pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new("infrastructure")
            .about("Interact with the entire infrastructure.")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(Get::app())
            .subcommand(Apply::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        match arg_matches.subcommand() {
            ("get", Some(arg_matches)) => Get::handle(client, arg_matches),
            ("apply", Some(arg_matches)) => Apply::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Infrastructure {
    droplets: Vec<Droplet>,
    domains: Vec<Domain>,
}

impl AsTable for Infrastructure {
    fn as_table(&self) {
        println!("Droplets:");
        self.droplets.as_table();
        println!("Domains:");
        self.domains.as_table();
    }
}
