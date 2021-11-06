use tui::{
    backend::Backend,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::ui::file_tree::file_tree::FileTree;

pub enum UISTATE {
    Resize(u16, u16),
    Input(crossterm::event::KeyEvent),
    Tick,
    Quit,
}

pub struct UI {
    rx: std::sync::mpsc::Receiver<UISTATE>,
    fmanager: FileTree,
}

impl UI {
    pub fn new() -> (Self, std::sync::mpsc::Sender<UISTATE>) {
        let (tx, rx) = std::sync::mpsc::channel();
        (
            Self {
                rx,
                fmanager: FileTree::new(),
            },
            tx,
        )
    }

    pub fn draw(&mut self) {
        match enable_raw_mode() {
            Ok(_) => debug!("Enabled raw mode"),
            Err(_) => panic!("Failed to enable raw mode."),
        }

        let mut stdout = std::io::stdout();

        match execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
            Ok(_) => debug!("Enabled mouse capture and entered to alternate screen"),
            Err(_) => panic!("Failed to enable mouse capture and entere to alternate screen"),
        }

        let backend = CrosstermBackend::new(stdout);

        let mut terminal = match Terminal::new(backend) {
            Ok(t) => {
                debug!("Created terminal");
                t
            }
            Err(_) => panic!("Failed to create terminal"),
        };

        loop {
            terminal
            .draw(|f| {
                self.draw_ui(f);
            })
            .unwrap();

            match self.rx.recv().unwrap() {
                UISTATE::Resize(_, _) => (),
                UISTATE::Input(event) => self.handle_ui_input(event),
                UISTATE::Tick => self.update_ui_data(),
                UISTATE::Quit => {
                    info!("Quit");
                    disable_raw_mode().unwrap();
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )
                    .unwrap();
                    terminal.show_cursor().unwrap();
                    break;
                }
            }
        }
    }

    fn update_ui_data(&mut self) {}
    fn handle_ui_input(&mut self, event: crossterm::event::KeyEvent) {
        self.fmanager.handle_event(event);
    }

    fn draw_ui<'a, B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60)].as_ref())
            .split(f.size());
        let mut state = self.fmanager.get_state();
        f.render_stateful_widget(self.fmanager.get_widget(), chunks[0], &mut state);
    }
}
