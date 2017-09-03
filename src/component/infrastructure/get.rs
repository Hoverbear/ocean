
use super::Infrastructure;
use clap::{App, Arg, ArgMatches};
use component::Component;
use digitalocean::prelude::*;
use error::{Result, ResultExt};

pub struct Get;

impl Component for Get {
    fn app() -> App<'static, 'static> {
        App::new("get").about("Get the current state of the infrastructure.")
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        let mut infra = Infrastructure::default();

        infra.domains = client.execute(Domain::list())?;
        infra.droplets = client.execute(Droplet::list())?;

        Self::output(infra, arg_matches.value_of("output"))?;

        Ok(())
    }
}
