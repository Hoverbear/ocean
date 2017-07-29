use clap::{App, ArgMatches};
use digitalocean::prelude::*;
use {arg, PrintTable};
use component::Component;
use error::Result;

pub struct List;

impl Component for List {
    fn app() -> App<'static, 'static> {
        App::new("list").about("List domains.").arg(arg::limit())
    }

    fn handle(client: DigitalOcean, _arg_matches: &ArgMatches) -> Result<()> {
        let output = client.execute(Domain::list())?;

        output.print_table();

        Ok(())
    }
}
