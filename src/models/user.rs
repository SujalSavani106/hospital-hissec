use serde::{Deserialize, Serialize};
use crate::models::RoleKind;

/// A registered hospital system user.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier (UUID).
    pub id: String,
    /// Login name — must be unique across all users.
    pub username: String,
    /// SHA-256 password hash (hex-encoded).
    pub password_hash: String,
    /// The ward this user belongs to.
    pub ward_id: String,
    /// Set of roles currently assigned to this user.
    pub roles: Vec<RoleKind>,
}

impl User {
    pub fn new(id: String, username: String, password_hash: String, ward_id: String) -> Self {
        Self {
            id,
            username,
            password_hash,
            ward_id,
            roles: Vec::new(),
        }
    }

    pub fn has_role(&self, role: &RoleKind) -> bool {
        self.roles.contains(role)
    }
}
