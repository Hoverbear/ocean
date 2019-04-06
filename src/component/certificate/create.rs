use crate::component::Component;
use clap::{App, Arg, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;
use std::{fs::File, io::Read};

pub struct Create;

impl Component for Create {
    const DEFAULT_OUTPUT: Option<&'static str> = Some("debug");

    fn app() -> App<'static, 'static> {
        App::new("create")
            .about("Create a certificate")
            .arg(
                Arg::with_name("name")
                    .value_name("NAME")
                    .help("The name")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("leaf_certificate")
                    .value_name("CERTIFICATE")
                    .help("The leaf certificate file")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("private_key")
                    .value_name("PRIVATE_KEY")
                    .help("The private key file suitable for the certificate")
                    .required(true)
                    .takes_value(true),
            )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        // These are required arguments.
        let name = arg_matches.value_of("name").unwrap();
        let mut private_key = String::new();
        File::open(arg_matches.value_of("private_key").unwrap())?
            .read_to_string(&mut private_key)?;
        let mut leaf_certificate = String::new();
        File::open(arg_matches.value_of("leaf_certificate").unwrap())?
            .read_to_string(&mut leaf_certificate)?;

        let response =
            client.execute(Certificate::create(name, &private_key, &leaf_certificate))?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
