use clap::{App, Arg, ArgMatches};
use component::Component;
use digitalocean::prelude::*;
use error::{Result, ResultExt};

pub struct Delete;

impl Component for Delete {
    fn app() -> App<'static, 'static> {
        App::new("delete").about("Delete an SSH key.").arg(
            Arg::with_name("key")
                .value_name("KEY")
                .help("The SSH key names or fingerprints to be deleted.")
                .required(true)
                .multiple(true)
                .takes_value(true),
        )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        let values = arg_matches.values_of("key").unwrap();

        let responses = values
            .map(|v| (v, SshKey::delete(v)))
            .map(|(name, req)| {
                client.execute(req)
                    .map(|()| name)
                    .chain_err(|| format!("Failed to delete {}.", name))
            })
            .collect::<Result<Vec<_>>>()?;

        Self::output(responses, arg_matches.value_of("output"))?;

        Ok(())
    }
}
