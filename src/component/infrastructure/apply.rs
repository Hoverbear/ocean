use super::Infrastructure;
use crate::component::Component;
use clap::{App, Arg, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;

pub struct Apply;

impl Component for Apply {
    fn app() -> App<'static, 'static> {
        App::new("apply").about("Apply a configuration to the infrastructure")
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        unimplemented!();

        Self::output(Infrastructure::default(), arg_matches.value_of("output"))?;

        Ok(())
    }
}
