use tui::{
    backend::Backend,
    widgets::{Block, Borders},
    layout::{Rect, Layout, Direction, Constraint},
    Terminal, Frame
};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen}
};

use crate::ui::file_tree::file_tree::{FileTree, ElemType};

pub enum UISTATE {
    Redraw,
    Resize(u16, u16),
    Tick,
    Quit,
}

pub struct UI {
    terminal: Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>,
    rx: std::sync::mpsc::Receiver<UISTATE>,
}

fn draw_ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(40),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[2]);
}

fn update_ui_data(ui: &mut UI) {}

impl UI {
    pub fn new(
        terminal: Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>,
    ) -> (Self, std::sync::mpsc::Sender<UISTATE>) {
        let (tx, rx) = std::sync::mpsc::channel();
        (Self { terminal, rx }, tx)
    }

    pub fn draw(&mut self) {
        let mut size = Rect::default();
        loop {
            match self.rx.recv().unwrap() {
                UISTATE::Redraw => {
                    self.terminal
                        .draw(|f| {
                            draw_ui(f);
                        })
                        .unwrap();
                },
                UISTATE::Resize(w, h) => size = Rect::new(0, 0, w, h),
                UISTATE::Tick => update_ui_data(self),
                UISTATE::Quit => {
                    disable_raw_mode().unwrap();
                    execute!(
                        self.terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    ).unwrap();
                    self.terminal.show_cursor().unwrap();
                    break
                },
            }
        }
    }
}
