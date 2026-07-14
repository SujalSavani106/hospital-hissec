/// Sensor Service — fetch sensor data.
///
/// Gated by PEP (Rule A: Physician/Nurse same ward; Rule B: Paramedic critical only).
use std::sync::MutexGuard;
use crate::services::AppState;
use crate::models::Sensor;
use crate::policy::pep::Pep;
use crate::policy::requests::{Operation, PolicyRequest};
use crate::policy::responses::PolicyResponse;
use crate::errors::AppError;

pub struct SensorService;

impl SensorService {
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

    /// Fetch a sensor — applies DYNAMO rules A and B.
    pub fn fetch_sensor(
        state: &MutexGuard<AppState>,
        session_id: &str,
        sensor_id: &str,
        pep: &Pep,
    ) -> Result<Sensor, AppError> {
        let sensor = state.sensors.find_by_id(sensor_id).cloned();
        let mut req = PolicyRequest::new(Operation::FetchSensor);
        Self::build_subject_fields(state, session_id, &mut req)?;
        req.target_sensor_exists = sensor.is_some();
        if let Some(ref s) = sensor {
            req.target_sensor_ward_id = Some(s.ward_id.clone());
            req.target_sensor_is_critical = Some(s.is_critical());
        }

        match pep.enforce(&req) {
            PolicyResponse::Allow => sensor.ok_or_else(|| AppError::NotFound("Sensor not found".into())),
            PolicyResponse::Deny(reason) => Err(AppError::PolicyDenied(reason)),
        }
    }
}
