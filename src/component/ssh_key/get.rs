use crate::component::Component;
use clap::{App, Arg, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;

pub struct Get;

impl Component for Get {
    fn app() -> App<'static, 'static> {
        App::new("get")
            .about("Get detailed info about SSH keys")
            .arg(
                Arg::with_name("name")
                    .value_name("NAME")
                    .help("The SSH keys to get information about")
                    .required(true)
                    .multiple(true)
                    .takes_value(true),
            )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        let args = arg_matches.values_of("name").unwrap();

        let response = args
            .map(SshKey::get)
            .map(|req| client.execute(req))
            .collect::<Result<Vec<_>, Error>>()?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
