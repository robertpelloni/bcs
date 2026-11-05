#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StyleHint {
    Helvetica = 0,
    Times = 1,
    Courier = 2,
    OldEnglish = 3,
    System = 4,
    AnyStyle = 5,
    Cursive = 6,
    Monospace = 7,
    Fantasy = 8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StyleStrategy {
    PreferDefault = 0x0001,
    PreferBitmap = 0x0002,
    PreferDevice = 0x0004,
    PreferOutline = 0x0008,
    ForceOutline = 0x0010,
    PreferMatch = 0x0020,
    PreferQuality = 0x0040,
    PreferAntialias = 0x0080,
    NoAntialias = 0x0100,
    NoSubpixelAntialias = 0x0800,
    PreferNoShaping = 0x1000,
    ContextFontMerging = 0x2000,
    PreferTypoLineMetrics = 0x4000,
    NoFontMerging = 0x8000,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    Thin = 0,
    ExtraLight = 12,
    Light = 25,
    Normal = 50,
    Medium = 57,
    DemiBold = 63,
    Bold = 75,
    ExtraBold = 81,
    Black = 87,
}

#[derive(Clone)]
pub struct BcsFont {
    family: String,
    point_size: f64,
    pixel_size: i32,
    weight: Weight,
    italic: bool,
    underline: bool,
    strike_out: bool,
    fixed_pitch: bool,
    style_hint: StyleHint,
    style_strategy: StyleStrategy,
}

impl BcsFont {
    pub fn new(family: &str, point_size: f64, weight: Weight, italic: bool) -> Self {
        BcsFont {
            family: family.to_string(),
            point_size,
            pixel_size: -1,
            weight,
            italic,
            underline: false,
            strike_out: false,
            fixed_pitch: false,
            style_hint: StyleHint::AnyStyle,
            style_strategy: StyleStrategy::PreferDefault,
        }
    }

    pub fn set_family(&mut self, family: &str) {
        self.family = family.to_string();
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn set_point_size(&mut self, size: f64) {
        self.point_size = size;
        self.pixel_size = -1; // mutually exclusive
    }

    pub fn point_size(&self) -> f64 {
        self.point_size
    }

    pub fn set_pixel_size(&mut self, size: i32) {
        self.pixel_size = size;
        self.point_size = -1.0; // mutually exclusive
    }

    pub fn pixel_size(&self) -> i32 {
        self.pixel_size
    }

    pub fn set_weight(&mut self, weight: Weight) {
        self.weight = weight;
    }

    pub fn weight(&self) -> Weight {
        self.weight
    }

    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
    }

    pub fn italic(&self) -> bool {
        self.italic
    }
}
