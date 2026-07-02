use std::rc::Rc;
use std::cell::RefCell;
use crate::core::kernel::bcs_event::{BcsEvent, BcsObject};
use crate::core::tools::bcsgeometry::BcsRect;

// BcsWidget is the base class for all UI elements
pub struct BcsWidget {
    pub base: Rc<RefCell<BcsObject>>,
    rect: BcsRect,
    visible: bool,
    pub tooltip: String,
    pub description: String,
    pub label: String,
}

impl BcsWidget {
    pub fn new(parent: Option<Rc<RefCell<BcsObject>>>) -> Self {
        BcsWidget {
            base: BcsObject::new(parent),
            rect: BcsRect::new(0, 0, 100, 100),
            visible: true,
            tooltip: String::new(),
            description: String::new(),
            label: String::new(),
        }
    }

    pub fn set_geometry(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.rect = BcsRect::new(x, y, width, height);
    }

    pub fn geometry(&self) -> &BcsRect {
        &self.rect
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn handle_event(&self, e: &BcsEvent) -> bool {
        self.base.borrow().handle_event(e)
    }
}
