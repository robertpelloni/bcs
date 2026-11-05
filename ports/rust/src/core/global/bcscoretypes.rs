// BcsGlobalColor defines standard core colors
pub enum BcsGlobalColor {
    White,
    Black,
    Red,
    Green,
    Blue,
    Transparent,
}

pub fn bcs_warning(msg: &str) {
    println!("[BCS Warning] {}", msg);
}

pub fn bcs_critical(msg: &str) {
    println!("[BCS Critical] {}", msg);
}

// BcsAlignment defines vertical and horizontal layout constraints
bitflags::bitflags! {
    pub struct BcsAlignment: u32 {
        const ALIGN_LEFT = 0x0001;
        const ALIGN_RIGHT = 0x0002;
        const ALIGN_HCENTER = 0x0004;
        const ALIGN_JUSTIFY = 0x0008;
        const ALIGN_TOP = 0x0020;
        const ALIGN_BOTTOM = 0x0040;
        const ALIGN_VCENTER = 0x0080;
        const ALIGN_CENTER = Self::ALIGN_VCENTER.bits | Self::ALIGN_HCENTER.bits;
    }
}

pub enum BcsOrientation {
    Horizontal = 0x1,
    Vertical = 0x2,
}
