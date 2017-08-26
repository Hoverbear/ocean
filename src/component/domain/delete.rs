use clap::{App, Arg, ArgMatches};
use component::Component;
use digitalocean::prelude::*;
use error::Result;

pub struct Delete;

impl Component for Delete {
    fn app() -> App<'static, 'static> {
        App::new("delete").about("Delete a domain.").arg(
            Arg::with_name("domain")
                .value_name("DOMAIN")
                .help("The domains to be deleted.")
                .required(true)
                .multiple(true)
                .takes_value(true),
        )
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        let domains = arg_matches.values_of("domain").unwrap();

        for domain in domains {
            client.execute(Domain::delete(domain))?;

            println!("{} deleted.", domain);
        }

        Ok(())
    }
}
