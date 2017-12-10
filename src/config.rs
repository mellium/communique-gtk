extern crate toml;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

use res;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub theme: Option<String>,

    #[serde(rename = "account", default)]
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub jid: String,
    pub password: Option<String>,
    pub enabled: Option<bool>,
}

pub fn load_config() -> Config {
    // Uses XDG_CONFIG_HOME, $HOME/.config, or "./" as the config search dir in that order
    let configdir = match env::var_os("XDG_CONFIG_HOME") {
        Some(s) => s,
        None => {
            match env::home_dir() {
                Some(s) => {
                    let mut path = PathBuf::from(s);
                    path.push(".config");

                    path.into_os_string()
                }
                None => OsStr::new("").to_os_string(),
            }
        }
    };
    let mut path = PathBuf::from(configdir);
    path.push(res::APP_NAME.to_lowercase());
    path.push("config.toml");

    let mut s = String::new();
    fs::File::open(path.into_os_string().to_str().unwrap())
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let parsed: Config = toml::from_str(&s).unwrap();
    parsed
}
