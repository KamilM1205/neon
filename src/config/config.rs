use serde::{Deserialize, Serialize};

use crate::config::theme::Theme;

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct ConfigFile<'a> {
    theme: Option<&'a str>,
    plugin_use: Option<bool>,
}

#[derive(Copy, Clone)]
pub struct Config {
    theme: Theme,
}

impl Config {
    pub fn load(path: String) -> Self {
        use std::fs;
        use std::path;
        let path = path::PathBuf::from(path);
        let mut data = String::new();
        if path.exists() {
            data = fs::read_to_string(path).unwrap();
        } else {
            fs::create_dir_all(&path).unwrap();
            data = toml::to_string(&ConfigFile {
                theme: None,
                plugin_use: Some(false),
            }).unwrap();
            fs::write(&path, &data).unwrap();
        }
        let decoded: ConfigFile = toml::from_str(&data).unwrap();
        Self {
            theme: Theme::load(decoded.theme),
        }
    }
}