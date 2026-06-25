use std::thread;
use std::time::Duration;

// BcsThread abstracts thread creation and joining for Rust
pub struct BcsThread {
    handle: Option<thread::JoinHandle<()>>,
}

impl BcsThread {
    pub fn new() -> Self {
        BcsThread { handle: None }
    }

    pub fn start<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.handle = Some(thread::spawn(f));
    }

    pub fn wait(&mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

// sleep halts execution of the current thread for ms milliseconds
pub fn sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
