use crate::{component::Component, AsTable};
use clap::{App, AppSettings, ArgMatches};
use digitalocean::prelude::*;
use failure::Error;
use prettytable::Cell;
use prettytable::Row;
use prettytable::{self, Table};

mod get;
pub use self::get::Get;

pub struct Root;

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new("account")
            .about("Interact with your account")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(Get::app())
    }

    fn handle(client: DigitalOcean, arg_matches: &ArgMatches) -> Result<(), Error> {
        match arg_matches.subcommand() {
            ("get", Some(arg_matches)) => Get::handle(client, arg_matches),
            _ => panic!("Unknown subcommand provided"),
        }
    }
}

impl AsTable for Account {
    fn as_table(&self) {
        vec![self.clone()].as_table()
    }
}

impl AsTable for Vec<Account> {
    fn as_table(&self) {
        let mut table = Table::new();

        table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(Row::new(vec![
            Cell::new("email"),
            Cell::new("droplet limit"),
            Cell::new("floating ip limit"),
            Cell::new("email verified"),
            Cell::new("status"),
            Cell::new("message"),
        ]));

        for row in self {
            table.add_row(Row::new(vec![
                Cell::new(row.email()),
                Cell::new(&row.droplet_limit().to_string()),
                Cell::new(&row.floating_ip_limit().to_string()),
                Cell::new(&format!("{}", row.email_verified())),
                Cell::new(row.status()),
                Cell::new(row.status_message()),
            ]));
        }

        table.printstd();
    }
}
