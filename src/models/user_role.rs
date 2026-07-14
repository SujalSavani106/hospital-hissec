use serde::{Deserialize, Serialize};
use crate::models::RoleKind;

/// Join record linking a User to an assigned Role.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: String,
    pub role: RoleKind,
}

impl UserRole {
    pub fn new(user_id: String, role: RoleKind) -> Self {
        Self { user_id, role }
    }
}
