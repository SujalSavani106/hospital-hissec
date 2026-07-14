use serde::{Deserialize, Serialize};

/// A hospital ward.
/// Wards are static — they do not change at runtime.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ward {
    pub id: String,
    pub name: String,
}

impl Ward {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
