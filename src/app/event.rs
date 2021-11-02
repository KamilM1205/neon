#[derive(Debug)]
pub enum EventType {
    Input(crossterm::event::KeyEvent),
    Resize(u16, u16),
    Tick,
}

pub struct EventHandler {
    tick_rate: std::time::Duration,
    tx: std::sync::mpsc::Sender<EventType>,
}

impl EventHandler {
    pub fn new() -> (Self, std::sync::mpsc::Receiver<EventType>) {
        debug!("Initializing ");

        let (tx, rx) = std::sync::mpsc::channel();
        (
            Self {
                tick_rate: std::time::Duration::from_millis(250),
                tx,
            },
            rx,
        )
    }

    pub fn handle_event(&mut self) {
        debug!("Starting handling events");

        let mut last_tick = std::time::Instant::now();

        loop {
            let timeout = self
                .tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| std::time::Duration::from_secs(0));
            if crossterm::event::poll(timeout).unwrap() {
                match crossterm::event::read().unwrap() {
                    crossterm::event::Event::Key(key) => {
                        debug!("Key pressed: {:?}", key);
                        self.tx.send(EventType::Input(key)).unwrap();
                    }
                    crossterm::event::Event::Resize(w, h) => {
                        debug!("Window resized: {}, {}", w, h);
                        self.tx.send(EventType::Resize(w, h)).unwrap();
                    }
                    _ => (),
                }
            }
            if last_tick.elapsed() >= self.tick_rate {
                self.tx.send(EventType::Tick).unwrap();
                last_tick = std::time::Instant::now();
            }
        }
    }
}
