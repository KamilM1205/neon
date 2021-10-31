pub struct EventHandler {
    tick_rate: std::time::Duration,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            tick_rate: std::time::Duration::from_millis(250),
        }
    }

    pub fn handle_event() {
        
    }
}