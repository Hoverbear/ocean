use clap::{App, Arg, ArgMatches};
use digitalocean::prelude::*;
use error::Result;
use PrintTable;
use component::Component;
use std::net::IpAddr;
use std::str::FromStr;

pub struct Create;

impl Component for Create {
    fn app() -> App<'static, 'static> {
        App::new("create")
            .about("Create a domain.")
            .arg(
                Arg::with_name("domain")
                    .value_name("DOMAIN")
                    .help("The domain.")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("address")
                    .value_name("ADDRESS")
                    .help("The IP address to point the domain to.")
                    .required(true)
                    .takes_value(true),
            )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        // These are required arguments.
        let domain = arg_matches.value_of("domain").unwrap();
        let address = IpAddr::from_str(arg_matches.value_of("address").unwrap())?;

        let output = client.execute(Domain::create(domain, address))?;

        output.print_table();

        Ok(())
    }
}
