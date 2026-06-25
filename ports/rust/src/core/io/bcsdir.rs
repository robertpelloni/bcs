use std::fs;
use std::path::Path;

// BcsDir abstracts standard directory operations for Rust
pub struct BcsDir {
    path: String,
}

impl BcsDir {
    pub fn new(path: &str) -> Self {
        BcsDir {
            path: path.to_string(),
        }
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.path).is_dir()
    }

    pub fn mkdir(&self) -> std::io::Result<()> {
        fs::create_dir(&self.path)
    }

    pub fn mkdir_path(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.path)
    }
}
