use clap::Arg;

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

pub fn output() -> Arg<'static, 'static> {
    Arg::with_name("output")
        .long("output")
        .short("o")
        .help("Output in a given format.")
        .takes_value(true)
        .possible_values(&["json", "yaml", "toml"])
        .required(false)
}
