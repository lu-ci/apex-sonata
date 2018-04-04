use std::fs::File;
use std::path::Path;
use std::process::exit;
use serde_yaml;
use std::io::Result;


#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SonataConfiguration {
    pub sigma: String
}

impl SonataConfiguration {
    pub fn new() -> Result<Self> {
        let file = File::open("cfg.yml")?;
        let snt_cfg: SonataConfiguration = match serde_yaml::from_reader(&file) {
            Ok(snt_cfg) => snt_cfg,
            Err(_) => {
                println!("Error: No location pointer.");
                exit(1);
            }
        };
        Ok(snt_cfg)
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DiscordConfiguration {
    pub token: String,
    pub bot: bool
}

impl DiscordConfiguration {
    fn new(cfgfile: &Path) -> Result<Self> {
        let file = File::open(cfgfile)?;
        let dsc_cfg: DiscordConfiguration = match serde_yaml::from_reader(&file) {
            Ok(dsc_cfg) => dsc_cfg,
            Err(_) => {
                println!("Error: No discord config.");
                exit(1);
            }
        };
        Ok(dsc_cfg)
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
    fn new(cfgfile: &Path) -> Result<Self> {
        let file = File::open(cfgfile)?;
        let db_cfg: DatabaseConfiguration = match serde_yaml::from_reader(&file) {
            Ok(db_cfg) => db_cfg,
            Err(_) => {
                println!("Error: No database config.");
                exit(1);
            }
        };
        Ok(db_cfg)
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
        fn new(cfgfile: &Path) -> Result<Self> {
        let file = File::open(cfgfile)?;
        let pref_cfg: PreferencesConfiguration = match serde_yaml::from_reader(&file) {
            Ok(pref_cfg) => pref_cfg,
            Err(_) => {
                println!("Error: No preferences config.");
                exit(1);
            }
        };
        Ok(pref_cfg)
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
    pub fn new(location: String) -> Result<Self> {
        let db_loc = format!("{}/config/core/database.yml", location);
        let dsc_loc = format!("{}/config/core/discord.yml", location);
        let pref_loc = format!("{}/config/core/preferences.yml", location);
        let db_path = Path::new(&db_loc);
        let dsc_path = Path::new(&dsc_loc);
        let pref_path = Path::new(&pref_loc);
        Configuration::check_config_files(db_path, dsc_path, pref_path);
        let cfg = Configuration {
            dsc: DiscordConfiguration::new(dsc_path)?,
            db: DatabaseConfiguration::new(db_path)?,
            pref: PreferencesConfiguration::new(pref_path)?
        };
        Ok(cfg)
    }
}
