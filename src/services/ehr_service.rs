/// EHR Service — read, create, delete EHRs.
///
/// Every operation is gated by the PEP.
use std::sync::MutexGuard;
use crate::services::AppState;
use crate::models::Ehr;
use crate::policy::pep::Pep;
use crate::policy::requests::{Operation, PolicyRequest};
use crate::policy::responses::PolicyResponse;
use crate::errors::AppError;
use crate::utils::helpers::generate_id;

pub struct EhrService;

impl EhrService {
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

    /// Read (retrieve) an EHR.
    pub fn read_ehr(
        state: &MutexGuard<AppState>,
        session_id: &str,
        ehr_id: &str,
        pep: &Pep,
    ) -> Result<Ehr, AppError> {
        let ehr = state.ehrs.find_by_id(ehr_id).cloned();
        let mut req = PolicyRequest::new(Operation::ReadEhr);
        Self::build_subject_fields(state, session_id, &mut req)?;
        req.target_ehr_exists = ehr.is_some();
        req.target_ehr_ward_id = ehr.as_ref().map(|e| e.ward_id.clone());

        match pep.enforce(&req) {
            PolicyResponse::Allow => ehr.ok_or_else(|| AppError::NotFound("EHR not found".into())),
            PolicyResponse::Deny(reason) => Err(AppError::PolicyDenied(reason)),
        }
    }

    /// Create a new EHR in the specified ward.
    pub fn create_ehr(
        state: &mut MutexGuard<AppState>,
        session_id: &str,
        ward_id: &str,
        patient_id: Option<String>,
        notes: Option<String>,
        pep: &Pep,
    ) -> Result<Ehr, AppError> {
        let new_id = generate_id();
        let mut req = PolicyRequest::new(Operation::CreateEhr);
        Self::build_subject_fields(state, session_id, &mut req)?;
        req.target_ehr_exists = state.ehrs.exists(&new_id); // should be false
        req.target_ward_id = Some(ward_id.to_string());
        req.target_ward_exists = state.wards.exists(ward_id);

        match pep.enforce(&req) {
            PolicyResponse::Allow => {
                let ehr = Ehr::new(new_id, ward_id.into(), patient_id, notes);
                state.ehrs.save(ehr.clone());
                Ok(ehr)
            }
            PolicyResponse::Deny(reason) => Err(AppError::PolicyDenied(reason)),
        }
    }

    /// Delete an EHR.
    pub fn delete_ehr(
        state: &mut MutexGuard<AppState>,
        session_id: &str,
        ehr_id: &str,
        pep: &Pep,
    ) -> Result<(), AppError> {
        let mut req = PolicyRequest::new(Operation::DeleteEhr);
        Self::build_subject_fields(state, session_id, &mut req)?;
        req.target_ehr_exists = state.ehrs.exists(ehr_id);
        req.target_ehr_ward_id = state.ehrs.find_by_id(ehr_id).map(|e| e.ward_id.clone());

        match pep.enforce(&req) {
            PolicyResponse::Allow => {
                state.ehrs.delete(ehr_id);
                Ok(())
            }
            PolicyResponse::Deny(reason) => Err(AppError::PolicyDenied(reason)),
        }
    }
}
