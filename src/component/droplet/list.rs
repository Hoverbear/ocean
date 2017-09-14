use arg;
use clap::{App, ArgMatches};
use component::Component;
use digitalocean::prelude::*;
use error::Result;

pub struct List;

impl Component for List {
    const DEFAULT_OUTPUT: Option<&'static str> = Some("table");

    fn app() -> App<'static, 'static> {
        App::new("list").about("List droplets.").arg(arg::limit())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        let response = client.execute(Droplet::list())?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
