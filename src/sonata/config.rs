use std::fs::File;
use std::path::Path;
use std::process::exit;
use serde_yaml;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiscordConfiguration {
    token: String,
    pub bot: bool,
}

impl DiscordConfiguration {
    fn new(cfgfile: &Path) -> DiscordConfiguration {
        let file = File::open(cfgfile).expect("Failed for find the discord configuration file.");
        let dsc_cfg: DiscordConfiguration = serde_yaml::from_reader(&file).expect("Failed to read discord configuration file.");
        return dsc_cfg;
    }
    pub fn get_token(self) -> String {
        self.token
    }
}

pub struct Configuration {
    pub dsc: DiscordConfiguration,
}

impl Configuration {
    fn check_config_files(db_path: &Path, dsc_path: &Path, pref_path: &Path) {
        let mut missing = false;
        if !db_path.exists() {
            missing = true;
        } else if !dsc_path.exists() {
            missing = true;
        } else if !pref_path.exists() {
            missing = true
        }
        if missing {
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
        Configuration {dsc: DiscordConfiguration::new(dsc_path)}
    }
}
