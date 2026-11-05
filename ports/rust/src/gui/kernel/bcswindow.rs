use bcs_core::tools::bcsgeometry::BcsRect;

// BcsWindow represents a top-level native window
pub struct BcsWindow {
    title: String,
    rect: BcsRect,
    visible: bool,
}

impl BcsWindow {
    pub fn new() -> Self {
        BcsWindow {
            title: String::new(),
            rect: BcsRect::new(0, 0, 800, 600),
            visible: false,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn set_geometry(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.rect = BcsRect::new(x, y, width, height);
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }
}
