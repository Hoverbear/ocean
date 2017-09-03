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
            .about("Create an SSH key.")
            .arg(
                Arg::with_name("name")
                    .value_name("NAME")
                    .help("The name of the key.")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("pubkey")
                    .value_name("PUBLIC_KEY")
                    .help("The public key of this SSH key.")
                    .required(true)
                    .takes_value(true),
            )
        // TODO: Allow specifying a pubkey file?
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        // These are required arguments.
        let name = arg_matches.value_of("name").unwrap();
        let pubkey = arg_matches.value_of("pubkey").unwrap();

        let response = client.execute(SshKey::create(name, pubkey))?;

        Self::output(response, arg_matches.value_of("output"))?;

        Ok(())
    }
}
