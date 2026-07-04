use std::rc::Rc;
use std::cell::RefCell;
use crate::core::kernel::bcs_event::BcsObject;
use crate::gui::kernel::bcsaction::BcsAction;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExclusionPolicy {
    None,
    Exclusive,
    ExclusiveOptional,
}

pub struct BcsActionGroup {
    pub base: Rc<RefCell<BcsObject>>,
    actions: Vec<Rc<RefCell<BcsAction>>>,
    exclusion_policy: ExclusionPolicy,
    enabled: bool,
    visible: bool,

    // Signals
    triggered: Option<Box<dyn Fn(Rc<RefCell<BcsAction>>)>>,
}

impl BcsActionGroup {
    pub fn new(parent: Option<Rc<RefCell<BcsObject>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(BcsActionGroup {
            base: BcsObject::new(parent),
            actions: Vec::new(),
            exclusion_policy: ExclusionPolicy::Exclusive,
            enabled: true,
            visible: true,
            triggered: None,
        }))
    }

    pub fn add_action(&mut self, action: Rc<RefCell<BcsAction>>) {
        self.actions.push(Rc::clone(&action));
        // Signal forwarding logic would be connected here via BcsSignal mechanism if implemented dynamically
    }

    pub fn actions(&self) -> &Vec<Rc<RefCell<BcsAction>>> {
        &self.actions
    }

    pub fn checked_action(&self) -> Option<Rc<RefCell<BcsAction>>> {
        for a in &self.actions {
            if a.borrow().is_checked() {
                return Some(Rc::clone(a));
            }
        }
        None
    }

    pub fn set_exclusion_policy(&mut self, policy: ExclusionPolicy) {
        self.exclusion_policy = policy;
    }

    pub fn exclusion_policy(&self) -> ExclusionPolicy {
        self.exclusion_policy
    }

    pub fn set_triggered_callback<F>(&mut self, cb: F)
    where
        F: Fn(Rc<RefCell<BcsAction>>) + 'static,
    {
        self.triggered = Some(Box::new(cb));
    }
}
