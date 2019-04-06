use clap::Arg;

pub fn limit() -> Arg<'static, 'static> {
    Arg::with_name("limit")
        .long("limit")
        .short("l")
        .value_name("LIMIT")
        .help("The maximum number to retrieve")
        .required(false)
}
