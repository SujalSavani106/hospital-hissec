use serde::{Deserialize, Serialize};
use crate::models::RoleKind;
use std::time::{SystemTime, UNIX_EPOCH};

/// An active session (Subject) in the HISSEC* model.
/// Created on login, destroyed on logout.
/// Corresponds to the DYNAMO `S` set (subject identifiers).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subject {
    /// Unique session identifier.
    pub session_id: String,
    /// The user who owns this session.
    pub user_id: String,
    /// The single active role chosen at login time.
    pub active_role: RoleKind,
    /// Unix timestamp (seconds) of session creation.
    pub created_at_secs: u64,
}

impl Subject {
    pub fn new(session_id: String, user_id: String, active_role: RoleKind) -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            session_id,
            user_id,
            active_role,
            created_at_secs: ts,
        }
    }
}
