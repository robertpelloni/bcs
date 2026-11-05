use super::bcspen::{BcsBrush, BcsPen};
use bcs_core::tools::bcsgeometry::{BcsPoint, BcsRect};

// BcsPainter abstracts native hardware-accelerated and software drawing ops
pub struct BcsPainter {
    pub pen: Option<BcsPen>,
    pub brush: Option<BcsBrush>,
}

impl BcsPainter {
    pub fn new() -> Self {
        BcsPainter {
            pen: None,
            brush: None,
        }
    }

    pub fn set_pen(&mut self, pen: BcsPen) {
        self.pen = Some(pen);
    }

    pub fn set_brush(&mut self, brush: BcsBrush) {
        self.brush = Some(brush);
    }

    pub fn draw_rect(&self, _rect: &BcsRect) {
        // Dispatch native draw rect
    }

    pub fn draw_line(&self, _p1: &BcsPoint, _p2: &BcsPoint) {
        // Dispatch native draw line
    }
}
