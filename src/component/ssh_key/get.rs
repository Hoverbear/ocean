use clap::{App, Arg, ArgMatches};
use component::Component;
use digitalocean::prelude::*;
use error::{Result, ResultExt};

pub struct Get;

impl Component for Get {
    fn app() -> App<'static, 'static> {
        App::new("get")
            .about("Get detailed info about SSH keys.")
            .arg(
                Arg::with_name("name")
                    .value_name("NAME")
                    .help("The SSH keys to get information about.")
                    .required(true)
                    .multiple(true)
                    .takes_value(true),
            )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        let args = arg_matches.values_of("name").unwrap();

        let response = args.map(|name| SshKey::get(name))
            .map(|req| {
                client.execute(req).chain_err(
                    || "Failed to make API request.",
                )
            })
            .collect::<Result<Vec<_>>>()?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}