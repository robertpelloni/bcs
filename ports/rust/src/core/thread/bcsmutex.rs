use std::sync::Mutex;

// BcsMutex wraps a standard rust Mutex with BCS semantics
// Note: In Rust, Mutex owns its data. This wrapper allows interior mutability.
pub struct BcsMutex<T> {
    inner: Mutex<T>,
}

impl<T> BcsMutex<T> {
    pub fn new(data: T) -> Self {
        BcsMutex {
            inner: Mutex::new(data),
        }
    }

    pub fn lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.inner.lock().unwrap();
        f(&mut *guard)
    }
}
