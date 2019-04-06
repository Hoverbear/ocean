use crate::component::Component;
use clap::{App, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;

pub struct Apply;

impl Component for Apply {
    fn app() -> App<'static, 'static> {
        App::new("apply").about("Apply a configuration to the infrastructure")
    }

    fn handle(_client: DigitalOcean, _arg_matches: &ArgMatches) -> Result<(), Error> {
        unimplemented!();
        // Self::output(Infrastructure::default(), _arg_matches.value_of("output"))?;
        // Ok(())
    }
}
