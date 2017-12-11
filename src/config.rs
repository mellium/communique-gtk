extern crate toml;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Read;
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

/// An error type that encapsulates all errors that can be returned while loading and parsing
/// configuration.
#[derive(Debug)]
pub enum Error {
    De(toml::de::Error),
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::De(err)
    }
}

pub fn load_config() -> Result<Config, Error> {
    // Uses XDG_CONFIG_HOME, $HOME/.config, or "./" as the config search dir in that order
    let configdir = match env::var_os("XDG_CONFIG_HOME") {
        Some(s) => s,
        None => {
            match env::home_dir() {
                Some(s) => {
                    let mut path = s;
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
    fs::File::open(path)?.read_to_string(&mut s)?;

    let cfg: Config = toml::from_str(&s)?;
    Ok(cfg)
}
