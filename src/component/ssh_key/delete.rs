use crate::component::Component;
use clap::{App, Arg, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;

pub struct Delete;

impl Component for Delete {
    const DEFAULT_OUTPUT: Option<&'static str> = Some("debug");

    fn app() -> App<'static, 'static> {
        App::new("delete").about("Delete an SSH key").arg(
            Arg::with_name("key")
                .value_name("KEY")
                .help("The SSH key names or fingerprints to be deleted")
                .required(true)
                .multiple(true)
                .takes_value(true),
        )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        let values = arg_matches.values_of("key").unwrap();

        let responses = values
            .map(|v| (v, SshKey::delete(v)))
            .map(|(name, req)| client.execute(req).map(|()| name))
            .collect::<Result<Vec<_>, Error>>()?;

        Self::output(responses, arg_matches.value_of("output"))?;

        Ok(())
    }
}
