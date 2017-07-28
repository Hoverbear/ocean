use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use {PrintTable, arg};
use component::Component;

pub struct List;

impl Component for List {
    fn app() -> App<'static, 'static> {
        App::new("list")
            .about("List droplets.")
            .arg(arg::limit())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) {
        let output = client.execute(Droplet::list()).unwrap();

        output.print_table()
    }
}

