use clap::{App, Arg, ArgMatches, Values};
use error::{Result, ResultExt};
use digitalocean::prelude::*;
use component::Component;
use serde_json;
use toml;
use PrintTable;
use arg;
use serde::Serialize;

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
        ).arg(arg::output())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        let args = arg_matches.values_of("domain").unwrap();

        let entries = args
            .map(|domain| Domain::get(domain))
            .map(|req| client.execute(req)
                .chain_err(|| "Failed to make API request."))
            .collect::<Result<Vec<_>>>()?;

        output(entries, arg_matches.value_of("output"))?;


        Ok(())
    }
}

fn output<'a, T>(values: T, format: Option<&'a str>) -> Result<()>
where T: Serialize + ::std::fmt::Debug {
    match format {
        None => println!("{:?}", values),
        Some("json") => println!("{:#}", serde_json::to_value(&values)?),
        Some("toml") => println!("{}", toml::to_string(&values)?),
        Some("yaml") => unimplemented!(),
        _ => unreachable!(),
    };
    Ok(())
}
