use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::global::bcssignal::BcsSignal;
use crate::core::kernel::bcs_event::BcsObject;

// BcsInputOwner maps the conceptual multi-user network ID to an input source
#[derive(Clone)]
pub struct BcsInputOwner {
    pub id: String,
    pub name: String,
}

impl BcsInputOwner {
    pub fn new(id: &str, name: &str) -> Self {
        BcsInputOwner {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}

// BcsInputArbitrator handles multi-user event streams and focus delegation natively
pub struct BcsInputArbitrator {
    active_owners: Arc<RwLock<HashMap<String, BcsInputOwner>>>,
    pub focus_changed: BcsSignal<BcsInputOwner>,
}

impl Default for BcsInputArbitrator {
    fn default() -> Self {
        Self::new()
    }
}

impl BcsInputArbitrator {
    pub fn new() -> Self {
        BcsInputArbitrator {
            active_owners: Arc::new(RwLock::new(HashMap::new())),
            focus_changed: BcsSignal::new(),
        }
    }

    // RegisterOwner adds a new network or local user to the arbitrator's pool
    pub fn register_owner(&self, owner: BcsInputOwner) {
        let mut map = self.active_owners.write().unwrap();
        map.insert(owner.id.clone(), owner);
    }

    // RequestFocus attempts to route the target focus token to the specified owner
    pub fn request_focus(&self, owner_id: &str, target_widget: Option<&Rc<RefCell<BcsObject>>>) -> bool {
        let map = self.active_owners.read().unwrap();
        let owner = map.get(owner_id);

        if owner.is_none() || target_widget.is_none() {
            return false;
        }

        // Emit signal broadcasting focus shift logic
        self.focus_changed.emit_async(owner.unwrap().clone());
        true
    }
}
