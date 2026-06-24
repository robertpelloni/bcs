use std::sync::mpsc::{channel, Sender, Receiver};

// BcsEvent represents the base class for all framework events
pub struct BcsEvent {
    pub event_type: i32,
}

impl BcsEvent {
    pub fn new(event_type: i32) -> Self {
        BcsEvent { event_type }
    }
}

// BcsObject is the base trait for event receivers
pub trait BcsObject {
    fn event(&mut self, e: &BcsEvent) -> bool;
}

// BcsEventLoop provides the execution loop for event dispatching
pub struct BcsEventLoop {
    sender: Sender<BcsEvent>,
    receiver: Receiver<BcsEvent>,
}

impl BcsEventLoop {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        BcsEventLoop {
            sender: tx,
            receiver: rx,
        }
    }

    pub fn post_event(&self, e: BcsEvent) {
        let _ = self.sender.send(e);
    }

    pub fn exec(&self) {
        while let Ok(_e) = self.receiver.recv() {
            // Dispatch event
        }
    }
}
