// BcsVector is a generic dynamically resizing array, mapped to Rust's Vec
pub struct BcsVector<T> {
    items: Vec<T>,
}

impl<T> BcsVector<T> {
    pub fn new() -> Self {
        BcsVector { items: Vec::new() }
    }

    pub fn append(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn at(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}
