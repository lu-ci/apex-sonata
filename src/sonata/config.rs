extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::fs::File;
use std::io::prelude;

fn getYamlContent(location: str) -> str {
    let mut yaml_file = File::open(location)?;
    let mut yaml_cont = String::new();
    yaml_file.read_to_string(&mut yaml_cont)?
}

struct SonataConfig {
    sigma_location: str
}

impl SonataConfig {
    pub fn new(location: str) -> SonataConfig {
        SonataConfig {sigma_location: location}
    }
}

struct DiscordConfig {
    token: str,
    owners: [i64],
    bot: bool
}

impl DiscordConfig {
    pub fn load(snt: SonataConfig) -> DiscordConfig {
        let config_location = snt::sigma_location + "/config/core/discord.yml";
        let config_yaml = getYamlContent(config_location);
        let configuration = YamlLoader::load_from_str(config_yaml)::unwrap();
    }
}
struct DatabaseConfig {
    database: str,
    auth: bool,
    host: str,
    port: i32,
    username: str,
    password: str
}

struct PreferencesConfig {
    dev_mode: bool,
    status_rotation: bool,
    prefix: str,
    currency: str,
    currency_icon: str,
    website: str,
    text_only: str,
    music_only: str,
    dscbots_token: str,
    movelog_channel: i64
}

struct Configuration {
    dsc: DiscordConfig,
    db: DatabaseConfig,
    pref: PreferencesConfig
}