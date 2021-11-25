use std::io::stdout;
use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Widget},
};

use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
};

use std::path::PathBuf;

#[derive(Clone)]
struct FTab {
    title: &'static str,
    path: PathBuf,
    buffer: Vec<String>,
    line: usize,
    cursor: (u16, u16),
}

#[derive(Clone)]
pub struct Editor {
    tabs: Vec<FTab>,
    current: usize,
}

impl Default for Editor {
    fn default() -> Self {
        execute!(stdout(), Show).unwrap();
        Self {
            tabs: vec![FTab {
                title: "untitled",
                path: PathBuf::from(""),
                buffer: vec![String::new()],
                line: 0,
                cursor: (0, 0),
            }],
            current: 0,
        }
    }
}

impl Editor {
    pub fn handle_event(&mut self, event: KeyEvent) {
        let line = self.tabs[self.current].line;
        match event {
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
            } => {
                if self.tabs[self.current].buffer[self.tabs[self.current].line].len() == 0
                    && self.tabs[self.current].line > 0
                {
                    self.tabs[self.current].buffer.remove(line);
                    self.tabs[self.current].line -= 1;
                } else if self.tabs[self.current].buffer[line].len() > 0 {
                    self.tabs[self.current].buffer[line].pop();
                }
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                self.tabs[self.current]
                    .buffer
                    .insert(line + 1, String::new());
                self.tabs[self.current].line += 1
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
            } => self.tabs[self.current].buffer[line].push(c),
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
            } => {
                if self.tabs[self.current].cursor.0 > 0 {
                    self.tabs[self.current].cursor.0 -= 1
                }
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
            } => {
                if self.tabs[self.current].cursor.0
                    < self.tabs[self.current].buffer[line].len() as u16
                {
                    self.tabs[self.current].cursor.0 += 1;
                }
                execute!(
                    stdout(),
                    Show,
                    MoveTo(
                        self.tabs[self.current].cursor.0,
                        self.tabs[self.current].cursor.1
                    )
                )
                .unwrap();
            }
            _ => {}
        }
    }
}

impl Widget for Editor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (x, y) = self.tabs[self.current].cursor;
        for i in 0..self.tabs[self.current].buffer.len() {
            buf.set_string(
                area.left(),
                area.top() + i as u16,
                self.tabs[self.current].buffer[i].clone(),
                Style::default(),
            );
        }
        execute!(stdout(), MoveTo(5 + area.left(), 5 + area.top()), Show).unwrap();
    }
}
