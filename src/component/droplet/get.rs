use crate::component::Component;
use clap::{App, Arg, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;
use std::num;

pub struct Get;

impl Component for Get {
    const DEFAULT_OUTPUT: Option<&'static str> = Some("debug");

    fn app() -> App<'static, 'static> {
        App::new("get")
            .about("Get detailed info about droplets")
            .arg(
                Arg::with_name("droplet")
                    .value_name("DROPLET")
                    .help("The droplets to get information about")
                    .required(true)
                    .multiple(true)
                    .takes_value(true),
            )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        let droplets = arg_matches
            .values_of("droplet")
            .unwrap()
            .map(|v| v.parse::<usize>())
            .collect::<Result<Vec<_>, num::ParseIntError>>()?;

        let response = droplets
            .into_iter()
            .map(Droplet::get)
            .map(|req| client.execute(req))
            .collect::<Result<Vec<_>, Error>>()?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
