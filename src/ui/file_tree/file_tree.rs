use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crossterm::{
    event::{KeyEvent, KeyCode, KeyModifiers}
};

pub struct FileTree {
    pub is_visible: bool,
    items: Vec<String>,
    state: usize,
}

impl FileTree {
    pub fn new() -> Self {
        let mut list: Vec<String> = Vec::new();

        for i in 1..50 {
            list.push(i.to_string())
        }
        Self {
            is_visible: false,
            items: list,
            state: 0,
        }
    }

    pub fn get_widget(&mut self) -> List {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| {
                let mut lines: Vec<Spans> = Vec::new();
                lines.push(Spans::from(Span::styled(
                    i,
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
                ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Black))
            })
            .collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );
        items
    }

    fn next(&mut self) {
        if self.state < self.items.len() - 1 {
            self.state += 1
        } else {
            self.state = 0
        }
    }

    fn pervious(&mut self) {
        if self.state > 0 {
            self.state -= 1
        } else {
            self.state = self.items.len() - 1
        }
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
            _ => (),
        }
    }

    pub fn get_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(Some(self.state));
        state
    }
}
