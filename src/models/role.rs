use serde::{Deserialize, Serialize};
use crate::models::RoleKind;

/// A role definition in the HISSEC* model.
/// The system has six fixed roles; this struct carries the name as an enum.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub kind: RoleKind,
}

impl Role {
    pub fn new(id: String, kind: RoleKind) -> Self {
        Self { id, kind }
    }
}
