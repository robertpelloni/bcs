use std::sync::{Arc, Mutex};
use std::thread;

pub struct BcsSignal<T: Clone + Send + 'static> {
    listeners: Arc<Mutex<Vec<Box<dyn Fn(&T) + Send + Sync>>>>,
}

impl<T: Clone + Send + 'static> BcsSignal<T> {
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

    pub fn emit_async(&self, payload: T) {
        let listeners = Arc::clone(&self.listeners);
        thread::spawn(move || {
            let listeners = listeners.lock().unwrap();
            for listener in listeners.iter() {
                listener(&payload);
            }
        });
    }
}
