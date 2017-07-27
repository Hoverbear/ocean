use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use arg;
use component::Component;

use prettytable::{self, Table};
use prettytable::row::Row;
use prettytable::cell::Cell;

pub struct List;

impl Component for List {
    fn app() -> App<'static, 'static> {
        App::new("list")
            .about("List domains.")
            .arg(arg::limit())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) {
        let list = client.execute(Domain::list()).unwrap();

        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(Row::new(vec![
                               Cell::new("name"),
                               Cell::new("ttl"),
        ]));

        for item in list {
            table.add_row(Row::new(vec![
                                   Cell::new(item.name()),
                                   Cell::new(&item.ttl().to_string()),
            ]));
        }
        table.printstd();
    }
}

