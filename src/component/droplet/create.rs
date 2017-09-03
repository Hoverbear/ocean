use clap::{App, Arg, ArgMatches};
use component::Component;
use digitalocean::prelude::*;
use error::Result;
use std::net::IpAddr;
use std::str::FromStr;

pub struct Create;

impl Component for Create {
    fn app() -> App<'static, 'static> {
        App::new("create")
            .about("Create a droplet.")
            .arg(
                Arg::with_name("name")
                    .value_name("NAME")
                    .help("The name of the droplet.")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("region")
                    .value_name("REGION")
                    .help("The region to place the droplet in.")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("size")
                    .value_name("SIZE")
                    .help("The size of the droplet.")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("image")
                    .value_name("IMAGE")
                    .help("The image for the droplet.")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("key")
                    .long("key")
                    .value_name("KEY")
                    .help("The key IDs that should have access to the droplet.")
                    .multiple(true)
                    .required(false)
                    .takes_value(true),
            )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        // These are required arguments.
        let name = arg_matches.value_of("name").unwrap();
        let region = arg_matches.value_of("region").unwrap();
        let size = arg_matches.value_of("size").unwrap();
        let image = arg_matches.value_of("image").unwrap();


        let request = Droplet::create(name, region, size, image);

        // These are optional arguments.
        let request = match arg_matches.values_of("key") {
            Some(values) => request.ssh_keys(values.collect()),
            None => request,
        };

        let response = client.execute(request)?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
