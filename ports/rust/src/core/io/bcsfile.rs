use std::fs;
use std::path::Path;

// BcsFile abstracts standard file I/O operations for Rust
pub struct BcsFile {
    path: String,
}

impl BcsFile {
    pub fn new(path: &str) -> Self {
        BcsFile {
            path: path.to_string(),
        }
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.path).is_file()
    }

    pub fn read_all(&self) -> std::io::Result<String> {
        fs::read_to_string(&self.path)
    }

    pub fn write_all(&self, payload: &str) -> std::io::Result<()> {
        fs::write(&self.path, payload)
    }
}
