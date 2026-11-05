// BcsVariant provides a type-erased enum for dynamic properties in Rust
#[derive(Clone, Debug)]
pub enum BcsVariant {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl BcsVariant {
    pub fn new_int(val: i32) -> Self {
        BcsVariant::Int(val)
    }

    pub fn new_float(val: f64) -> Self {
        BcsVariant::Float(val)
    }

    pub fn new_string(val: &str) -> Self {
        BcsVariant::String(val.to_string())
    }

    pub fn new_bool(val: bool) -> Self {
        BcsVariant::Bool(val)
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, BcsVariant::Nil)
    }

    pub fn get_type(&self) -> String {
        match self {
            BcsVariant::Int(_) => "Int".to_string(),
            BcsVariant::Float(_) => "Float".to_string(),
            BcsVariant::String(_) => "String".to_string(),
            BcsVariant::Bool(_) => "Bool".to_string(),
            BcsVariant::Nil => "Nil".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            BcsVariant::Int(v) => v.to_string(),
            BcsVariant::Float(v) => v.to_string(),
            BcsVariant::String(v) => v.clone(),
            BcsVariant::Bool(v) => v.to_string(),
            BcsVariant::Nil => "Nil".to_string(),
        }
    }

    pub fn to_int(&self) -> Option<i32> {
        if let BcsVariant::Int(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn to_float(&self) -> Option<f64> {
        if let BcsVariant::Float(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        if let BcsVariant::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }
}
