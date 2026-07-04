use std::rc::Rc;
use std::cell::RefCell;
use crate::core::kernel::bcs_event::BcsObject;

pub struct BcsAction {
    pub base: Rc<RefCell<BcsObject>>,
    text: String,
    enabled: bool,
    visible: bool,
    checkable: bool,
    checked: bool,

    // Signal abstractions
    triggered: Option<Box<dyn Fn(bool)>>,
    toggled: Option<Box<dyn Fn(bool)>>,
}

impl BcsAction {
    pub fn new(parent: Option<Rc<RefCell<BcsObject>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(BcsAction {
            base: BcsObject::new(parent),
            text: String::new(),
            enabled: true,
            visible: true,
            checkable: false,
            checked: false,
            triggered: None,
            toggled: None,
        }))
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_checkable(&mut self, checkable: bool) {
        self.checkable = checkable;
    }

    pub fn is_checkable(&self) -> bool {
        self.checkable
    }

    pub fn set_checked(&mut self, checked: bool) {
        if !self.checkable {
            return;
        }
        if self.checked != checked {
            self.checked = checked;
            if let Some(cb) = &self.toggled {
                cb(self.checked);
            }
        }
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_triggered_callback<F>(&mut self, cb: F)
    where
        F: Fn(bool) + 'static,
    {
        self.triggered = Some(Box::new(cb));
    }

    pub fn set_toggled_callback<F>(&mut self, cb: F)
    where
        F: Fn(bool) + 'static,
    {
        self.toggled = Some(Box::new(cb));
    }

    pub fn trigger(&mut self) {
        if !self.enabled {
            return;
        }
        if self.checkable {
            let next_checked = !self.checked;
            self.set_checked(next_checked);
        }
        if let Some(cb) = &self.triggered {
            cb(self.checked);
        }
    }
}
