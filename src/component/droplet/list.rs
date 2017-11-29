use arg;
use clap::{App, ArgMatches};
use component::Component;
use digitalocean::prelude::*;
use failure::Error;

pub struct List;

impl Component for List {
    const DEFAULT_OUTPUT: Option<&'static str> = Some("table");

    fn app() -> App<'static, 'static> {
        App::new("list").about("List droplets.").arg(arg::limit())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        let response = client.execute(Droplet::list())?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
