use clap::{App, Arg, ArgMatches};
use error::{Result, ResultExt};
use digitalocean::prelude::*;
use component::Component;

pub struct Get;

impl Component for Get {
    fn app() -> App<'static, 'static> {
        App::new("get").about("Get detailed info about domains.").arg(
            Arg::with_name("domain")
                .value_name("DOMAIN")
                .help("The domains to get information about.")
                .required(true)
                .multiple(true)
                .takes_value(true)
        )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        let args = arg_matches.values_of("domain").unwrap();

        let response = args
            .map(|domain| Domain::get(domain))
            .map(|req| client.execute(req)
                .chain_err(|| "Failed to make API request."))
            .collect::<Result<Vec<_>>>()?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}