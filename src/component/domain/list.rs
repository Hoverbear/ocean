use clap::{App, ArgMatches};
use digitalocean::prelude::*;
use arg;
use component::Component;
use error::Result;

pub struct List;

impl Component for List {
    fn app() -> App<'static, 'static> {
        App::new("list").about("List domains.").arg(arg::limit())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        let response = client.execute(Domain::list())?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
