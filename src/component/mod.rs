use error::Result;
use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};
use digitalocean::prelude::*;

mod droplet;
pub use self::droplet::Root as Droplet;
mod domain;
pub use self::domain::Root as Domain;
mod root;
pub use self::root::Root;

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
}
