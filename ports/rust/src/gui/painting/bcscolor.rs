#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorSpec {
    Invalid = 0,
    Rgb = 1,
    Hsv = 2,
    Cmyk = 3,
    Hsl = 4,
    ExtendedRgb = 5,
}

#[derive(Clone, Copy)]
pub struct BcsColor {
    spec: ColorSpec,
    a: f32,
    r: f32,
    g: f32,
    b: f32,
}

impl BcsColor {
    pub fn new() -> Self {
        BcsColor {
            spec: ColorSpec::Invalid,
            a: 0.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn new_rgba(r: i32, g: i32, b: i32, a: i32) -> Self {
        let mut color = BcsColor {
            spec: ColorSpec::Rgb,
            a: 0.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
        color.set_rgba(r, g, b, a);
        color
    }

    pub fn set_rgba(&mut self, r: i32, g: i32, b: i32, a: i32) {
        self.r = Self::clamp(r as f32 / 255.0);
        self.g = Self::clamp(g as f32 / 255.0);
        self.b = Self::clamp(b as f32 / 255.0);
        self.a = Self::clamp(a as f32 / 255.0);
        self.spec = ColorSpec::Rgb;
    }

    fn clamp(v: f32) -> f32 {
        if v < 0.0 {
            0.0
        } else if v > 1.0 {
            1.0
        } else {
            v
        }
    }

    pub fn r(&self) -> i32 {
        (self.r * 255.0) as i32
    }

    pub fn g(&self) -> i32 {
        (self.g * 255.0) as i32
    }

    pub fn b(&self) -> i32 {
        (self.b * 255.0) as i32
    }

    pub fn a(&self) -> i32 {
        (self.a * 255.0) as i32
    }

    pub fn is_valid(&self) -> bool {
        self.spec != ColorSpec::Invalid
    }
}
