use regex::bytes::Regex;
use serde::Deserialize;
use std::{fs, io};

#[derive(Debug)]
pub enum LoadConfigError {
    CouldNotReadConfig(io::Error),
    CouldNotParseConfig(serde_json::Error),
    InvalidRegex(regex::Error),
}

impl From<io::Error> for LoadConfigError {
    fn from(error: io::Error) -> Self {
        LoadConfigError::CouldNotReadConfig(error)
    }
}

impl From<serde_json::Error> for LoadConfigError {
    fn from(error: serde_json::Error) -> Self {
        LoadConfigError::CouldNotParseConfig(error)
    }
}

impl From<regex::Error> for LoadConfigError {
    fn from(error: regex::Error) -> Self {
        LoadConfigError::InvalidRegex(error)
    }
}

#[derive(Deserialize, Debug)]
struct ConfigJson {
    flag_re: String,
    addresses: Vec<String>,
    flagbot_address: String,
    interval: usize,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub flag_re: Regex,
    pub addresses: Vec<String>,
    pub flagbot_address: String,
    pub interval: usize,
}

pub fn load_config(file_path: Option<&str>) -> Result<Config, LoadConfigError> {
    let path = file_path.unwrap_or("/bambiXploit.conf");
    let cfg_raw: ConfigJson = serde_json::from_str(&fs::read_to_string(path)?)?;
    let flag_re = Regex::new(&cfg_raw.flag_re)?;
    Ok(Config {
        flag_re,
        addresses: cfg_raw.addresses,
        flagbot_address: cfg_raw.flagbot_address,
        interval: cfg_raw.interval,
    })
}
