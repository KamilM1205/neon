use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Widget},
};

use crossterm::{
    event::{KeyEvent, KeyCode, KeyModifiers},
};

use std::path::PathBuf;

#[derive(Clone)]
struct FTabs<'a> {
    title: &'a str,
    path: PathBuf,
    buffer: String,
}

#[derive(Clone)]
pub struct Editor<'a> {
    tabs: Vec<FTabs<'a>>
}

impl<'a> Default for Editor<'a> {
    fn default() -> Self {
        Self {
            tabs: vec!(FTabs {title: "untitled", path: PathBuf::from(""), buffer: "Hello".to_owned()})
        }
    }
}

impl<'a> Editor<'a> {
    pub fn handle_event(&mut self, event: KeyEvent) {

    }
}

impl<'a> Widget for Editor<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.tabs[0].buffer.clone(), Style::default());
    }
}