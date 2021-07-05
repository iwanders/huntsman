mod base;
pub mod effects;
pub mod loader;

pub use base::{Canvas, State, RGBA};

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BasicState {
    pub stored: HashMap<String, Canvas>,
    pub base_canvas: Canvas,
    pub last_update_cycle: f64,
}

impl State for BasicState {
    fn get_stored(&self, name: &str) -> Option<Canvas> {
        self.stored.get(name).cloned()
    }

    fn set_stored(&mut self, name: &str, canvas: Canvas) {
        self.stored.insert(name.to_string(), canvas);
    }

    fn get_time(&self) -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        return (since_the_epoch.as_secs() as f64)
            + (since_the_epoch.subsec_nanos() as f64 / 1_000_000_000.0);
    }

    fn get_canvas(&self) -> Canvas {
        self.base_canvas.clone()
    }

    /// Retrieve the time elapsed since last update cycle.
    fn get_elapsed(&self) -> f64 {
        self.get_time() - self.last_update_cycle
    }

    /// Function to tell the state a new update cycle has started.
    fn finish_update(&mut self) {
        self.last_update_cycle = self.get_time();
    }
}
