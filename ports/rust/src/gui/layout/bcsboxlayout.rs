use super::bcslayout::BcsLayout;

pub enum BcsDirection {
    Horizontal,
    Vertical,
}

// BcsBoxLayout organizes widgets into a horizontal or vertical array
pub struct BcsBoxLayout {
    layout: BcsLayout,
    pub direction: BcsDirection,
}

impl BcsBoxLayout {
    pub fn new(direction: BcsDirection) -> Self {
        BcsBoxLayout {
            layout: BcsLayout::new(),
            direction,
        }
    }

    // Invalidate triggers an algorithmic recalculation of widget bounds
    pub fn invalidate(&mut self) {
        // Recompute sizes
    }
}
