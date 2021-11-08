use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct ConfigFile<'a> {
    theme: &'a str,
}

#[derive(Copy, Clone)]
pub struct Config<'a> {
    theme: Theme<'a>,
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Theme<'a> {
    theme: &'a str,
}