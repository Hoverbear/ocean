use clap::{_clap_count_exprs, arg_enum};

arg_enum! {
    #[derive(Debug)]
    enum Output {
        Json,
        Yaml,
        Toml,
        Table,
        Debug
    }
}
