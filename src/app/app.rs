use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{backend::CrosstermBackend, Terminal};

use crate::app::event::{EventHandler, EventType};

pub struct App {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl App {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();

        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        Self {
            terminal,
        }
    }

    pub fn run(&mut self) {
        let (mut eh, rx) = EventHandler::new();

        std::thread::spawn(move || {
            eh.handle_event();
        });

        loop {
            match rx.recv().unwrap() {
                EventType::Input(key) => match key.code {
                    crossterm::event::KeyCode::Char('q') => {
                        disable_raw_mode().unwrap();
                        execute!(
                            self.terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        ).unwrap();
                        self.terminal.show_cursor().unwrap();
                        break;
                    },
                    _ => (),
                },
                EventType::Tick => (),
            }
        }
    }
}
