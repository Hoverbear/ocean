use super::Infrastructure;
use crate::component::Component;
use clap::{App, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;

pub struct Get;

impl Component for Get {
    const DEFAULT_OUTPUT: Option<&'static str> = Some("debug");

    fn app() -> App<'static, 'static> {
        App::new("get").about("Get the current state of the infrastructure")
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        let mut infra = Infrastructure::default();

        infra.domains = client.execute(Domain::list())?;
        infra.droplets = client.execute(Droplet::list())?;

        Self::output(infra, arg_matches.value_of("output"))?;

        Ok(())
    }
}
