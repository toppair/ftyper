use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Timer {
    running: bool,
    passed: Arc<Mutex<u32>>,
    limit: u32,
}

impl Timer {
    pub fn new(limit: u32) -> Self {
        Self {
            running: false,
            passed: Arc::new(Mutex::new(0)),
            limit,
        }
    }

    pub fn set(&mut self, limit: u32) {
        self.limit = limit;
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn passed(&self) -> u32 {
        *self.passed.clone().lock().unwrap()
    }

    pub fn is_limit(&self) -> bool {
        self.limit <= *self.passed.clone().lock().unwrap()
    }

    pub fn start(&mut self) {
        self.running = true;
        let passed = self.passed.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(1000));
            *passed.lock().unwrap() += 1;
        });
    }
}
