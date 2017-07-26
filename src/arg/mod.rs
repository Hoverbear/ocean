use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};

pub fn limit() -> Arg<'static, 'static> {
    Arg::with_name("limit")
        .long("limit")
        .short("l")
        .value_name("LIMIT")
        .help("The maximum number to retrieve.")
        .required(false)
}

pub fn id() -> Arg<'static, 'static> {
    Arg::with_name("id")
        .value_name("ID")
        .help("The ID to retrieve.")
        .required(true)
}
