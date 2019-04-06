use crate::component::Component;
use clap::{App, Arg, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;

pub struct Get;

impl Component for Get {
    const DEFAULT_OUTPUT: Option<&'static str> = Some("debug");

    fn app() -> App<'static, 'static> {
        App::new("get")
            .about("Get detailed info about certificates")
            .arg(
                Arg::with_name("certificate")
                    .value_name("CERTIFICATE")
                    .help("The certificates to get information about")
                    .required(true)
                    .multiple(true)
                    .takes_value(true),
            )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        let args = arg_matches.values_of("certificate").unwrap();

        let response = args
            .map(|certificate| Certificate::get(certificate))
            .map(|req| client.execute(req))
            .collect::<Result<Vec<_>, Error>>()?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
