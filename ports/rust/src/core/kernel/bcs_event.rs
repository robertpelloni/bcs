use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
pub struct BcsEvent {
    pub event_type: i32,
    pub priority: i32,
}

impl BcsEvent {
    pub fn new(event_type: i32, priority: i32) -> Self {
        BcsEvent { event_type, priority }
    }
}

// Order by priority for BinaryHeap (max-heap by default)
impl Ord for BcsEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for BcsEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct BcsObject {
    pub parent: Option<Weak<RefCell<BcsObject>>>,
    pub children: Vec<Rc<RefCell<BcsObject>>>,
    pub event_filters: Vec<Box<dyn Fn(&BcsEvent) -> bool>>,
}

impl BcsObject {
    pub fn new(parent: Option<Rc<RefCell<BcsObject>>>) -> Rc<RefCell<Self>> {
        let obj = Rc::new(RefCell::new(BcsObject {
            parent: parent.as_ref().map(|p| Rc::downgrade(p)),
            children: Vec::new(),
            event_filters: Vec::new(),
        }));

        if let Some(p) = parent {
            p.borrow_mut().children.push(obj.clone());
        }

        obj
    }

    pub fn install_event_filter<F>(&mut self, filter: F)
    where
        F: Fn(&BcsEvent) -> bool + 'static,
    {
        self.event_filters.push(Box::new(filter));
    }

    pub fn handle_event(&self, e: &BcsEvent) -> bool {
        for filter in &self.event_filters {
            if filter(e) {
                return true;
            }
        }
        false
    }
}

pub struct BcsEventLoop {
    queue: Mutex<BinaryHeap<BcsEvent>>,
    cond: Condvar,
    quit: AtomicBool,
}

impl BcsEventLoop {
    pub fn new() -> Arc<Self> {
        Arc::new(BcsEventLoop {
            queue: Mutex::new(BinaryHeap::new()),
            cond: Condvar::new(),
            quit: AtomicBool::new(false),
        })
    }

    pub fn post_event(&self, e: BcsEvent) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(e);
        self.cond.notify_one();
    }

    pub fn exec(&self) {
        let mut queue = self.queue.lock().unwrap();

        while !self.quit.load(AtomicOrdering::SeqCst) {
            while queue.is_empty() && !self.quit.load(AtomicOrdering::SeqCst) {
                // Wait for condition variable, releasing the queue lock during the wait
                queue = self.cond.wait(queue).unwrap();
            }

            if self.quit.load(AtomicOrdering::SeqCst) {
                break;
            }

            let e_opt = queue.pop();
            drop(queue);

            if let Some(_e) = e_opt {
                // Dispatch event
            }
            queue = self.queue.lock().unwrap();
        }
    }

    pub fn quit(&self) {
        self.quit.store(true, AtomicOrdering::SeqCst);
        self.cond.notify_all();
    }
}
