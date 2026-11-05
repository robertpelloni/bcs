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

impl std::fmt::Display for BcsStringWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BcsStringWrapper {
    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn substr(&self, index: usize, length: usize) -> String {
        let str = &self.0;
        if index >= str.len() {
            return String::new();
        }
        let end = std::cmp::min(index + length, str.len());
        str[index..end].to_string()
    }

    pub fn to_utf8(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}
