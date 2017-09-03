use clap::{App, Arg, ArgMatches};
use component::Component;
use digitalocean::prelude::*;
use error::{Result, ResultExt};

pub struct Get;

impl Component for Get {
    fn app() -> App<'static, 'static> {
        App::new("get")
            .about("Get detailed info about droplets.")
            .arg(
                Arg::with_name("droplet")
                    .value_name("DROPLET")
                    .help("The droplets to get information about.")
                    .required(true)
                    .multiple(true)
                    .takes_value(true),
            )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        let droplets = arg_matches
            .values_of("droplet")
            .unwrap()
            .map(|v| {
                v.parse::<usize>().chain_err(|| {
                    format!("failed to parse {} to usize.", v)
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let response = droplets
            .into_iter()
            .map(|v| Droplet::get(v))
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
