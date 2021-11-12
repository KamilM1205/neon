use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ThemeFile {
    theme_header: Option<ThemeHeader>,
    file_manager: Option<FileManager>,
}

#[derive(Clone, Deserialize, Serialize)]
struct ThemeHeader {
    name: String,
    version: String,
    author: String,
    description: String,
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct FileManager {
    back: [u8; 3],
    select: [u8; 3],
    folder: [u8; 3],
    folder_select: [u8; 3],
    file: [u8; 3],
    file_select: [u8; 3],
}

#[derive(Copy, Clone)]
pub struct Theme {
    fmanager: FileManager,
}

#[derive(Debug)]
struct ThemeError {
    e: Box<dyn std::error::Error>,
}

impl ThemeError {
    pub fn new(e: Box<dyn std::error::Error>) -> Self {
        Self { e }
    }
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Theme load error: {}", self.e)
    }
}

impl std::error::Error for ThemeError {
    fn description(&self) -> &str {
        format!("Theme load error: {}", self.e)
    }
}

impl Theme {
    pub fn load(theme: Option<&str>) -> Result<Self, ThemeError> {
        use std::path::PathBuf;
        use tui::style::Color;
        let t: Result<Self, ThemeError> = match theme {
            Some(t) => {
                let file = PathBuf::from(t).with_extension(".toml");
                let path = PathBuf::from("themes/");
                path.push(file);
                if path.exists() {
                } else {
                }
            }
            None => Ok(Theme {
                fmanager: FileManager {
                    back: [0, 0, 0],
                    select: [255, 255, 255],
                    folder: [80, 158, 47],
                    folder_select: [74, 119, 41],
                    file: [93, 93, 93],
                    file_select: [0, 0, 0],
                },
            }),
        };
        t
    }
}
