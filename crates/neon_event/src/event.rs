use crate::event_type;

pub struct EventHandler {
    tick_rate: std::time::Duration,
    tx: std::sync::mpsc::Sender<event_type::EventType>,
}

impl EventHandler {
    pub fn new() -> (Self, std::sync::mpsc::Receiver<event_type::EventType>) {
        let (tx, rx) = std::sync::mpsc::channel();
        (
            Self {
                tick_rate: std::time::Duration::from_millis(250),
                tx,
            },
            rx
        )
    }

    pub fn handle_event(&mut self) {
        let mut last_tick = std::time::Instant::now();

        loop {
            let timeout = self.tick_rate.checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));
            
        }
    }
}