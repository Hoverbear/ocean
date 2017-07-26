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
            .about("List droplets.")
            .arg(arg::limit())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) {
        let list = client.execute(Droplet::list()).unwrap();

        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(Row::new(vec![
                               Cell::new("id"),
                               Cell::new("name"),
                               Cell::new("size"),
                               Cell::new("region"),
                               Cell::new("image"),
        ]));

        for item in list {
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

