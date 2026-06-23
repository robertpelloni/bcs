// BcsString represents the core string type, mapped to Rust's native String
pub struct BcsStringWrapper(String);

impl BcsStringWrapper {
    pub fn new(val: &str) -> Self {
        BcsStringWrapper(val.to_string())
    }

    pub fn to_upper(&self) -> String {
        self.0.to_uppercase()
    }

    pub fn to_lower(&self) -> String {
        self.0.to_lowercase()
    }
}
