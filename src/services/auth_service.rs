/// Authentication Service — handles login and logout.
///
/// Flow:
///   login(username, password, role) → PEP → PDP → ALLOW → create Subject
///   logout(session_id)              → PEP → PDP → ALLOW → remove Subject
use std::sync::MutexGuard;
use crate::services::AppState;
use crate::models::{Subject, RoleKind};
use crate::policy::pep::Pep;
use crate::policy::requests::{Operation, PolicyRequest};
use crate::policy::responses::PolicyResponse;
use crate::errors::AppError;
use crate::utils::helpers::{generate_id, verify_password};

pub struct AuthService;

impl AuthService {
    /// Attempt to login a user with a specific active role.
    ///
    /// Returns (session_id, Subject) on success, AppError on failure.
    pub fn login(
        state: &mut MutexGuard<AppState>,
        username: &str,
        password: &str,
        requested_role: &str,
        pep: &Pep,
    ) -> Result<Subject, AppError> {
        // G1: User must exist
        let user = state.users.find_by_username(username)
            .ok_or_else(|| AppError::Unauthorized("Unknown username (G1)".into()))?
            .clone();

        // Verify password
        if !verify_password(password, &user.password_hash) {
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }

        // Parse requested role
        let role = RoleKind::from_str(requested_role)
            .ok_or_else(|| AppError::BadRequest(format!("Unknown role: {}", requested_role)))?;

        // Check if a session for this user+role already exists
        let already_exists = state.subjects
            .sessions_for_user(&user.id)
            .iter()
            .any(|s| s.active_role == role);

        // Build PEP request
        let mut req = PolicyRequest::new(Operation::Login);
        req.login_requested_role = Some(role.clone());
        req.login_user_roles = user.roles.clone();
        req.login_session_already_exists = already_exists;
        req.requester_user_id = Some(user.id.clone());

        match pep.enforce(&req) {
            PolicyResponse::Allow => {
                let session_id = generate_id();
                let subject = Subject::new(session_id, user.id.clone(), role);
                state.subjects.save(subject.clone());
                Ok(subject)
            }
            PolicyResponse::Deny(reason) => Err(AppError::PolicyDenied(reason)),
        }
    }

    /// Logout — destroy an active session.
    pub fn logout(
        state: &mut MutexGuard<AppState>,
        session_id: &str,
        pep: &Pep,
    ) -> Result<(), AppError> {
        // G2/G5: Session must exist
        let subject = state.subjects.find_by_session_id(session_id)
            .ok_or_else(|| AppError::Unauthorized("Session not found (G2)".into()))?
            .clone();

        let user = state.users.find_by_id(&subject.user_id)
            .ok_or_else(|| AppError::Internal("User not found for session".into()))?
            .clone();

        let mut req = PolicyRequest::new(Operation::Logout);
        req.session_id = Some(session_id.to_string());
        req.requester_user_id = Some(user.id.clone());
        req.active_role = Some(subject.active_role.clone());
        req.requester_all_roles = user.roles.clone();
        req.requester_ward_id = Some(user.ward_id.clone());

        match pep.enforce(&req) {
            PolicyResponse::Allow => {
                state.subjects.delete(session_id);
                Ok(())
            }
            PolicyResponse::Deny(reason) => Err(AppError::PolicyDenied(reason)),
        }
    }
}
