use bcs_core::kernel::bcs_event::BcsEventLoop;

// BcsApplication manages the GUI application's control flow and main settings.
pub struct BcsApplication {
    event_loop: BcsEventLoop,
    args: Vec<String>,
}

impl BcsApplication {
    pub fn new(args: Vec<String>) -> Self {
        BcsApplication {
            event_loop: BcsEventLoop::new(),
            args,
        }
    }

    pub fn exec(&self) -> i32 {
        println!("BCS Application starting event loop...");
        self.event_loop.exec();
        0
    }

    pub fn quit(&self) {
        println!("BCS Application quitting...");
    }
}
