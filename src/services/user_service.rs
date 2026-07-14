/// User Service — add/remove users, assign roles, change wards.
///
/// Every operation goes through the PEP before modifying state.
use std::sync::MutexGuard;
use crate::services::AppState;
use crate::models::{User, RoleKind};
use crate::policy::pep::Pep;
use crate::policy::requests::{Operation, PolicyRequest};
use crate::policy::responses::PolicyResponse;
use crate::errors::AppError;
use crate::utils::helpers::{generate_id, hash_password};

pub struct UserService;

impl UserService {
    /// Helper: build the requesting subject fields from a session_id.
    fn build_subject_fields(
        state: &MutexGuard<AppState>,
        session_id: &str,
        req: &mut PolicyRequest,
    ) -> Result<(), AppError> {
        let subject = state.subjects.find_by_session_id(session_id)
            .ok_or_else(|| AppError::Unauthorized("Session not found (G2)".into()))?;
        let user = state.users.find_by_id(&subject.user_id)
            .ok_or_else(|| AppError::Internal("User not found".into()))?;

        req.session_id = Some(session_id.to_string());
        req.requester_user_id = Some(user.id.clone());
        req.active_role = Some(subject.active_role.clone());
        req.requester_ward_id = Some(user.ward_id.clone());
        req.requester_all_roles = user.roles.clone();
        Ok(())
    }

    /// Add a new user to the system.
    pub fn add_user(
        state: &mut MutexGuard<AppState>,
        session_id: &str,
        username: &str,
        password: &str,
        ward_id: &str,
        pep: &Pep,
    ) -> Result<User, AppError> {
        let mut req = PolicyRequest::new(Operation::AddUser);
        Self::build_subject_fields(state, session_id, &mut req)?;
        req.target_ward_id = Some(ward_id.to_string());
        req.target_ward_exists = state.wards.exists(ward_id);

        if state.users.username_taken(username) {
            return Err(AppError::BadRequest(format!("Username '{}' already taken", username)));
        }

        match pep.enforce(&req) {
            PolicyResponse::Allow => {
                let user = User::new(
                    generate_id(), username.into(),
                    hash_password(password), ward_id.into(),
                );
                state.users.save(user.clone());
                Ok(user)
            }
            PolicyResponse::Deny(reason) => Err(AppError::PolicyDenied(reason)),
        }
    }

    /// Remove an existing user from the system.
    pub fn remove_user(
        state: &mut MutexGuard<AppState>,
        session_id: &str,
        target_user_id: &str,
        pep: &Pep,
    ) -> Result<(), AppError> {
        let mut req = PolicyRequest::new(Operation::RemoveUser);
        Self::build_subject_fields(state, session_id, &mut req)?;
        req.target_user_id = Some(target_user_id.to_string());
        req.target_user_exists = state.users.exists_by_id(target_user_id);

        match pep.enforce(&req) {
            PolicyResponse::Allow => {
                state.users.delete(target_user_id);
                Ok(())
            }
            PolicyResponse::Deny(reason) => Err(AppError::PolicyDenied(reason)),
        }
    }

    /// Assign a role to a user.
    pub fn assign_role(
        state: &mut MutexGuard<AppState>,
        session_id: &str,
        target_user_id: &str,
        role_str: &str,
        pep: &Pep,
    ) -> Result<(), AppError> {
        let role = RoleKind::from_str(role_str)
            .ok_or_else(|| AppError::BadRequest(format!("Unknown role: {}", role_str)))?;

        let mut req = PolicyRequest::new(Operation::AssignRole);
        Self::build_subject_fields(state, session_id, &mut req)?;
        req.target_user_id = Some(target_user_id.to_string());
        req.target_user_exists = state.users.exists_by_id(target_user_id);
        req.role_to_assign = Some(role.clone());

        match pep.enforce(&req) {
            PolicyResponse::Allow => {
                state.users.assign_role(target_user_id, role);
                Ok(())
            }
            PolicyResponse::Deny(reason) => Err(AppError::PolicyDenied(reason)),
        }
    }

    /// Change a user's ward.
    pub fn change_ward(
        state: &mut MutexGuard<AppState>,
        session_id: &str,
        target_user_id: &str,
        new_ward_id: &str,
        pep: &Pep,
    ) -> Result<(), AppError> {
        let target_user = state.users.find_by_id(target_user_id).cloned();
        let mut req = PolicyRequest::new(Operation::ChangeWard);
        Self::build_subject_fields(state, session_id, &mut req)?;
        req.target_user_id = Some(target_user_id.to_string());
        req.target_user_exists = target_user.is_some();
        req.target_ward_id = Some(new_ward_id.to_string());
        req.target_ward_exists = state.wards.exists(new_ward_id);
        if let Some(ref tu) = target_user {
            req.target_user_ward_id = Some(tu.ward_id.clone());
            req.target_user_roles = tu.roles.clone();
        }

        match pep.enforce(&req) {
            PolicyResponse::Allow => {
                state.users.update_ward(target_user_id, new_ward_id.to_string());
                Ok(())
            }
            PolicyResponse::Deny(reason) => Err(AppError::PolicyDenied(reason)),
        }
    }
}
