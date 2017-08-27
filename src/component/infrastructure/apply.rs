use clap::{App, Arg, ArgMatches};
use component::Component;
use digitalocean::prelude::*;
use error::{Result, ResultExt};
use super::Infrastructure;

pub struct Apply;

impl Component for Apply {
    fn app() -> App<'static, 'static> {
        App::new("apply")
            .about("Apply a configuration to the infrastructure.")
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        unimplemented!();

        Self::output(Infrastructure::default(), arg_matches.value_of("output"))?;

        Ok(())
    }
}
