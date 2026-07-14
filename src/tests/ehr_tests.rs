/// EHR service tests — read, create, delete EHRs.
#[cfg(test)]
mod ehr_tests {
    use std::sync::Mutex;
    use crate::services::{AppState, AuthService, EhrService};
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

    // ---- READ EHR ----

    #[test]
    fn physician_reads_same_ward_ehr_allows() {
        let (state_lock, pep, sid) = setup_with_session("devarsya", "physician");
        let state = state_lock.lock().unwrap();
        // devarsya is in ward-icu; ehr-001 is in ward-icu
        let result = EhrService::read_ehr(&state, &sid, "ehr-001", &pep);
        assert!(result.is_ok(), "Physician same ward EHR read should ALLOW");
    }

    #[test]
    fn physician_reads_different_ward_ehr_denies() {
        let (state_lock, pep, sid) = setup_with_session("devarsya", "physician");
        let state = state_lock.lock().unwrap();
        // ehr-002 is in ward-surgery, devarsya is in ward-icu
        let result = EhrService::read_ehr(&state, &sid, "ehr-002", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Different ward should DENY");
    }

    #[test]
    fn patient_reads_ehr_denies() {
        let (state_lock, pep, sid) = setup_with_session("vivek", "patient");
        let state = state_lock.lock().unwrap();
        let result = EhrService::read_ehr(&state, &sid, "ehr-001", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Patient EHR read should DENY");
    }

    #[test]
    fn nurse_reads_same_ward_ehr_allows() {
        let (state_lock, pep, sid) = setup_with_session("jay", "nurse");
        let state = state_lock.lock().unwrap();
        let result = EhrService::read_ehr(&state, &sid, "ehr-001", &pep);
        assert!(result.is_ok(), "Nurse same ward EHR read should ALLOW");
    }

    #[test]
    fn manager_reads_same_ward_ehr_allows() {
        let (state_lock, pep, sid) = setup_with_session("ronak", "manager");
        let state = state_lock.lock().unwrap();
        // ronak is in ward-icu; ehr-001 is in ward-icu
        let result = EhrService::read_ehr(&state, &sid, "ehr-001", &pep);
        assert!(result.is_ok(), "Manager same ward EHR read should ALLOW");
    }

    #[test]
    fn read_nonexistent_ehr_denies() {
        let (state_lock, pep, sid) = setup_with_session("devarsya", "physician");
        let state = state_lock.lock().unwrap();
        let result = EhrService::read_ehr(&state, &sid, "ehr-GHOST", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Non-existent EHR should DENY (G4)");
    }

    // ---- CREATE EHR ----

    #[test]
    fn manager_creates_ehr_allows() {
        let (state_lock, pep, sid) = setup_with_session("ronak", "manager");
        let mut state = state_lock.lock().unwrap();
        let result = EhrService::create_ehr(&mut state, &sid, "ward-icu", None, None, &pep);
        assert!(result.is_ok(), "Manager create EHR should ALLOW");
    }

    #[test]
    fn clerk_creates_ehr_allows() {
        let (state_lock, pep, sid) = setup_with_session("sujal", "clerk");
        let mut state = state_lock.lock().unwrap();
        let result = EhrService::create_ehr(&mut state, &sid, "ward-icu", None, None, &pep);
        assert!(result.is_ok(), "Clerk create EHR should ALLOW");
    }

    #[test]
    fn physician_creates_ehr_denies() {
        let (state_lock, pep, sid) = setup_with_session("devarsya", "physician");
        let mut state = state_lock.lock().unwrap();
        let result = EhrService::create_ehr(&mut state, &sid, "ward-icu", None, None, &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Physician create EHR should DENY");
    }

    // ---- DELETE EHR ----

    #[test]
    fn manager_deletes_existing_ehr_allows() {
        let (state_lock, pep, sid) = setup_with_session("ronak", "manager");
        let mut state = state_lock.lock().unwrap();
        let result = EhrService::delete_ehr(&mut state, &sid, "ehr-001", &pep);
        assert!(result.is_ok(), "Manager delete EHR should ALLOW");
    }

    #[test]
    fn manager_deletes_nonexistent_ehr_denies() {
        let (state_lock, pep, sid) = setup_with_session("ronak", "manager");
        let mut state = state_lock.lock().unwrap();
        let result = EhrService::delete_ehr(&mut state, &sid, "ehr-GHOST", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Non-existent EHR delete should DENY (G4)");
    }

    #[test]
    fn clerk_deletes_ehr_denies() {
        let (state_lock, pep, sid) = setup_with_session("sujal", "clerk");
        let mut state = state_lock.lock().unwrap();
        let result = EhrService::delete_ehr(&mut state, &sid, "ehr-001", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Clerk delete EHR should DENY");
    }
}
