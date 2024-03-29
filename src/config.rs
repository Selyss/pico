use serde::Deserialize;
use std::{fs, path::Path};

// TODO: add more fields for ui
#[derive(Debug, Deserialize)]
pub struct EditorConfig {
    #[allow(dead_code)]
    pub insert: Insert,
    pub additional_quit_amount: u8,
    pub colors: Colors,
}

// TODO: add cursor style options, its in termion

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub status_fg_color: Option<[u8; 3]>,
    pub status_bg_color: Option<[u8; 3]>,
}

#[derive(Debug, Deserialize)]
pub struct Insert {
    pub autopairs: bool,
    pub expand_tab: bool,
    pub space_expansion: Option<u8>,
    pub cursor_style: Option<String>,
}

impl EditorConfig {
    // default config
    pub fn default() -> Self {
        EditorConfig {
            insert: Insert {
                cursor_style: Some(String::from("steady_block")),
                autopairs: true,
                expand_tab: true,
                space_expansion: Some(2),
            },
            additional_quit_amount: 3,
            colors: Colors {
                status_fg_color: Some([63, 63, 63]),
                status_bg_color: Some([239, 239, 239]),
            },
        }
    }

    pub fn new(insert: Insert, colors: Colors, quit_amount: u8) -> Self {
        EditorConfig {
            insert,
            colors,
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

        let path = Path::new(&user_home);
        let config_path = path.join(".config/pico.toml").display().to_string();

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
