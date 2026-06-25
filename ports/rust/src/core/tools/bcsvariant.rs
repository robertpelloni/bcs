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

    pub fn new_string(val: &str) -> Self {
        BcsVariant::String(val.to_string())
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, BcsVariant::Nil)
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
}
