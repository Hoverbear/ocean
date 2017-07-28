use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use {arg, PrintTable};
use component::Component;

pub struct List;

impl Component for List {
    fn app() -> App<'static, 'static> {
        App::new("list")
            .about("List domains.")
            .arg(arg::limit())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) {
        let output = client.execute(Domain::list()).unwrap();

        output.print_table();
    }
}

