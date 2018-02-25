use std::fs::File;
use std::path::Path;
use std::process::exit;
use serde_yaml;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DiscordConfiguration {
    pub token: String,
    pub bot: bool,
}

impl DiscordConfiguration {
    fn new(cfgfile: &Path) -> DiscordConfiguration {
        let file = File::open(cfgfile).expect("Failed for find the discord configuration file.");
        let dsc_cfg: DiscordConfiguration = serde_yaml::from_reader(&file).expect("Failed to read discord configuration file.");
        return dsc_cfg;
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
    pub database: String,
    pub auth: bool,
    pub host: String,
    pub port: i64,
    pub username: String,
    pub password: String
}

impl DatabaseConfiguration {
    fn new(cfgfile: &Path) -> DatabaseConfiguration {
        let file = File::open(cfgfile).expect("Failed for find the database configuration file.");
        let db_cfg: DatabaseConfiguration = serde_yaml::from_reader(&file).expect("Failed to read database configuration file.");
        return db_cfg;
    }
}
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PreferencesConfiguration {
    pub dev_mode: bool,
    pub status_rotation: bool,
    pub text_only: bool,
    pub music_only: bool,
    pub prefix: String,
    pub currency: String,
    pub currency_icon: String,
    pub website: String,
    pub movelog_channel: i64,
    pub key_to_my_heart: String
}

impl PreferencesConfiguration {
        fn new(cfgfile: &Path) -> PreferencesConfiguration {
        let file = File::open(cfgfile).expect("Failed for find the database configuration file.");
        let pref_cfg: PreferencesConfiguration = serde_yaml::from_reader(&file).expect("Failed to read database configuration file.");
        return pref_cfg;
    }
}

pub struct Configuration {
    pub db: DatabaseConfiguration,
    pub dsc: DiscordConfiguration,
    pub pref: PreferencesConfiguration
}

impl Configuration {
    fn check_config_files(db_path: &Path, dsc_path: &Path, pref_path: &Path) {
        if !(db_path.exists() && dsc_path.exists() && pref_path.exists()) {
            println!("Missing configuration file.");
            exit(1);
        }
    }
    pub fn new(location: String) -> Configuration {
        let db_loc = format!("{}/config/core/database.yml", location);
        let dsc_loc = format!("{}/config/core/discord.yml", location);
        let pref_loc = format!("{}/config/core/preferences.yml", location);
        let db_path = Path::new(&db_loc);
        let dsc_path = Path::new(&dsc_loc);
        let pref_path = Path::new(&pref_loc);
        Configuration::check_config_files(db_path, dsc_path, pref_path);
        Configuration {dsc: DiscordConfiguration::new(dsc_path), db: DatabaseConfiguration::new(db_path), pref: PreferencesConfiguration::new(pref_path)}
    }
}
