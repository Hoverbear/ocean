use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use component::Component;

/// The root of the application.
pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new(env!("CARGO_PKG_NAME"))
            .version(crate_version!())
            .author(crate_authors!())
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .global_settings(&[
                             AppSettings::ColoredHelp,
                             AppSettings::GlobalVersion,
                             AppSettings::InferSubcommands,
            ])
            .setting(AppSettings::SubcommandRequired)
            .arg(Arg::with_name("token")
                 .long("token")
                 .short("t")
                 .value_name("TOKEN")
                 .help("The DigitalOcean API key to use.")
                 .required(false)
                 .takes_value(true))
            .subcommand(super::Droplet::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) {
        match arg_matches.subcommand() {
            ("droplet", Some(arg_matches)) =>
                super::Droplet::handle(client, arg_matches),
//            ("domain", Some(arg_matches)) =>
//                Component::domain::Root::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }

    }
}
