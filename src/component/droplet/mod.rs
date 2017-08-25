use clap::{App, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use AsTable;
use component::Component;
use prettytable::{self, Table};
use prettytable::row::Row;
use prettytable::cell::Cell;
use error::Result;

mod list;
pub use self::list::List;

pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new("droplet")
            .about("Interact with droplets.")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(List::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<()> {
        match arg_matches.subcommand() {
            ("list", Some(arg_matches)) => List::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }
    }
}

impl AsTable for Droplet {
    fn as_table(&self) {
        vec![self.clone()].as_table()
    }
}

impl AsTable for Vec<Droplet> {
    fn as_table(&self) {
        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR,
        );
        table.set_titles(Row::new(vec![
            Cell::new("id"),
            Cell::new("name"),
            Cell::new("size"),
            Cell::new("region"),
            Cell::new("image"),
        ]));

        for item in self {
            table.add_row(Row::new(vec![
                Cell::new(&format!("{}", item.id())),
                Cell::new(item.name()),
                Cell::new(item.size_slug()),
                Cell::new(item.region().slug()),
                Cell::new(item.image().name()),
            ]));
        }
        table.printstd();
    }
}
