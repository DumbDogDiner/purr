use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;
use std::{fs, path::Path, sync::RwLock, io::Write};

static CONFIG_PATH: &'static str = "$HOME/.config/purr/config.toml";

lazy_static! {
    static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings::new().unwrap());
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub api_endpoint: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::from_str(include_str!("../assets/config.toml"), FileFormat::Toml))?;
        s.merge(File::with_name(CONFIG_PATH).required(false))?;

        // write config
        // match Path::new(CONFIG_PATH).exists() {
        //     false => { 
        //         let mut out = fs::File::create(CONFIG_PATH).expect("unable to write configuration file");
        //         out.write(include_str!("../assets/config.toml").as_bytes()).expect("failed to write config file");
        //     }
        //     _ => ()
        // }

        s.try_into()
    }
}

pub fn get_config() -> Settings {
    SETTINGS.read().unwrap().clone()
}
