// BcsSystemSemaphore abstracts IPC semaphores in Rust
pub struct BcsSystemSemaphore {
    pub key: String,
}

impl BcsSystemSemaphore {
    pub fn new(key: &str) -> Self {
        BcsSystemSemaphore {
            key: key.to_string(),
        }
    }

    pub fn acquire(&self) -> bool {
        // Conceptual IPC acquire block
        true
    }

    pub fn release(&self) -> bool {
        // Conceptual IPC release block
        true
    }

    pub fn set_key(&mut self, key: &str) {
        self.key = key.to_string();
    }
}

// BcsSharedMemory abstracts IPC shared memory in Rust
pub struct BcsSharedMemory {
    pub key: String,
    pub size: usize,
    pub is_attached: bool,
}

impl BcsSharedMemory {
    pub fn new(key: &str) -> Self {
        BcsSharedMemory {
            key: key.to_string(),
            size: 0,
            is_attached: false,
        }
    }

    pub fn attach(&mut self) -> bool {
        self.is_attached = true;
        true
    }

    pub fn detach(&mut self) -> bool {
        self.is_attached = false;
        true
    }

    pub fn set_key(&mut self, key: &str) {
        self.key = key.to_string();
    }
}
