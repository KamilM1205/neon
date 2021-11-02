use tui::{
    widgets::{Block, Borders},
    layout::{Rect},
    Terminal,
};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen}
};

pub enum UISTATE {
    Redraw,
    Resize(u16, u16),
    Quit,
}

pub struct UI {
    terminal: Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>,
    rx: std::sync::mpsc::Receiver<UISTATE>,
}

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
                            let block = Block::default().title("Block").borders(Borders::ALL);
                            f.render_widget(block, size);
                        })
                        .unwrap();
                },
                UISTATE::Resize(w, h) => size = Rect::new(0, 0, w, h),
                UISTATE::Quit => {
                    disable_raw_mode().unwrap();
                    execute!(
                        self.terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    ).unwrap();
                    self.terminal.show_cursor().unwrap();
                },
            }
        }
    }
}
