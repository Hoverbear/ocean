use crate::component::Component;
use clap::{App, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;

pub struct Get;

impl Component for Get {
    const DEFAULT_OUTPUT: Option<&'static str> = Some("debug");

    fn app() -> App<'static, 'static> {
        App::new("get")
            .about("Get detailed info about your account")
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        let response = client.execute(Account::get())?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
