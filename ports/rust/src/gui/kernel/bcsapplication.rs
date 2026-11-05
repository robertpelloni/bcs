use crate::core::kernel::bcs_event::{BcsEventLoop, BcsObject, BcsEvent};
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;

// BcsApplication manages the GUI application's control flow and main settings.
pub struct BcsApplication {
    pub base: Rc<RefCell<BcsObject>>,
    event_loop: Arc<BcsEventLoop>,
    args: Vec<String>,

    pub application_name: String,
    pub application_version: String,
    pub organization_name: String,
    pub organization_domain: String,
}

// Emulate a singleton (usually wrapped in a OnceCell or thread_local in Rust frameworks)
// but for the sake of the port structure we map the methods natively.

impl BcsApplication {
    pub fn new(args: Vec<String>) -> Self {
        BcsApplication {
            base: BcsObject::new(None),
            event_loop: BcsEventLoop::new(),
            args,
            application_name: String::new(),
            application_version: String::new(),
            organization_name: String::new(),
            organization_domain: String::new(),
        }
    }

    pub fn exec(&self) -> i32 {
        println!("BCS Application starting event loop...");
        self.event_loop.exec();
        0
    }

    pub fn quit(&self) {
        println!("BCS Application quitting...");
        self.event_loop.quit();
    }

    pub fn process_events(&self) {
        // Abstract wrapper matching processEvents
    }

    pub fn post_event(&self, _receiver: Rc<RefCell<BcsObject>>, e: BcsEvent) {
        self.event_loop.post_event(e);
    }

    pub fn send_event(&self, receiver: Rc<RefCell<BcsObject>>, e: &BcsEvent) -> bool {
        receiver.borrow().handle_event(e)
    }
}
