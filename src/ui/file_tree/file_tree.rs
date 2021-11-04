use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub struct FileTree {
    pub is_visible: bool,
    items: Vec<String>,
    state: usize,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            items: vec![
                "dgfg".to_owned(),
                "dfgdgdfg".to_owned(),
                "dfgsdgdfg".to_owned(),
            ],
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

    pub fn handle_event(&mut self) {

    }

    pub fn get_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(Some(self.state));
        state
    }
}
