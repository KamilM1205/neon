use serde::{Deserialize, Serialize};
use ratatui::style::Color;

#[derive(Clone, Deserialize, Serialize)]
pub struct ThemeFile {
    theme_header: Option<ThemeHeader>,
    file_manager: Option<FileManagerFile>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ThemeHeader {
    name: String,
    version: String,
    author: String,
    description: String,
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct FileManagerFile {
    back: [u8; 3],
    select: [u8; 3],
    folder: [u8; 3],
    folder_select: [u8; 3],
    file: [u8; 3],
    file_select: [u8; 3],
}

#[derive(Clone)]
pub struct FileManager {
    pub back: Color,
    pub select: Color,
    pub folder: Color,
    pub folder_select: Color,
    pub file: Color,
    pub file_select: Color,
}

#[derive(Clone)]
pub struct Theme {
    pub header: ThemeHeader,
    pub fmanager: FileManager,
}

#[derive(Debug)]
pub struct ThemeError<T> {
    e: T,
}

impl<'a> From<&'a str> for ThemeError<&'a str> {
    fn from(msg: &'a str) -> Self {
        Self { e: msg }
    }
}

impl From<String> for ThemeError<String> {
    fn from(msg: String) -> Self {
        Self { e: msg }
    }
}

impl<T> std::fmt::Display for ThemeError<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Theme load error: {}", self.e)
    }
}

impl<'a> Theme {
    pub fn load(theme: &str) -> Result<Self, ThemeError<String>> {
        use std::fs;
        use std::path::PathBuf;
        let file = PathBuf::from(&theme).with_extension("toml");
        let mut path = PathBuf::from("themes/");
        path.push(file);
        if !path.exists() {
            path.pop();
            fs::create_dir_all(&path).unwrap();
            return Err(ThemeError::from(format!("File not found: {}", path.to_str().unwrap())));
        }
        let data = fs::read_to_string(path).unwrap();
        let theme: ThemeFile = toml::from_str(&data).unwrap();
        let fmf = theme.file_manager.unwrap();
        let fm = FileManager {
            back: Color::Rgb(fmf.back[0], fmf.back[1], fmf.back[2]),
            select:Color::Rgb(fmf.select[0], fmf.select[1], fmf.select[2]),
            folder: Color::Rgb(fmf.folder[0], fmf.folder[1], fmf.folder[2]),
            folder_select: Color::Rgb(fmf.folder_select[0], fmf.folder_select[1], fmf.folder_select[2]),
            file: Color::Rgb(fmf.file[0], fmf.file[1], fmf.file[2]),
            file_select: Color::Rgb(fmf.file_select[0], fmf.file_select[1], fmf.file_select[2]),
        };
        let theme = Theme {
            header: theme.theme_header.unwrap(),
            fmanager: fm,
        };

        Ok(theme)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            header: ThemeHeader {
                name: "Default(Apprentice)".to_owned(),
                version: "Not specified".to_owned(),
                author: "Jeet Sukumaran".to_owned(),
                description: r#"Apprentice is a dark, low-contrast
                colorscheme for Vim based on the awesome Sorcerer by 
                Jeet Sukumaran."#
                    .to_owned(),
            },
            fmanager: FileManager {
                back: Color::Rgb(0, 0, 0),
                select: Color::Rgb(255, 255, 255),
                folder: Color::Rgb(80, 158, 47),
                folder_select: Color::Rgb(74, 119, 41),
                file: Color::Rgb(93, 93, 93),
                file_select: Color::Rgb(0, 0, 0),
            },
        }
    }
}
