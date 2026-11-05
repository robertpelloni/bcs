use bcs_gui::widgets::bcswidget::BcsWidget;
use std::rc::Rc;
use std::cell::RefCell;

// BcsLayout governs spatial organization of UI components
pub struct BcsLayout {
    children: Vec<Rc<RefCell<BcsWidget>>>,
    pub margin: i32,
    pub spacing: i32,
}

impl BcsLayout {
    pub fn new() -> Self {
        BcsLayout {
            children: Vec::new(),
            margin: 0,
            spacing: 5,
        }
    }

    pub fn add_widget(&mut self, w: Rc<RefCell<BcsWidget>>) {
        self.children.push(w);
    }

    // Removing requires comparing ptrs or an ID system in Rust
    pub fn remove_widget(&mut self, _w: Rc<RefCell<BcsWidget>>) {
        // Find and remove logic
    }
}
