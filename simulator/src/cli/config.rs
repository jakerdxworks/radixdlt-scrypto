use scrypto::rust::collections::*;
use std::fs;
use std::path::PathBuf;

use crate::cli::*;

/// Returns the data directory.
pub fn get_data_dir() -> Result<PathBuf, Error> {
    let mut path = dirs::home_dir().ok_or(Error::NoHomeFolder)?;
    path.push(".radix-engine-simulator");
    if !path.exists() {
        std::fs::create_dir_all(&path).map_err(|e| Error::IOError(e))?;
    }
    Ok(path)
}

/// Returns the config file.
pub fn get_config_json() -> Result<PathBuf, Error> {
    let mut path = get_data_dir()?;
    path.push("config");
    Ok(path.with_extension("json"))
}

pub fn get_configs() -> Result<HashMap<String, String>, Error> {
    let path = get_config_json()?;
    if path.exists() {
        Ok(
            serde_json::from_str(&fs::read_to_string(path).map_err(|e| Error::IOError(e))?)
                .map_err(|e| Error::JSONError(e))?,
        )
    } else {
        Ok(HashMap::new())
    }
}

pub fn set_configs(config: HashMap<String, String>) -> Result<(), Error> {
    let path = get_config_json()?;
    fs::write(
        path,
        serde_json::to_string_pretty(&config).map_err(|e| Error::JSONError(e))?,
    )
    .map_err(|e| Error::IOError(e))
}

pub fn get_config(key: &str) -> Result<Option<String>, Error> {
    Ok(get_configs()?.get(key).map(ToOwned::to_owned))
}

pub fn set_config(key: &str, value: &str) -> Result<(), Error> {
    let mut configs = get_configs()?;
    configs.insert(key.to_owned(), value.to_owned());
    set_configs(configs)
}
