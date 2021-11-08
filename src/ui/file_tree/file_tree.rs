use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::ui::file_tree::file_adapter::{FileAdapter, FileType};

pub struct FileTree {
    pub is_visible: bool,
    file_adapter: FileAdapter,
    state: usize,
}

impl FileTree {
    pub fn new() -> Self {
        let mut file_adapter = FileAdapter::new();
        file_adapter.gen_list();

        Self {
            is_visible: false,
            file_adapter,
            state: 0,
        }
    }

    pub fn get_widget(&mut self) -> List {
        let items: Vec<ListItem> = self
            .file_adapter
            .files
            .iter()
            .map(|i| {
                let mut lines: Vec<Spans> = Vec::new();
                lines.push(Spans::from(Span::styled(
                    i.name.clone(),
                    match i.ftype {
                        FileType::Up => Style::default(),
                        FileType::Folder => Style::default().fg(Color::Green),
                        FileType::File => Style::default(),
                    },
                )));
                ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Black))
            })
            .collect();

        let items = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.file_adapter.curr_dir.to_str().unwrap()),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );
        items
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
        match event {
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Up,
            } => self.pervious(),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Down,
            } => self.next(),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Enter,
            } => self.select(),
            _ => (),
        }
    }

    pub fn get_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(Some(self.state));
        state
    }
}
