use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::event::{EventHandler, EventType};
//use crate::app::state::AppState;

use crate::ui::ui::{UI, UISTATE};

use crate::config::config::Config;

pub struct App {
    //state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            //state: AppState::Buffer(0),
        }
    }

    pub fn run(&mut self) {
        info!("Starting app");
        let config = Config::load("config/config.toml".to_owned()).unwrap();
        let (mut eh, rx) = EventHandler::new();

        std::thread::spawn(move || {
            eh.handle_event();
        });

        let (mut ui, ui_tx) = UI::new(config.theme);

        let ui_thread = std::thread::spawn(move || {
            ui.draw();
        });

        loop {
            match rx.recv().unwrap() {
                EventType::Input(event) => match event {
                    KeyEvent {
                        modifiers: KeyModifiers::CONTROL,
                        code: KeyCode::Char('q'),
                    } => {
                        ui_tx.send(UISTATE::Quit).unwrap();
                        break;
                    }
                    _ => ui_tx.send(UISTATE::Input(event)).unwrap(),
                },
                EventType::Resize(w, h) => ui_tx.send(UISTATE::Resize(w, h)).unwrap(),
                EventType::Tick => ui_tx.send(UISTATE::Tick).unwrap(),
            }
        }
        ui_thread.join().unwrap();
    }
}
