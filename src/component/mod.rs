use crate::AsTable;
use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;
use serde;
use serde_json;
use serde_yaml;
use toml;

mod account;
pub use self::account::Root as Account;
mod droplet;
pub use self::droplet::Root as Droplet;
mod domain;
pub use self::domain::Root as Domain;
mod ssh_key;
pub use self::ssh_key::Root as SshKey;
mod region;
pub use self::region::Root as Region;
mod infrastructure;
pub use self::infrastructure::Root as Infrastructure;

/// `Component`s are `clap::App`s. Just call them with `app` to get the Clap `App`/`Subcommand`
/// object. (Yes you can just use an `App`, cool huh?) then later use `handle()` to chain the args
/// in.
///
/// `Component`s are deliberately simple: They just take a `digitalocean::DigitalOcean` client and
/// the `clap::ArgMatches` when they handle the request. This means splitting one into its own app,
/// or integrating it with another `clap::App` is ridiculously simple. If you want tighter
/// integration please check out the `DigitalOcean` crate.
pub trait Component {
    /// The default output style of the `Component`. For non-outputting parts of the component
    /// chain this is `None`.
    const DEFAULT_OUTPUT: Option<&'static str> = None;

    /// Builds the `clap::App` that corresponds to this `Component`.

    fn app() -> App<'static, 'static>;
    /// Handle the request with some `ArgMatches`. This does one of two things:
    ///   1. It finalizes and terminates a request.
    ///   2. Invoke another `Component`. Eg. With `ocean droplet create` the
    ///      `component::droplet::Root` compnent which would call the `component::droplet::Create`
    ///      component.
    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error>;

    /// Handles outputting the values appropriately.
    fn output<'a, T>(values: T, format: Option<&'a str>) -> Result<(), Error>
    where
        T: serde::ser::Serialize + ::std::fmt::Debug + AsTable,
    {
        match format.or(Self::DEFAULT_OUTPUT) {
            Some("debug") => println!("{:#?}", values),
            Some("json") => println!("{}", serde_json::to_string_pretty(&values)?),
            Some("toml") => {
                // This is a slightly nasty workaround for
                // https://github.com/alexcrichton/toml-rs/issues/142
                let workaround = toml::value::Value::try_from(&values)?;
                println!("{}", toml::to_string_pretty(&workaround)?)
            }
            Some("yaml") => println!("{}", serde_yaml::to_string(&values)?),
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
            .global_settings(&[
                AppSettings::ColoredHelp,
                AppSettings::GlobalVersion,
                AppSettings::InferSubcommands,
            ])
            .setting(AppSettings::SubcommandRequired)
            .arg(
                Arg::with_name("token")
                    .long("token")
                    .short("t")
                    .value_name("TOKEN")
                    .help("The DigitalOcean API key to use")
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .help("Output in a given format")
                    .takes_value(true)
                    .possible_values(&["json", "yaml", "toml", "table", "debug"])
                    .required(false)
                    .global(true),
            )
            .subcommand(Account::app())
            .subcommand(Droplet::app())
            .subcommand(Domain::app())
            .subcommand(Infrastructure::app())
            .subcommand(SshKey::app())
            .subcommand(Region::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        match arg_matches.subcommand() {
            ("account", Some(arg_matches)) => Account::handle(client, arg_matches),
            ("droplet", Some(arg_matches)) => Droplet::handle(client, arg_matches),
            ("domain", Some(arg_matches)) => Domain::handle(client, arg_matches),
            ("key", Some(arg_matches)) => SshKey::handle(client, arg_matches),
            ("infrastructure", Some(arg_matches)) => Infrastructure::handle(client, arg_matches),
            ("region", Some(arg_matches)) => Region::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }
    }
}
