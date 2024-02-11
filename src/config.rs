use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::path::PathBuf;
use std::{env, fs};
use toml::Table;

// TODO: make config fields better

#[derive(Debug, Deserialize)]
pub struct EditorConfig {
    #[allow(dead_code)]
    pub insert: Insert,
    pub additional_quit_amount: u8,
}
#[derive(Debug, Deserialize)]
pub struct Insert {
    pub autopairs: bool,
    pub expand_tab: bool,
    pub space_expansion: Option<u8>,
}

impl EditorConfig {
    // default config
    pub fn default() -> Self {
        EditorConfig {
            insert: Insert {
                autopairs: true,
                expand_tab: true,
                space_expansion: Some(2),
            },
            additional_quit_amount: 3,
        }
    }

    pub fn new(insert: Insert, quit_amount: u8) -> Self {
        EditorConfig {
            insert,
            additional_quit_amount: quit_amount,
        }
    }
}

// singleton for configuration, all operations read from this
pub struct ConfigManager {
    config: EditorConfig,
}

impl ConfigManager {
    // default values
    pub fn default() -> Self {
        let user_home = match dirs::home_dir() {
            Some(path) => path,
            None => {
                // FIXME: do we exit or do we just give the default config?
                eprintln!("Failed to get user home directory");
                std::process::exit(1); // TODO: more uniform errors
            }
        };

        let config_path = user_home.join(".config/pico.toml");

        let config = if let Ok(config_contents) = fs::read_to_string(config_path) {
            if let Ok(config) = toml::from_str::<EditorConfig>(&config_contents) {
                config
            } else {
                eprintln!("Failed to deserialize config. Using default config.");
                EditorConfig::default()
            }
        } else {
            eprintln!("Failed to read the config file. Using default config.");
            EditorConfig::default()
        };

        ConfigManager { config }
    }

    pub fn get_config(&self) -> &EditorConfig {
        &self.config
    }

    // access editor config (mutable)
    pub fn get_mut_config(&mut self) -> &mut EditorConfig {
        &mut self.config
    }
}

// lazy_static because it is initiated at runtime
lazy_static::lazy_static! {
        pub static ref CONFIG_MANAGER: ConfigManager = ConfigManager::default();
}

// we dont support a cli flag for configs, so pretend it doesnt exist rn
