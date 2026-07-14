/// Sensor service tests — fetch sensor with DYNAMO Rule A / Rule B.
#[cfg(test)]
mod sensor_tests {
    use std::sync::Mutex;
    use crate::services::{AppState, AuthService, SensorService};
    use crate::policy::pep::Pep;
    use crate::errors::AppError;

    fn setup_with_session(username: &str, role: &str) -> (Mutex<AppState>, Pep, String) {
        let state_lock = Mutex::new(AppState::init());
        let pep = Pep::new();
        let session = {
            let mut state = state_lock.lock().unwrap();
            AuthService::login(&mut state, username, "password", role, &pep).unwrap()
        };
        (state_lock, pep, session.session_id)
    }

    // ---- Rule A: Physician/Nurse same ward ----

    #[test]
    fn physician_fetches_normal_sensor_same_ward() {
        let (state_lock, pep, sid) = setup_with_session("devarsya", "physician");
        // devarsya in ward-icu; sen-icu-normal in ward-icu
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &sid, "sen-icu-normal", &pep);
        assert!(result.is_ok(), "Physician same ward normal sensor should ALLOW (Rule A)");
    }

    #[test]
    fn physician_fetches_critical_sensor_same_ward() {
        let (state_lock, pep, sid) = setup_with_session("devarsya", "physician");
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &sid, "sen-icu-critical", &pep);
        assert!(result.is_ok(), "Physician same ward critical sensor should ALLOW (Rule A)");
    }

    #[test]
    fn nurse_fetches_sensor_same_ward() {
        let (state_lock, pep, sid) = setup_with_session("jay", "nurse");
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &sid, "sen-icu-normal", &pep);
        assert!(result.is_ok(), "Nurse same ward sensor should ALLOW (Rule A)");
    }

    #[test]
    fn physician_fetches_sensor_different_ward_denies() {
        let (state_lock, pep, sid) = setup_with_session("devarsya", "physician");
        // sen-surg-normal is in ward-surgery; devarsya is in ward-icu
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &sid, "sen-surg-normal", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Different ward sensor should DENY");
    }

    // ---- Rule B: Paramedic + critical ----

    #[test]
    fn paramedic_fetches_critical_sensor_allows() {
        let (state_lock, pep, sid) = setup_with_session("kishan", "paramedic");
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &sid, "sen-icu-critical", &pep);
        assert!(result.is_ok(), "Paramedic + critical sensor should ALLOW (Rule B)");
    }

    #[test]
    fn paramedic_fetches_normal_sensor_denies() {
        let (state_lock, pep, sid) = setup_with_session("kishan", "paramedic");
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &sid, "sen-icu-normal", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Paramedic + normal sensor should DENY");
    }

    // ---- Other roles ----

    #[test]
    fn patient_fetches_sensor_denies() {
        let (state_lock, pep, sid) = setup_with_session("vivek", "patient");
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &sid, "sen-icu-critical", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Patient fetch sensor should DENY");
    }

    #[test]
    fn manager_fetches_sensor_denies() {
        let (state_lock, pep, sid) = setup_with_session("ronak", "manager");
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &sid, "sen-icu-critical", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Manager fetch sensor should DENY");
    }

    #[test]
    fn clerk_fetches_sensor_denies() {
        let (state_lock, pep, sid) = setup_with_session("sujal", "clerk");
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &sid, "sen-icu-critical", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Clerk fetch sensor should DENY");
    }

    #[test]
    fn fetch_nonexistent_sensor_denies() {
        let (state_lock, pep, sid) = setup_with_session("devarsya", "physician");
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &sid, "sen-ghost", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Non-existent sensor should DENY (G4)");
    }
}
