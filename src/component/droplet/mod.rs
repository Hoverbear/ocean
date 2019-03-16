use crate::component::Component;
use crate::AsTable;
use clap::{App, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;
use prettytable::Cell;
use prettytable::Row;
use prettytable::{self, Table};

mod list;
pub use self::list::List;

mod get;
pub use self::get::Get;

mod create;
pub use self::create::Create;

pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new("droplet")
            .about("Interact with droplets")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(List::app())
            .subcommand(Get::app())
            .subcommand(Create::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        match arg_matches.subcommand() {
            ("list", Some(arg_matches)) => List::handle(client, arg_matches),
            ("get", Some(arg_matches)) => Get::handle(client, arg_matches),
            ("create", Some(arg_matches)) => Create::handle(client, arg_matches),
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
        table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
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
