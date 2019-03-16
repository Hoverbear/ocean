use crate::{component::Component, AsTable};
use clap::{App, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;
use prettytable::Cell;
use prettytable::Row;
use prettytable::{self, Table};

mod list;
pub use self::list::List;

pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new("region")
            .about("Interact with regions")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(List::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        match arg_matches.subcommand() {
            ("list", Some(arg_matches)) => List::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }
    }
}

impl AsTable for Region {
    fn as_table(&self) {
        vec![self.clone()].as_table()
    }
}

impl AsTable for Vec<Region> {
    fn as_table(&self) {
        let mut table = Table::new();

        table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(Row::new(vec![
            Cell::new("id"),
            Cell::new("slug"),
            Cell::new("sizes"),
            Cell::new("available"),
            Cell::new("features"),
        ]));

        for row in self {
            table.add_row(Row::new(vec![
                Cell::new(row.name()),
                Cell::new(row.slug()),
                Cell::new(&format!("{:?} sizes", row.sizes().len())),
                Cell::new(&format!("{}", row.available())),
                Cell::new(&format!("{:?}", row.features())),
            ]));
        }

        table.printstd();
    }
}
