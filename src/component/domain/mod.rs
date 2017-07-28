use clap::{App, Arg, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use {arg, PrintTable};
use component::Component;
use prettytable::{self, Table};
use prettytable::row::Row;
use prettytable::cell::Cell;

mod list;
pub use self::list::List;

mod create;
pub use self::create::Create;

mod delete;
pub use self::delete::Delete;

pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new("domain")
            .about("Interact with domains.")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(List::app())
            .subcommand(Create::app())
            .subcommand(Delete::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) {
        match arg_matches.subcommand() {
            ("list", Some(arg_matches)) => List::handle(client, arg_matches),
            ("create", Some(arg_matches)) => Create::handle(client, arg_matches),
            ("delete", Some(arg_matches)) => Delete::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }
    }
}

impl PrintTable for Domain {
    fn print_table(&self) {
        [self.clone()].print_table()
    }
}

impl PrintTable for [Domain] {
    fn print_table(&self) {
        let mut table = Table::new();

        table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(Row::new(vec![
                               Cell::new("name"),
                               Cell::new("ttl"),
        ]));

        for row in self {
            table.add_row(Row::new(vec![
                               Cell::new(row.name()),
                               Cell::new(&row.ttl()
                                             .map(|v| v.to_string())
                                             .unwrap_or(String::from("-"))),
            ]));
        }

        table.printstd();
    }
}
