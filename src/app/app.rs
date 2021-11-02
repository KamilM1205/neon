use crossterm::{
    event::{EnableMouseCapture},
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
        let (mut eh, rx) = EventHandler::new();

        std::thread::spawn(move || {
            eh.handle_event();
        });

        enable_raw_mode().unwrap();

        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);

        let terminal = Terminal::new(backend).unwrap();

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
                    },
                    _ => (),
                },
                EventType::Resize(w, h) => ui_tx.send(UISTATE::Resize(w, h)).unwrap(),
                EventType::Tick => ui_tx.send(UISTATE::Redraw).unwrap(),
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
