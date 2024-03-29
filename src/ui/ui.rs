use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::config::theme::Theme;
use crate::ui::editor::editor::Editor;
use crate::ui::file_tree::file_tree::FileTree;

pub enum UISTATE {
    Resize(u16, u16),
    Input(crossterm::event::KeyEvent),
    Tick,
    Quit,
}

#[derive(PartialEq)]
enum State {
    Editor,
    FManager,
}

pub struct UI {
    rx: std::sync::mpsc::Receiver<UISTATE>,
    fmanager: FileTree,
    editor: Editor,
    state: State,
}

impl UI {
    pub fn new(theme: Theme) -> (Self, std::sync::mpsc::Sender<UISTATE>) {
        let (tx, rx) = std::sync::mpsc::channel();
        (
            Self {
                rx,
                fmanager: FileTree::new(theme.fmanager),
                state: State::Editor,
                editor: Editor::default(),
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
        
        terminal.draw(|f| {
            self.draw_ui(f);
        }).unwrap();

        loop {
            match self.rx.recv().unwrap() {
                UISTATE::Resize(_, _) => {
                    terminal
                        .draw(|f| {
                            self.draw_ui(f);
                        })
                        .unwrap();
                }
                UISTATE::Input(event) => {
                    self.handle_ui_input(event);
                    terminal
                        .draw(|f| {
                            self.draw_ui(f);
                        })
                        .unwrap();
                }
                UISTATE::Tick => self.update_ui_data(),
                UISTATE::Quit => {
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
            if let State::Editor = self.state {
                let (x, y) = self.editor.get_pos();
                terminal.set_cursor(x, y).unwrap();
                terminal.show_cursor().unwrap();
            } else {
                terminal.hide_cursor().unwrap();
            }
        }
    }

    fn update_ui_data(&mut self) {}

    fn handle_ui_input(&mut self, event: KeyEvent) {
        if let KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code: KeyCode::Char('w'),
            ..
        } = event
        {
            self.state = if self.state == State::Editor {
                State::FManager
            } else {
                State::Editor
            };
        }
        match self.state {
            State::FManager => self.fmanager.handle_event(event),
            State::Editor => self.editor.handle_event(event),
        }
    }

    fn draw_ui<'b>(&mut self, f: &mut Frame) {
        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(95)].as_ref())
            .split(f.size());
        let hchunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(vchunks[0]);
        let mut state = self.fmanager.get_state();
        f.render_stateful_widget(self.fmanager.clone(), hchunks[0], &mut state);
        self.editor.update_area(hchunks[1].left(), hchunks[1].top(), 
                                hchunks[1].right(), hchunks[1].bottom());
        f.render_widget(self.editor.clone(), hchunks[1]);
    }
}
