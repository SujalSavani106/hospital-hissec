/// Login / logout service-level tests using the full AppState.
#[cfg(test)]
mod login_tests {
    use std::sync::Mutex;
    use crate::services::{AppState, AuthService};
    use crate::policy::pep::Pep;
    use crate::errors::AppError;

    fn setup() -> (Mutex<AppState>, Pep) {
        (Mutex::new(AppState::init()), Pep::new())
    }

    #[test]
    fn login_valid_physician() {
        let (state_lock, pep) = setup();
        let mut state = state_lock.lock().unwrap();
        let result = AuthService::login(&mut state, "devarsya", "password", "physician", &pep);
        assert!(result.is_ok(), "Valid physician login should succeed");
        let subject = result.unwrap();
        assert_eq!(subject.user_id, "u-ph1");
    }

    #[test]
    fn login_valid_manager() {
        let (state_lock, pep) = setup();
        let mut state = state_lock.lock().unwrap();
        let result = AuthService::login(&mut state, "ronak", "password", "manager", &pep);
        assert!(result.is_ok(), "Manager login should succeed");
    }

    #[test]
    fn login_unknown_user_denies() {
        let (state_lock, pep) = setup();
        let mut state = state_lock.lock().unwrap();
        let result = AuthService::login(&mut state, "ghost_user", "password", "physician", &pep);
        assert!(matches!(result, Err(AppError::Unauthorized(_))), "Unknown user should fail");
    }

    #[test]
    fn login_wrong_password_denies() {
        let (state_lock, pep) = setup();
        let mut state = state_lock.lock().unwrap();
        let result = AuthService::login(&mut state, "devarsya", "wrongpassword", "physician", &pep);
        assert!(matches!(result, Err(AppError::Unauthorized(_))), "Wrong password should fail");
    }

    #[test]
    fn login_unassigned_role_denies() {
        let (state_lock, pep) = setup();
        let mut state = state_lock.lock().unwrap();
        // devarsya is a Physician, not a Manager
        let result = AuthService::login(&mut state, "devarsya", "password", "manager", &pep);
        assert!(matches!(result, Err(AppError::PolicyDenied(_))), "Unassigned role should DENY");
    }

    #[test]
    fn logout_valid_session() {
        let (state_lock, pep) = setup();
        let mut state = state_lock.lock().unwrap();
        let subject = AuthService::login(&mut state, "devarsya", "password", "physician", &pep).unwrap();
        let logout_result = AuthService::logout(&mut state, &subject.session_id, &pep);
        assert!(logout_result.is_ok(), "Valid logout should succeed");
        // Session should be gone
        assert!(state.subjects.find_by_session_id(&subject.session_id).is_none());
    }

    #[test]
    fn logout_invalid_session_denies() {
        let (state_lock, pep) = setup();
        let mut state = state_lock.lock().unwrap();
        let result = AuthService::logout(&mut state, "no-such-session", &pep);
        assert!(matches!(result, Err(AppError::Unauthorized(_))), "Bad session logout should fail");
    }

    #[test]
    fn login_all_six_roles() {
        let pairs = [
            ("ronak",   "manager"),
            ("sujal",     "clerk"),
            ("devarsya",   "physician"),
            ("jay",  "nurse"),
            ("kishan",   "paramedic"),
            ("vivek",   "patient"),
        ];
        for (username, role) in &pairs {
            let (state_lock, pep) = setup();
            let mut state = state_lock.lock().unwrap();
            let result = AuthService::login(&mut state, username, "password", role, &pep);
            assert!(result.is_ok(), "Login for {} as {} should succeed", username, role);
        }
    }
}
