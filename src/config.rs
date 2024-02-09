use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use toml::Table;

// TODO: make config fields better

#[derive(Debug, Deserialize)]
pub struct EditorConfig {
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

impl EditorConfig {
    pub fn new(insert: Insert, quit_amount: u8) -> Self {
        EditorConfig {
            insert,
            quit_amount,
        }
    }
}

// singleton for configuration, all operations read from this
pub struct ConfigManager {
    config: EditorConfig,
}

impl ConfigManager {
    // default values
    pub fn initialize_default() -> Self {
        ConfigManager {
            config: EditorConfig::new(
                Insert {
                    autopairs: true,
                    expand_tab: true,
                    space_expansion: Some(4),
                },
                2, // quit amount
            ),
        }
    }

    pub fn get_config(&self) -> &EditorConfig {
        return &self.config;
    }

    // access editor config (mutable)
    pub fn get_mut_config(&mut self) -> &mut EditorConfig {
        return &mut self.config;
    }
}

// lazy_static because it is initiated at runtime
lazy_static::lazy_static! {
        pub static ref CONFIG_MANAGER: ConfigManager = ConfigManager::initialize_default();
}

// we dont support a cli flag for configs, so pretend it doesnt exist rn
// TODO: refactor this entire thing and provide a default config
fn get_config_file() -> Result<PathBuf> {
    let config = std::env::var("XDG_CONFIG_HOME").context("Unable to get $XDG_CONFIG_HOME")?;

    let mut config = PathBuf::from(config);

    config.push("pico.toml");

    return Ok(config);
}

pub fn get_config(config_file: Option<PathBuf>) -> EditorConfig {
    let default_path = PathBuf::from("wowow");
    let config_file = get_config_file().unwrap_or(default_path); // FIXME: add default config for
                                                                 // default file
    let toml_str = fs::read_to_string(config_file).expect("Failed to read pico.toml config");

    let pico_toml: EditorConfig =
        toml::from_str(&toml_str).expect("Failed to deserialize pico.toml");

    return pico_toml;
}
