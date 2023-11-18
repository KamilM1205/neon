use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};
use ratatui::buffer::Buffer;
use ratatui::layout::{Direction, Layout, Rect, Constraint};
use ratatui::style::{Style, Modifier};
use ratatui::text::{Span, Line};
use ratatui::widgets::{ListItem, ListState, StatefulWidget, List, Borders, Block};

use crate::config::theme::FileManager;
use crate::ui::file_tree::file_adapter::{FileAdapter, FileType};
use crate::ui::file_tree::menu::Menu;

#[derive(Clone)]
pub struct FileTree {
    pub is_visible: bool,
    file_adapter: FileAdapter,
    state: usize,
    theme: FileManager,
    menu: Menu,
}

impl FileTree {
    pub fn new(theme: FileManager) -> Self {
        let mut file_adapter = FileAdapter::new();
        file_adapter.gen_list();

        Self {
            is_visible: false,
            file_adapter,
            state: 0,
            theme: theme.clone(),
            menu: Menu::new(theme.clone()),
        }
    }

    fn next(&mut self) {
        if self.state < self.file_adapter.files.len() - 1 {
            self.state += 1
        } else {
            self.state = 0
        }
    }

    fn pervious(&mut self) {
        if self.state > 0 {
            self.state -= 1
        } else {
            self.state = self.file_adapter.files.len() - 1
        }
    }

    fn select(&mut self) {
        if self.file_adapter.files[self.state].name == ".." {
            if let Some(parent) = self.file_adapter.curr_dir.parent() {
                self.file_adapter.curr_dir = parent.to_path_buf();
            }
        } else if self.file_adapter.files[self.state].ftype == FileType::Folder {
            self.file_adapter
                .curr_dir
                .push(self.file_adapter.files[self.state].name.clone());
        }
        self.state = 0;
        self.file_adapter.gen_list();
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        if let KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code: KeyCode::Char('a'),
            ..
        } = event
        {
            self.menu.change_visiblity();
            return;
        }
        if !self.menu.get_visible() {
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
        } else {
            self.menu.set_dir(String::from(self.file_adapter.files[self.state].path.to_str().unwrap()));
            self.menu.handle_event(event);
            self.file_adapter.gen_list();
            self.update_state();
        }
    }

    pub fn get_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(Some(self.state));
        state
    }

    pub fn update_state(&mut self) {
        if self.state >= self.file_adapter.files.len() {
            self.state = 0
        }
    }
}

impl StatefulWidget for FileTree {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(if self.menu.get_visible() { 70 } else { 100 }),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(area);
        let items: Vec<ListItem> = self
            .file_adapter
            .files
            .iter()
            .map(|i| {
                let mut lines: Vec<Span> = Vec::new();
                lines.push(Span::styled(
                    i.name.clone(),
                    match i.ftype {
                        FileType::Up => Style::default().fg(self.theme.file),
                        FileType::Folder => Style::default().fg(self.theme.folder),
                        FileType::File => Style::default().fg(self.theme.file),
                    },
                ));
                ListItem::new(Line::from(lines))
                    .style(Style::default().fg(self.theme.select).bg(self.theme.back))
            })
            .collect();

        let items = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.file_adapter.curr_dir.to_str().unwrap())
                    .style(Style::default().bg(self.theme.back).fg(self.theme.folder_select)),
            )
            .highlight_style(
                Style::default()
                    .fg(self.theme.file_select)
                    .bg(self.theme.select)
                    .add_modifier(Modifier::BOLD),
            );

        items.render(chunks[0], buf, state);
        if self.menu.get_visible() {
            let mut state = ListState::default();
            state.select(Some(self.menu.get_state()));
            self.menu.render(chunks[1], buf, &mut state);
        }
    }
}
