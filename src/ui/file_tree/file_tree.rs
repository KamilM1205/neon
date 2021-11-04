use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{List, ListItem, ListState, Widget, Block, Borders},
};

pub enum ElemType {
    FILE(String),
    FOLDER(String),
}

pub struct FileTree<'a> {
    is_visible: bool,
    items: Vec<&'a str>,
    list: ListState,
}

impl<'a> FileTree<'a> {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            items: Vec::new(),
            list: ListState::default(),
        }
    }

    pub fn get_widget(&'a mut self) -> Box<dyn Widget> {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| {
                let mut lines = vec![Spans::from(*i)];
                lines.push(Spans::from(Span::styled(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

            let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
            Box::new(items)
    }
}
