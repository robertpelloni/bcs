use std::rc::Rc;
use std::sync::Arc;

// BcsSharedPointer maps to Rust's Arc for thread-safe shared ownership
pub type BcsSharedPointer<T> = Arc<T>;

// BcsUniquePointer maps to Rust's Box for exclusive ownership
pub type BcsUniquePointer<T> = Box<T>;

// Local (non-thread-safe) shared pointer alias
pub type BcsLocalSharedPointer<T> = Rc<T>;

pub fn new_shared<T>(val: T) -> BcsSharedPointer<T> {
    Arc::new(val)
}

pub fn new_unique<T>(val: T) -> BcsUniquePointer<T> {
    Box::new(val)
}
