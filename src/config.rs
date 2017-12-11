extern crate toml;

use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io;
use std::io::Read;
use std::path;

use res;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub theme: Option<String>,

    #[serde(rename = "account", default)]
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Account {
    pub jid: String,
    pub password: Option<String>,
    pub enabled: Option<bool>,
}

/// An error type that encapsulates all errors that can be returned while loading and parsing
/// configuration.
#[derive(Debug)]
pub enum InnerError {
    De(toml::de::Error),
    Io(io::Error),
}

impl From<io::Error> for InnerError {
    fn from(err: io::Error) -> Self {
        InnerError::Io(err)
    }
}

impl From<toml::de::Error> for InnerError {
    fn from(err: toml::de::Error) -> Self {
        InnerError::De(err)
    }
}

impl fmt::Display for InnerError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &InnerError::De(ref err) => err.fmt(f),
            &InnerError::Io(ref err) => err.fmt(f),
        }
	}
}

/// An error type that will be returned by load_config that contains information about the state of
/// the application when the failure occured and contains the underlying error.
#[derive(Debug)]
pub struct Error {
    path: Option<path::PathBuf>,
    inner: InnerError,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error {
            path: None,
            inner: InnerError::Io(err),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error {
        path: None,
        inner: InnerError::De(err),
        }
    }
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.path {
            None => write!(f, "Error parsing config: `{:?}'", self.inner),
            Some(ref path) => write!(f, "Error loading config from `{}': `{}'", path.to_string_lossy(), self.inner),
        }
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
    let mut path = path::PathBuf::from(configdir);
    path.push(res::APP_NAME.to_lowercase());
    path.push("config.toml");

    let mut s = String::new();
    fs::File::open(&path).or_else(|e| Err(Error{
        path: Some(path),
        inner: e.into(),
    }))?.read_to_string(&mut s)?;

    let cfg: Config = toml::from_str(&s)?;
    Ok(cfg)
}
