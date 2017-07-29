use digitalocean;
use std::net::AddrParseError;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    links {
        DigitalOcean(digitalocean::Error, digitalocean::ErrorKind);
    }

    foreign_links {
        Addr(AddrParseError);
    }
}
