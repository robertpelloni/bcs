use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::global::bcssignal::BcsSignal;
use crate::core::kernel::bcs_event::BcsObject;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Widget,
    Window,
    Application,
    Workspace,
    Global,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ModalityPolicy {
    Shared,
    OwnerExclusive,
    WindowExclusive,
    ApplicationExclusive,
    SystemExclusive,
}

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

#[derive(Clone)]
pub struct BcsFocusToken {
    pub token_id: String,
    pub owner_id: String,
    pub surface_id: String,
    pub scope: Scope,
    pub modality_policy: ModalityPolicy,
    pub priority: i32,
    pub active: bool,
}

impl BcsFocusToken {
    pub fn new(token_id: &str, owner_id: &str, surface_id: &str) -> Self {
        BcsFocusToken {
            token_id: token_id.to_string(),
            owner_id: owner_id.to_string(),
            surface_id: surface_id.to_string(),
            scope: Scope::Widget,
            modality_policy: ModalityPolicy::Shared,
            priority: 0,
            active: true,
        }
    }
}

#[derive(Clone)]
pub enum RouteDecision {
    Reject,
    Share,
    Transfer,
    Queue,
}

#[derive(Clone)]
pub struct BcsInputRouteRequest {
    pub owner_id: String,
    pub surface_id: String,
    pub device_id: String,
    pub kind: i32,
}

#[derive(Clone)]
pub struct BcsInputRouteResult {
    pub decision: RouteDecision,
    pub resolved_owner_id: String,
    pub blocking_owner_id: String,
    pub blocking_surface_id: String,
}

impl BcsInputRouteResult {
    pub fn accepted(&self) -> bool {
        !matches!(self.decision, RouteDecision::Reject)
    }
}

pub struct BcsInputArbitrator {
    active_owners: Arc<RwLock<HashMap<String, BcsInputOwner>>>,
    focus_tokens: Arc<RwLock<Vec<BcsFocusToken>>>,
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
            focus_tokens: Arc::new(RwLock::new(Vec::new())),
            focus_changed: BcsSignal::new(),
        }
    }

    pub fn register_owner(&self, owner: BcsInputOwner) {
        let mut map = self.active_owners.write().unwrap();
        map.insert(owner.id.clone(), owner);
    }

    pub fn add_focus_token(&self, token: BcsFocusToken) {
        let mut list = self.focus_tokens.write().unwrap();
        list.push(token);
    }

    fn highest_priority_active_token(&self) -> Option<BcsFocusToken> {
        let list = self.focus_tokens.read().unwrap();
        let mut highest: Option<&BcsFocusToken> = None;
        for token in list.iter() {
            if token.active {
                if highest.is_none() || token.priority > highest.unwrap().priority {
                    highest = Some(token);
                }
            }
        }
        highest.cloned()
    }

    pub fn would_block(&self, owner_id: &str, _surface_id: &str) -> bool {
        let highest = self.highest_priority_active_token();
        if let Some(token) = highest {
            if token.modality_policy != ModalityPolicy::Shared && token.owner_id != owner_id {
                return true;
            }
        }
        false
    }

    pub fn route(&self, request: &BcsInputRouteRequest) -> BcsInputRouteResult {
        let mut result = BcsInputRouteResult {
            decision: RouteDecision::Share,
            resolved_owner_id: String::new(),
            blocking_owner_id: String::new(),
            blocking_surface_id: String::new(),
        };

        if let Some(highest) = self.highest_priority_active_token() {
            if highest.modality_policy != ModalityPolicy::Shared && highest.owner_id != request.owner_id {
                result.decision = RouteDecision::Reject;
                result.blocking_owner_id = highest.owner_id.clone();
                result.blocking_surface_id = highest.surface_id.clone();
            }
        }
        result
    }

    pub fn request_focus(&self, owner_id: &str, target_widget: Option<&Rc<RefCell<BcsObject>>>) -> bool {
        let map = self.active_owners.read().unwrap();
        let owner = map.get(owner_id);

        if owner.is_none() || target_widget.is_none() {
            return false;
        }

        self.focus_changed.emit_async(owner.unwrap().clone());
        true
    }
}
