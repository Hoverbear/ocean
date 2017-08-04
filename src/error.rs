use digitalocean;
use std::net::AddrParseError;
use serde_json;
use toml;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    links {
        DigitalOcean(digitalocean::Error, digitalocean::ErrorKind);
    }

    foreign_links {
        Addr(AddrParseError);
        SerdeJson(serde_json::Error);
        TomlSerialize(toml::ser::Error);
    }
}
