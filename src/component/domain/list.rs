use crate::{arg, component::Component};
use clap::{App, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;

pub struct List;

impl Component for List {
    const DEFAULT_OUTPUT: Option<&'static str> = Some("table");

    fn app() -> App<'static, 'static> {
        App::new("list").about("List domains").arg(arg::limit())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        let response = client.execute(Domain::list())?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
