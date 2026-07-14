/// Integration tests — multi-step scenarios simulating real hospital workflows.
/// Each test exercises the full stack: login → operation → verify state change.
#[cfg(test)]
mod integration_tests {
    use std::sync::Mutex;
    use crate::services::{AppState, AuthService, UserService, EhrService, SensorService};
    use crate::policy::pep::Pep;
    use crate::errors::AppError;

    fn setup() -> (Mutex<AppState>, Pep) {
        (Mutex::new(AppState::init()), Pep::new())
    }

    fn login(state_lock: &Mutex<AppState>, pep: &Pep, username: &str, role: &str) -> String {
        let mut state = state_lock.lock().unwrap();
        AuthService::login(&mut state, username, "password", role, pep)
            .expect("Login failed in test setup")
            .session_id
    }

    // ── Scenario 1: Manager creates a new user, then physician logs in ────────

    #[test]
    fn scenario_manager_creates_user_then_physician_logs_in() {
        let (state_lock, pep) = setup();
        let mgr_sid = login(&state_lock, &pep, "ronak", "manager");

        // Manager adds a new user
        let new_user = {
            let mut state = state_lock.lock().unwrap();
            UserService::add_user(
                &mut state, &mgr_sid,
                "new_physician", "secret123", "ward-surgery", &pep,
            ).expect("Manager should be able to add user")
        };
        assert_eq!(new_user.username, "new_physician");

        // Manager assigns physician role to the new user
        {
            let mut state = state_lock.lock().unwrap();
            UserService::assign_role(&mut state, &mgr_sid, &new_user.id, "physician", &pep)
                .expect("Manager should assign physician role");
        }

        // New physician can now log in
        let ph_sid = {
            let mut state = state_lock.lock().unwrap();
            AuthService::login(&mut state, "new_physician", "secret123", "physician", &pep)
                .expect("New physician should be able to login")
                .session_id
        };
        assert!(!ph_sid.is_empty());
    }

    // ── Scenario 2: Patient tries to read EHR — must be denied ───────────────

    #[test]
    fn scenario_patient_cannot_read_ehr() {
        let (state_lock, pep) = setup();
        let pt_sid = login(&state_lock, &pep, "vivek", "patient");
        let state = state_lock.lock().unwrap();
        let result = EhrService::read_ehr(&state, &pt_sid, "ehr-001", &pep);
        assert!(
            matches!(result, Err(AppError::PolicyDenied(_))),
            "Patient MUST NOT read EHR"
        );
    }

    // ── Scenario 3: Physician changes patient's ward ──────────────────────────

    #[test]
    fn scenario_physician_changes_patient_ward() {
        let (state_lock, pep) = setup();
        let ph_sid = login(&state_lock, &pep, "devarsya", "physician");

        {
            let mut state = state_lock.lock().unwrap();
            UserService::change_ward(&mut state, &ph_sid, "u-pt1", "ward-surgery", &pep)
                .expect("Physician should change patient ward");
            // Verify the ward changed
            let updated = state.users.find_by_id("u-pt1").unwrap();
            assert_eq!(updated.ward_id, "ward-surgery");
        }
    }

    // ── Scenario 4: Physician cannot change Nurse's ward ─────────────────────

    #[test]
    fn scenario_physician_cannot_change_nurse_ward() {
        let (state_lock, pep) = setup();
        let ph_sid = login(&state_lock, &pep, "devarsya", "physician");
        let mut state = state_lock.lock().unwrap();
        let result = UserService::change_ward(&mut state, &ph_sid, "u-nu1", "ward-surgery", &pep);
        assert!(
            matches!(result, Err(AppError::PolicyDenied(_))),
            "Physician MUST NOT change Nurse's ward"
        );
    }

    // ── Scenario 5: Clerk assigns Physician role — must be denied ─────────────

    #[test]
    fn scenario_clerk_cannot_assign_physician_role() {
        let (state_lock, pep) = setup();
        let clk_sid = login(&state_lock, &pep, "sujal", "clerk");
        let mut state = state_lock.lock().unwrap();
        let result = UserService::assign_role(&mut state, &clk_sid, "u-pt1", "physician", &pep);
        assert!(
            matches!(result, Err(AppError::PolicyDenied(_))),
            "Clerk MUST NOT assign Physician role"
        );
    }

    // ── Scenario 6: Clerk can assign Patient role ─────────────────────────────

    #[test]
    fn scenario_clerk_can_assign_patient_role() {
        let (state_lock, pep) = setup();
        let mgr_sid = login(&state_lock, &pep, "ronak", "manager");
        // First, manager adds a blank user
        let new_user = {
            let mut state = state_lock.lock().unwrap();
            UserService::add_user(&mut state, &mgr_sid, "new_patient_u", "pass", "ward-icu", &pep).unwrap()
        };
        let clk_sid = login(&state_lock, &pep, "sujal", "clerk");
        let mut state = state_lock.lock().unwrap();
        let result = UserService::assign_role(&mut state, &clk_sid, &new_user.id, "patient", &pep);
        assert!(result.is_ok(), "Clerk MUST be able to assign Patient role");
    }

    // ── Scenario 7: Manager removes a user ───────────────────────────────────

    #[test]
    fn scenario_manager_removes_user() {
        let (state_lock, pep) = setup();
        let mgr_sid = login(&state_lock, &pep, "ronak", "manager");
        {
            let mut state = state_lock.lock().unwrap();
            UserService::remove_user(&mut state, &mgr_sid, "u-pt1", &pep)
                .expect("Manager should remove user");
            assert!(state.users.find_by_id("u-pt1").is_none(), "User should be gone");
        }
    }

    // ── Scenario 8: Manager cannot delete non-existing EHR ───────────────────

    #[test]
    fn scenario_manager_cannot_delete_nonexistent_ehr() {
        let (state_lock, pep) = setup();
        let mgr_sid = login(&state_lock, &pep, "ronak", "manager");
        let mut state = state_lock.lock().unwrap();
        let result = EhrService::delete_ehr(&mut state, &mgr_sid, "ehr-DOES-NOT-EXIST", &pep);
        assert!(
            matches!(result, Err(AppError::PolicyDenied(_))),
            "Manager MUST NOT delete non-existing EHR (G4)"
        );
    }

    // ── Scenario 9: Paramedic accesses critical sensor ────────────────────────

    #[test]
    fn scenario_paramedic_critical_sensor_flow() {
        let (state_lock, pep) = setup();
        let pa_sid = login(&state_lock, &pep, "kishan", "paramedic");
        let state = state_lock.lock().unwrap();
        let result = SensorService::fetch_sensor(&state, &pa_sid, "sen-icu-critical", &pep);
        assert!(result.is_ok(), "Paramedic + critical sensor = ALLOW");

        let result2 = SensorService::fetch_sensor(&state, &pa_sid, "sen-icu-normal", &pep);
        assert!(matches!(result2, Err(AppError::PolicyDenied(_))), "Paramedic + normal sensor = DENY");
    }

    // ── Scenario 10: Session becomes invalid after logout ────────────────────

    #[test]
    fn scenario_session_invalid_after_logout() {
        let (state_lock, pep) = setup();
        let ph_sid = login(&state_lock, &pep, "devarsya", "physician");

        // Read EHR works while logged in
        {
            let state = state_lock.lock().unwrap();
            EhrService::read_ehr(&state, &ph_sid, "ehr-001", &pep).expect("Should work while logged in");
        }

        // Logout
        {
            let mut state = state_lock.lock().unwrap();
            AuthService::logout(&mut state, &ph_sid, &pep).expect("Logout should succeed");
        }

        // Read EHR must fail after logout (G2)
        {
            let state = state_lock.lock().unwrap();
            let result = EhrService::read_ehr(&state, &ph_sid, "ehr-001", &pep);
            assert!(
                matches!(result, Err(AppError::Unauthorized(_))),
                "EHR read MUST fail after logout (G2)"
            );
        }
    }
}
