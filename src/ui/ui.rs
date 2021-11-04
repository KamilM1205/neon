use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame, Terminal,
};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

use crate::ui::file_tree::file_tree::FileTree;

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
        .constraints([Constraint::Percentage(60)].as_ref())
        .split(f.size());
    let mut fmanager = FileTree::new();
    let mut state = fmanager.get_state();
    f.render_stateful_widget(fmanager.get_widget(), chunks[0], &mut state);
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
        self.terminal
            .draw(|f| {
                draw_ui(f);
            })
            .unwrap();
        loop {
            match self.rx.recv().unwrap() {
                UISTATE::Redraw => {
                    self.terminal
                        .draw(|f| {
                            draw_ui(f);
                        })
                        .unwrap();
                }
                UISTATE::Resize(w, h) => {
                    size = Rect::new(0, 0, w, h);
                    self.terminal
                        .draw(|f| {
                            draw_ui(f);
                        })
                        .unwrap();
                }
                UISTATE::Tick => update_ui_data(self),
                UISTATE::Quit => {
                    info!("Quit");
                    disable_raw_mode().unwrap();
                    execute!(
                        self.terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )
                    .unwrap();
                    self.terminal.show_cursor().unwrap();
                    break;
                }
            }
        }
    }
}
