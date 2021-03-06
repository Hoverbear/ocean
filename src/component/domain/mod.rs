use crate::{component::Component, AsTable};
use clap::{App, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;
use prettytable::Cell;
use prettytable::Row;
use prettytable::{self, Table};

mod list;
pub use self::list::List;

mod create;
pub use self::create::Create;

mod delete;
pub use self::delete::Delete;

mod get;
pub use self::get::Get;

pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new("domain")
            .about("Interact with domains")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(List::app())
            .subcommand(Create::app())
            .subcommand(Delete::app())
            .subcommand(Get::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        match arg_matches.subcommand() {
            ("list", Some(arg_matches)) => List::handle(client, arg_matches),
            ("create", Some(arg_matches)) => Create::handle(client, arg_matches),
            ("delete", Some(arg_matches)) => Delete::handle(client, arg_matches),
            ("get", Some(arg_matches)) => Get::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }
    }
}

impl AsTable for Domain {
    fn as_table(&self) {
        vec![self.clone()].as_table()
    }
}

impl AsTable for Vec<Domain> {
    fn as_table(&self) {
        let mut table = Table::new();

        table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(Row::new(vec![Cell::new("name"), Cell::new("ttl")]));

        for row in self {
            table.add_row(Row::new(vec![
                Cell::new(row.name()),
                Cell::new(
                    &row.ttl()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| String::from("-")),
                ),
            ]));
        }

        table.printstd();
    }
}
