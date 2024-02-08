use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use toml::Table;

// FIXME: find a better way to access these globally
#[derive(Debug, Deserialize)]
pub struct Config {
    #[allow(dead_code)]
    pub insert: Insert,
    pub quit_amount: u8,
}
#[derive(Debug, Deserialize)]
pub struct Insert {
    pub autopairs: bool, // TODO: implement this
    pub expand_tab: bool,
    pub space_expansion: Option<u8>,
}

// we dont support a cli flag for configs, so pretend it doesnt exist rn
// TODO: refactor this entire thing and provide a default config
fn get_config_file() -> Result<PathBuf> {
    let config = std::env::var("XDG_CONFIG_HOME").context("Unable to get $XDG_CONFIG_HOME")?;

    let mut config = PathBuf::from(config);

    config.push("pico.toml");

    return Ok(config);
}

pub fn get_config(config_file: Option<PathBuf>) -> Config {
    let default_path = PathBuf::from("wowow");
    let config_file = get_config_file().unwrap_or(default_path); // FIXME: add default config for
                                                                 // default file
    let toml_str = fs::read_to_string(config_file).expect("Failed to read pico.toml config");

    let pico_toml: Config = toml::from_str(&toml_str).expect("Failed to deserialize pico.toml");

    return pico_toml;
}
