use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};
use ratatui::{widgets::{StatefulWidget, ListItem, List, Block, Borders, Paragraph, ListState, Widget}, layout::{Rect, Direction, Layout, Constraint}, buffer::Buffer, text::{Span, Line}, style::{Modifier, Style}};

use crate::config::theme::FileManager;

#[derive(Clone)]
pub struct Menu {
    is_visible: bool,
    state: MenuState,
    theme: FileManager,
    items: Vec<String>,
    cur_dir: String,
}

#[derive(Clone)]
enum MenuState {
    Select(usize),
    New(String),
    Move(String),
    Delete(String),
}

impl Menu {
    pub fn new(theme: FileManager) -> Self {
        Self {
            is_visible: false,
            state: MenuState::Select(0),
            theme,
            items: vec![
                "1.Create new object.".to_owned(),
                "2. Move object.".to_owned(),
                "3. Delete object.".to_owned(),
            ],
            cur_dir: "".to_owned(),
        }
    }

    fn next(&mut self) {
        if let MenuState::Select(mut e) = self.state {
            if e < self.items.len() - 1 {
                e += 1
            } else {
                e = 0
            }
            self.state = MenuState::Select(e)
        }
    }

    fn pervious(&mut self) {
        if let MenuState::Select(mut e) = self.state {
            if e > 0 {
                e -= 1
            } else {
                e = self.items.len() - 1
            }
            self.state = MenuState::Select(e)
        }
    }

    fn select(&mut self) {
        if let MenuState::Select(e) = self.state {
            match e {
                0 => self.state = MenuState::New(self.cur_dir.clone()),
                1 => self.state = MenuState::Move(self.cur_dir.clone()),
                2 => self.state = MenuState::Delete(self.cur_dir.clone()),
                _ => (),
            }
        }
    }

    fn new_object(&self, name: String) {
        use std::fs;
        use std::path::PathBuf;
        let mut path = PathBuf::from(name.clone());
        let name: Vec<char> = name.clone().chars().collect();
        if name[name.len() - 1] == '/' {
            fs::create_dir_all(path).unwrap();
        } else {
            let file = PathBuf::from(path.file_name().unwrap());
            path.pop();
            fs::create_dir_all(&path).unwrap();
            path.push(file);
            fs::File::create(path).unwrap();
        }
    }

    fn move_object(&self, name: String) {
        use std::fs;
        use std::path::PathBuf;
        fs::rename(PathBuf::from(self.cur_dir.clone()), PathBuf::from(name)).unwrap();
    }

    fn delete_object(&self, name: String) {
        use std::fs;
        use std::path::PathBuf;
        let path = PathBuf::from(name);
        if path.exists() && path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        } else if path.exists() {
            fs::remove_file(path).unwrap();
        }
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        if let MenuState::Select(_) = self.state {
            match event {
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Up,
                    ..
                } => self.pervious(),
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Down,
                    ..
                } => self.next(),
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Enter,
                    ..
                } => self.select(),
                _ => (),
            }
        } else if let MenuState::New(mut e) = self.state.clone() {
            match event {
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Esc,
                    ..
                } => {
                    self.state = MenuState::Select(0);
                    return;
                }
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Enter,
                    ..
                } => {
                    self.state = MenuState::Select(0);
                    self.new_object(e.clone());
                    return;
                }
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Backspace,
                    ..
                } => {
                    e.pop();
                }
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Char(c),
                    ..
                } => e.push(c),
                _ => (),
            }
            self.state = MenuState::New(e.clone())
        } else if let MenuState::Move(mut e) = self.state.clone() {
            match event {
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Esc,
                    ..
                } => {
                    self.state = MenuState::Select(0);
                    return;
                }
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Enter,
                    ..
                } => {
                    self.state = MenuState::Select(1);
                    self.move_object(e.clone());
                    return;
                }
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Backspace,
                    ..
                } => {
                    e.pop();
                }
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Char(c),
                    ..
                } => e.push(c),
                _ => (),
            }
            self.state = MenuState::Move(e.clone())
        } else if let MenuState::Delete(mut e) = self.state.clone() {
            match event {
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Esc,
                    ..
                } => {
                    self.state = MenuState::Select(0);
                    return;
                }
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Enter,
                    ..
                } => {
                    self.state = MenuState::Select(2);
                    self.delete_object(e.clone());
                    return;
                }
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Backspace,
                    ..
                } => {
                    e.pop();
                }
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Char(c),
                    ..
                } => e.push(c),
                _ => (),
            }
            self.state = MenuState::Delete(e.clone())
        }
    }

    pub fn change_visiblity(&mut self) {
        self.is_visible = !self.is_visible;
    }

    pub fn get_visible(&self) -> bool {
        self.is_visible
    }

    pub fn get_state(&self) -> usize {
        if let MenuState::Select(e) = self.state {
            return e;
        }
        0
    }

    pub fn set_dir(&mut self, dir: String) {
        self.cur_dir = dir;
    }
}

impl StatefulWidget for Menu {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(area);
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| {
                let mut lines: Vec<Span> = Vec::new();
                lines.push(Span::styled(i, Style::default().fg(self.theme.file)));
                ListItem::new(Line::from(lines))
                    .style(Style::default().fg(self.theme.select).bg(self.theme.back))
            })
            .collect();
        let items = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Menu")
                    .style(Style::default().bg(self.theme.back).fg(self.theme.folder_select)),
            )
            .highlight_style(
                Style::default()
                    .fg(self.theme.file_select)
                    .bg(self.theme.select)
                    .add_modifier(Modifier::BOLD),
            );
        let s = match self.state {
            MenuState::New(st) => format!("New file/folder name: {}", st),
            MenuState::Move(st) => format!("Move object to: {}", st),
            MenuState::Delete(st) => format!("Delete object name: {}", st),
            _ => "".to_owned(),
        };

        let text =
            Paragraph::new(s).style(Style::default().bg(self.theme.back).fg(self.theme.file));
        <List as StatefulWidget>::render(items, chunks[0], buf, state);
        text.render(chunks[1], buf);
    }
}
