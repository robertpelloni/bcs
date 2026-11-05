use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use crate::core::kernel::bcs_event::BcsObject;
use std::rc::Rc;
use std::cell::RefCell;

// BcsTimer provides a cross-platform high-level timer mapping to C++ QTimer natively in Rust
pub struct BcsTimer {
    pub base: Rc<RefCell<BcsObject>>,
    interval: Arc<Mutex<Duration>>,
    single_shot: Arc<Mutex<bool>>,
    active: Arc<Mutex<bool>>,
    cond: Arc<Condvar>,
    timeout_event: Arc<Mutex<Option<Box<dyn Fn() + Send + Sync>>>>,
}

impl BcsTimer {
    pub fn new(parent: Option<Rc<RefCell<BcsObject>>>) -> Self {
        BcsTimer {
            base: BcsObject::new(parent),
            interval: Arc::new(Mutex::new(Duration::from_millis(0))),
            single_shot: Arc::new(Mutex::new(false)),
            active: Arc::new(Mutex::new(false)),
            cond: Arc::new(Condvar::new()),
            timeout_event: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_interval(&self, msec: u64) {
        let mut interval = self.interval.lock().unwrap();
        *interval = Duration::from_millis(msec);
    }

    pub fn set_single_shot(&self, single_shot: bool) {
        let mut ss = self.single_shot.lock().unwrap();
        *ss = single_shot;
    }

    pub fn set_timeout_event<F>(&self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let mut event = self.timeout_event.lock().unwrap();
        *event = Some(Box::new(callback));
    }

    pub fn start(&self) {
        let mut active = self.active.lock().unwrap();
        if *active {
            return;
        }
        *active = true;

        let active_clone = Arc::clone(&self.active);
        let interval_clone = Arc::clone(&self.interval);
        let single_shot_clone = Arc::clone(&self.single_shot);
        let timeout_event_clone = Arc::clone(&self.timeout_event);
        let cond_clone = Arc::clone(&self.cond);

        thread::spawn(move || {
            let single_shot = *single_shot_clone.lock().unwrap();
            let interval = *interval_clone.lock().unwrap();

            if single_shot {
                let mut active_lock = active_clone.lock().unwrap();
                let (active_lock_res, timeout_result) = cond_clone.wait_timeout(active_lock, interval).unwrap();
                active_lock = active_lock_res;

                if *active_lock && timeout_result.timed_out() {
                    drop(active_lock);
                    let event = timeout_event_clone.lock().unwrap();
                    if let Some(cb) = &*event {
                        cb();
                    }
                    active_lock = active_clone.lock().unwrap();
                }
                *active_lock = false;
            } else {
                let mut active_lock = active_clone.lock().unwrap();
                while *active_lock {
                    let wait_res = cond_clone.wait_timeout(active_lock, interval).unwrap();
                    active_lock = wait_res.0;

                    if *active_lock && wait_res.1.timed_out() {
                        drop(active_lock);
                        let event = timeout_event_clone.lock().unwrap();
                        if let Some(cb) = &*event {
                            cb();
                        }
                        active_lock = active_clone.lock().unwrap();
                    }
                }
            }
        });
    }

    pub fn stop(&self) {
        let mut active = self.active.lock().unwrap();
        *active = false;
        self.cond.notify_all();
    }

    pub fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }
}
