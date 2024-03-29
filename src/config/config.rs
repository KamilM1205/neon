use serde::{Deserialize, Serialize};

use crate::config::theme::{Theme, ThemeError};

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigFile {
    theme: Option<Box<str>>,
    plugin_use: Option<bool>,
}

#[derive(Clone)]
pub struct Config {
    pub theme: Theme,
}

impl<'a> Config {
    pub fn load(path: String) -> Result<Self, ThemeError<&'a str>> {
        use std::fs;
        use std::path;
        let path = path::PathBuf::from(path);
        let data;
        if path.exists() {
            data = fs::read_to_string(path).unwrap();
        } else {
            fs::create_dir_all(&path).unwrap();
            data = toml::to_string(&ConfigFile {
                theme: None,
                plugin_use: Some(false),
            })
            .unwrap();
            fs::write(&path, data.clone()).unwrap();
        }
        
        let decoded: ConfigFile = toml::from_str(&data).unwrap();
        let theme = match Theme::load(&decoded.theme.unwrap()) {
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        };
        Ok(Self { theme })
    }
}
