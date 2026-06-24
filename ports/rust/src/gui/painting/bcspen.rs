use bcs_core::global::bcscoretypes::BcsGlobalColor;

// BcsPen defines how lines and outlines are drawn
pub struct BcsPen {
    pub color: BcsGlobalColor,
    pub width: i32,
}

impl BcsPen {
    pub fn new(color: BcsGlobalColor, width: i32) -> Self {
        BcsPen { color, width }
    }
}

// BcsBrush defines how shapes are filled
pub struct BcsBrush {
    pub color: BcsGlobalColor,
}

impl BcsBrush {
    pub fn new(color: BcsGlobalColor) -> Self {
        BcsBrush { color }
    }
}
