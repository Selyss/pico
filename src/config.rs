use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use toml::Table;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[allow(dead_code)]
    insert: Insert,
}
#[derive(Debug, Deserialize)]
struct Insert {
    autopairs: bool, // TODO:
    expand_tab: bool,
    space_expansion: Option<u8>,
}

// we dont support a cli flag for configs, so pretend it doesnt exist rn

fn get_config_file() -> Result<PathBuf> {
    let config = std::env::var("XDG_CONFIG_HOME").context("Unable to get $XDG_CONFIG_HOME")?;

    let mut config = PathBuf::from(config);

    config.push("pico.toml");

    return Ok(config);
}

pub fn get_config(config_file: Option<PathBuf>) {
    let default_path = PathBuf::from("wowow");
    let config_file = get_config_file().unwrap_or(default_path); // FIXME: add default config for
                                                                 // default file
    let toml_str = fs::read_to_string(config_file).expect("Failed to read pico.toml config");

    let pico_toml: Config = toml::from_str(&toml_str).expect("Failed to deserialize pico.toml");

    println!("{:#?}", pico_toml);
}
