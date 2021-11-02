use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};

use tui::{backend::CrosstermBackend, Terminal};

use crate::app::event::{EventHandler, EventType};
use crate::app::state::AppState;

use crate::ui::ui::{UI, UISTATE};

pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Buffer(0),
        }
    }

    pub fn run(&mut self) {
        info!("Starting app");
        let (mut eh, rx) = EventHandler::new();

        std::thread::spawn(move || {
            eh.handle_event();
        });

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

        let terminal = match Terminal::new(backend) {
            Ok(t) => {
                debug!("Created terminal");
                t
            }
            Err(_) => panic!("Failed to create terminal"),
        };

        let (mut ui, ui_tx) = UI::new(terminal);

        std::thread::spawn(move || {
            ui.draw();
        });

        loop {
            match rx.recv().unwrap() {
                EventType::Input(key) => match key.code {
                    crossterm::event::KeyCode::Char('q') => {
                        ui_tx.send(UISTATE::Quit).unwrap();
                        break;
                    }
                    _ => (),
                },
                EventType::Resize(w, h) => ui_tx.send(UISTATE::Resize(w, h)).unwrap(),
                EventType::Tick => ui_tx.send(UISTATE::Tick).unwrap(),
            }
        }
    }
}
