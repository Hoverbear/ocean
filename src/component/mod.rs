use error::Result;
use clap::{App, Arg, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use serde_json;
use AsTable;
use toml;
use serde::Serialize;

mod droplet;
pub use self::droplet::Root as Droplet;
mod domain;
pub use self::domain::Root as Domain;

/// `Components`s are `clap::App`s. Just call them with `app` to get the Clap `App`/`Subcommand`
/// object. (Yes you can just use an `App`, cool huh?) then later use `handle()` to chain the args
/// in.
///
/// `Components`s are deliberately simple: They just take a `digitalocean::DigitalOcean` client and
/// the `clap::ArgMatches` when they handle the request. This means splitting one into its own app,
/// or integrating it with another `clap::App` is ridiculously simple. If you want tighter
/// integration please check out the `DigitalOcean` crate.
pub trait Component {
    /// Builds the `clap::App` that corresponds to this `Component`.
    fn app() -> App<'static, 'static>;
    /// Handle the request with some `ArgMatches`. This does one of two things:
    ///   1. It finalizes and terminates a request.
    ///   2. Invoke another `Component`. Eg. With `ocean droplet create` the
    ///      `component::droplet::Root` compnent which would call the `component::droplet::Create`
    ///      component.
    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()>;

    fn output<'a, T>(values: T, format: Option<&'a str>) -> Result<()>
    where T: Serialize + ::std::fmt::Debug + AsTable {
        match format {
            Some("debug") => println!("{:#?}", values),
            Some("json") => println!("{:#}", serde_json::to_value(&values)?),
            Some("toml") => println!("{}", toml::to_string(&values)?),
            Some("yaml") => unimplemented!(),
            Some("table") => values.as_table(),
            _ => unreachable!(),
        };
        Ok(())
    }
}

/// The root of the application.
pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new(env!("CARGO_PKG_NAME"))
            .version(crate_version!())
            .author(crate_authors!())
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .global_settings(
                &[
                    AppSettings::ColoredHelp,
                    AppSettings::GlobalVersion,
                    AppSettings::InferSubcommands,
                ],
            )
            .setting(AppSettings::SubcommandRequired)
            .arg(
                Arg::with_name("token")
                    .long("token")
                    .short("t")
                    .value_name("TOKEN")
                    .help("The DigitalOcean API key to use.")
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .help("Output in a given format.")
                    .takes_value(true)
                    .possible_values(&["json", "yaml", "toml", "table", "debug"])
                    .default_value("table")
                    .required(false)
                    .global(true)
            )
            .subcommand(Droplet::app())
            .subcommand(Domain::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        match arg_matches.subcommand() {
            ("droplet", Some(arg_matches)) => Droplet::handle(client, arg_matches),
            ("domain", Some(arg_matches)) => Domain::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }
    }
}
