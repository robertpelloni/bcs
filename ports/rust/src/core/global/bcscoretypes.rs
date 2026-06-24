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
