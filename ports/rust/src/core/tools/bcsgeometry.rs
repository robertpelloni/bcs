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

    pub fn contains(&self, p: &BcsPoint) -> bool {
        p.x >= self.x && p.x <= self.x + self.width &&
        p.y >= self.y && p.y <= self.y + self.height
    }

    pub fn intersects(&self, other: &BcsRect) -> bool {
        self.x < other.x + other.width && self.x + self.width > other.x &&
        self.y < other.y + other.height && self.y + self.height > other.y
    }
}
