use clap::{App, ArgMatches};
use error::Result;
use digitalocean::prelude::*;
use {PrintTable, arg};
use component::Component;

pub struct List;

impl Component for List {
    fn app() -> App<'static, 'static> {
        App::new("list").about("List droplets.").arg(arg::limit())
    }

    fn handle(client: DigitalOcean, _arg_matches: &ArgMatches) -> Result<()> {
        let output = client.execute(Droplet::list())?;

        output.print_table();

        Ok(())
    }
}
