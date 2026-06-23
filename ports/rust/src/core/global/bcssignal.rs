use std::sync::{Arc, Mutex};

// BcsSignal implements the event dispatcher/observer pattern for Rust
pub struct BcsSignal<T> {
    listeners: Arc<Mutex<Vec<Box<dyn Fn(&T) + Send + Sync>>>>,
}

impl<T> BcsSignal<T> {
    pub fn new() -> Self {
        BcsSignal {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn connect<F>(&self, callback: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.push(Box::new(callback));
    }

    pub fn emit(&self, payload: &T) {
        let listeners = self.listeners.lock().unwrap();
        for listener in listeners.iter() {
            listener(payload);
        }
    }
}
