extern crate toml;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub theme: String,
    #[serde(rename = "account")]
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub jid: String,
    pub password: String,
    pub enabled: bool,
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
    path.push("yokel");
    path.push("yokelrc");

    let mut s = String::new();
    fs::File::open(path.into_os_string().to_str().unwrap())
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let parsed: Config = toml::decode(s.parse().unwrap()).unwrap();
    parsed
}
