// BcsPoint defines an X/Y coordinate
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BcsPoint {
    pub x: i32,
    pub y: i32,
}

impl BcsPoint {
    pub fn new(x: i32, y: i32) -> Self {
        BcsPoint { x, y }
    }
}

// BcsSize defines width and height dimensions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BcsSize {
    pub width: i32,
    pub height: i32,
}

impl BcsSize {
    pub fn new(width: i32, height: i32) -> Self {
        BcsSize { width, height }
    }
}

// BcsRect defines a 2D bounding box
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BcsRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl BcsRect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        BcsRect { x, y, width, height }
    }
}
