use digitalocean::prelude::*;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Infrastructure {
    domains: Vec<Domain>,
    droplets: Vec<Droplet>,
    sshkeys: Vec<SshKey>,
    // ...
}

impl Infrastructure {
    /// Handles outputting the values appropriately.
    fn write<'a, T>(&self, output: &mut Write, format: Option<&'a str>) -> Result<()>
    where
        T: serde::ser::Serialize + ::std::fmt::Debug + AsTable,
    {
        match format {
            Some("debug") => println!("{:#?}", self),
            Some("json") => println!("{:#}", serde_json::to_value(self)?),
            Some("toml") => println!("{}", toml::to_string(self)?),
            Some("yaml") => println!("{}", serde_yaml::to_string(self)?),
            Some("table") => values.as_table(),
            _ => unreachable!(),
        };
        Ok(())
    }
}
