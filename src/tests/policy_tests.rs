/// Policy unit tests — directly tests PDP decisions against the DYNAMO rules.
///
/// Tests G1–G7 general rules plus all 10 operations.
/// Each test verifies an ALLOW or DENY scenario from the permission matrix.
#[cfg(test)]
mod policy_tests {
    use crate::policy::pdp::Pdp;
    use crate::policy::requests::{Operation, PolicyRequest};
    use crate::policy::responses::PolicyResponse;
    use crate::models::RoleKind::*;

    fn make_subject_req(op: Operation, role: crate::models::RoleKind) -> PolicyRequest {
        let mut req = PolicyRequest::new(op);
        req.session_id = Some("test-session".into());
        req.requester_user_id = Some("u-test".into());
        req.active_role = Some(role.clone());
        req.requester_ward_id = Some("ward-icu".into());
        req.requester_all_roles = vec![role];
        req
    }

    // ---- General rules ----

    #[test]
    fn g2_no_session_denies_read_ehr() {
        let pdp = Pdp::new();
        let mut req = PolicyRequest::new(Operation::ReadEhr);
        // session_id is None → G2 fail
        req.target_ehr_exists = true;
        req.target_ehr_ward_id = Some("ward-icu".into());
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Expected DENY when no session");
    }

    #[test]
    fn g3_invalid_role_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::ReadEhr, Physician);
        // active_role not in requester_all_roles
        req.requester_all_roles = vec![Nurse]; // mismatch
        req.target_ehr_exists = true;
        req.target_ehr_ward_id = Some("ward-icu".into());
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "G3 mismatch should DENY");
    }

    // ---- LOGIN ----

    #[test]
    fn login_valid_role_allows() {
        let pdp = Pdp::new();
        let mut req = PolicyRequest::new(Operation::Login);
        req.login_requested_role = Some(Physician);
        req.login_user_roles = vec![Physician, Nurse];
        req.login_session_already_exists = false;
        req.requester_user_id = Some("u-ph1".into());
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Valid login should ALLOW");
    }

    #[test]
    fn login_unassigned_role_denies() {
        let pdp = Pdp::new();
        let mut req = PolicyRequest::new(Operation::Login);
        req.login_requested_role = Some(Manager);
        req.login_user_roles = vec![Physician]; // Manager not assigned
        req.login_session_already_exists = false;
        req.requester_user_id = Some("u-ph1".into());
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Unassigned role should DENY");
    }

    // ---- READ EHR ----

    #[test]
    fn read_ehr_physician_same_ward_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::ReadEhr, Physician);
        req.target_ehr_exists = true;
        req.target_ehr_ward_id = Some("ward-icu".into()); // same as user ward
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Physician same ward should ALLOW");
    }

    #[test]
    fn read_ehr_patient_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::ReadEhr, Patient);
        req.target_ehr_exists = true;
        req.target_ehr_ward_id = Some("ward-icu".into());
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Patient should DENY read EHR");
    }

    #[test]
    fn read_ehr_nurse_same_ward_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::ReadEhr, Nurse);
        req.target_ehr_exists = true;
        req.target_ehr_ward_id = Some("ward-icu".into());
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Nurse same ward should ALLOW");
    }

    #[test]
    fn read_ehr_physician_different_ward_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::ReadEhr, Physician);
        req.target_ehr_exists = true;
        req.target_ehr_ward_id = Some("ward-surgery".into()); // different ward
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Different ward should DENY");
    }

    #[test]
    fn read_ehr_missing_ehr_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::ReadEhr, Physician);
        req.target_ehr_exists = false; // G4 failure
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Missing EHR should DENY (G4)");
    }

    // ---- FETCH SENSOR ----

    #[test]
    fn fetch_sensor_physician_same_ward_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::FetchSensor, Physician);
        req.target_sensor_exists = true;
        req.target_sensor_ward_id = Some("ward-icu".into());
        req.target_sensor_is_critical = Some(false);
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Physician same ward sensor should ALLOW");
    }

    #[test]
    fn fetch_sensor_nurse_same_ward_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::FetchSensor, Nurse);
        req.target_sensor_exists = true;
        req.target_sensor_ward_id = Some("ward-icu".into());
        req.target_sensor_is_critical = Some(false);
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Nurse same ward sensor should ALLOW");
    }

    #[test]
    fn fetch_sensor_paramedic_critical_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::FetchSensor, Paramedic);
        req.target_sensor_exists = true;
        req.target_sensor_ward_id = Some("ward-icu".into());
        req.target_sensor_is_critical = Some(true); // critical
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Paramedic + critical sensor should ALLOW");
    }

    #[test]
    fn fetch_sensor_paramedic_normal_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::FetchSensor, Paramedic);
        req.target_sensor_exists = true;
        req.target_sensor_ward_id = Some("ward-icu".into());
        req.target_sensor_is_critical = Some(false); // normal
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Paramedic + normal sensor should DENY");
    }

    #[test]
    fn fetch_sensor_patient_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::FetchSensor, Patient);
        req.target_sensor_exists = true;
        req.target_sensor_ward_id = Some("ward-icu".into());
        req.target_sensor_is_critical = Some(true);
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Patient should DENY fetch sensor");
    }

    #[test]
    fn fetch_sensor_manager_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::FetchSensor, Manager);
        req.target_sensor_exists = true;
        req.target_sensor_ward_id = Some("ward-icu".into());
        req.target_sensor_is_critical = Some(true);
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Manager should DENY fetch sensor");
    }

    // ---- ADD USER ----

    #[test]
    fn add_user_manager_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::AddUser, Manager);
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Manager add user should ALLOW");
    }

    #[test]
    fn add_user_clerk_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::AddUser, Clerk);
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Clerk add user should ALLOW");
    }

    #[test]
    fn add_user_physician_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::AddUser, Physician);
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Physician should DENY add user");
    }

    #[test]
    fn add_user_patient_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::AddUser, Patient);
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Patient should DENY add user");
    }

    // ---- REMOVE USER ----

    #[test]
    fn remove_user_manager_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::RemoveUser, Manager);
        req.target_user_id = Some("u-pt1".into());
        req.target_user_exists = true;
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Manager remove user should ALLOW");
    }

    #[test]
    fn remove_user_clerk_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::RemoveUser, Clerk);
        req.target_user_id = Some("u-pt1".into());
        req.target_user_exists = true;
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Clerk should DENY remove user");
    }

    #[test]
    fn remove_user_nonexistent_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::RemoveUser, Manager);
        req.target_user_id = Some("no-such-user".into());
        req.target_user_exists = false; // G4 failure
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Non-existing user should DENY (G4)");
    }

    // ---- CREATE EHR ----

    #[test]
    fn create_ehr_manager_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::CreateEhr, Manager);
        req.target_ehr_exists = false; // unique ID
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Manager create EHR should ALLOW");
    }

    #[test]
    fn create_ehr_clerk_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::CreateEhr, Clerk);
        req.target_ehr_exists = false;
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Clerk create EHR should ALLOW");
    }

    #[test]
    fn create_ehr_nurse_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::CreateEhr, Nurse);
        req.target_ehr_exists = false;
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Nurse should DENY create EHR");
    }

    // ---- DELETE EHR ----

    #[test]
    fn delete_ehr_manager_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::DeleteEhr, Manager);
        req.target_ehr_exists = true;
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Manager delete EHR should ALLOW");
    }

    #[test]
    fn delete_ehr_clerk_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::DeleteEhr, Clerk);
        req.target_ehr_exists = true;
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Clerk should DENY delete EHR");
    }

    #[test]
    fn delete_ehr_missing_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::DeleteEhr, Manager);
        req.target_ehr_exists = false; // G4 failure
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Missing EHR should DENY (G4)");
    }

    // ---- ASSIGN ROLE ----

    #[test]
    fn assign_role_manager_any_role_allows() {
        let pdp = Pdp::new();
        for role in [Patient, Physician, Nurse, Paramedic, Manager, Clerk] {
            let mut req = make_subject_req(Operation::AssignRole, Manager);
            req.target_user_id = Some("u-pt1".into());
            req.target_user_exists = true;
            req.role_to_assign = Some(role.clone());
            let result = pdp.evaluate(&req);
            assert_eq!(result, PolicyResponse::Allow, "Manager assign {:?} should ALLOW", role);
        }
    }

    #[test]
    fn assign_role_clerk_patient_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::AssignRole, Clerk);
        req.target_user_id = Some("u-pt1".into());
        req.target_user_exists = true;
        req.role_to_assign = Some(Patient);
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Clerk assign Patient should ALLOW");
    }

    #[test]
    fn assign_role_clerk_physician_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::AssignRole, Clerk);
        req.target_user_id = Some("u-pt1".into());
        req.target_user_exists = true;
        req.role_to_assign = Some(Physician); // Clerk cannot assign Physician
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Clerk assign Physician should DENY");
    }

    #[test]
    fn assign_role_physician_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::AssignRole, Physician);
        req.target_user_id = Some("u-pt1".into());
        req.target_user_exists = true;
        req.role_to_assign = Some(Patient);
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Physician should DENY assign role");
    }

    // ---- CHANGE WARD ----

    #[test]
    fn change_ward_manager_any_user_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::ChangeWard, Manager);
        req.target_user_id = Some("u-nu1".into());
        req.target_user_exists = true;
        req.target_user_roles = vec![Nurse];
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Manager change any ward should ALLOW");
    }

    #[test]
    fn change_ward_physician_patient_allows() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::ChangeWard, Physician);
        req.target_user_id = Some("u-pt1".into());
        req.target_user_exists = true;
        req.target_user_roles = vec![Patient]; // target is a patient
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert_eq!(result, PolicyResponse::Allow, "Physician change patient ward should ALLOW");
    }

    #[test]
    fn change_ward_physician_nurse_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::ChangeWard, Physician);
        req.target_user_id = Some("u-nu1".into());
        req.target_user_exists = true;
        req.target_user_roles = vec![Nurse]; // target is NOT a patient
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Physician change Nurse ward should DENY");
    }

    #[test]
    fn change_ward_nurse_denies() {
        let pdp = Pdp::new();
        let mut req = make_subject_req(Operation::ChangeWard, Nurse);
        req.target_user_id = Some("u-pt1".into());
        req.target_user_exists = true;
        req.target_user_roles = vec![Patient];
        req.target_ward_exists = true;
        let result = pdp.evaluate(&req);
        assert!(matches!(result, PolicyResponse::Deny(_)), "Nurse should DENY change ward");
    }
}
