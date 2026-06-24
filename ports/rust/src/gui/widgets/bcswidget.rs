use bcs_core::kernel::bcs_event::{BcsEvent, BcsObject};
use bcs_core::tools::bcsgeometry::BcsRect;

// BcsWidget is the base class for all UI elements
pub struct BcsWidget {
    rect: BcsRect,
    visible: bool,
}

impl BcsWidget {
    pub fn new() -> Self {
        BcsWidget {
            rect: BcsRect::new(0, 0, 100, 100),
            visible: true,
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
}

// Implement BcsObject for BcsWidget event propagation
impl BcsObject for BcsWidget {
    fn event(&mut self, _e: &BcsEvent) -> bool {
        // Handle UI specific events
        false
    }
}
