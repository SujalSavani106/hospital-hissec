// ============================================================
// DYNAMO-Generated Policy Rules — HISSEC*
// Source: policy/hissec.dynamo
//
// This file is the Rust translation of the DYNAMO pre-clauses.
// It contains ONLY pure predicate functions (no side effects).
// It should be treated as auto-generated — do not modify by hand.
//
// The PDP (pdp.rs) imports and calls these functions to decide
// ALLOW / DENY for each operation.
// ============================================================

use crate::models::{RoleKind, SensorType};

// ---- Input snapshots passed from PDP -------------------------

/// Snapshot of a subject's state, extracted from the Subject repository.
#[derive(Debug, Clone)]
pub struct SubjectSnapshot {
    pub session_id: String,
    pub user_id: String,
    pub active_role: RoleKind,
    pub user_ward_id: String,
    pub user_roles: Vec<RoleKind>,
}

/// Snapshot of an EHR's attributes.
#[derive(Debug, Clone)]
pub struct EhrSnapshot {
    pub id: String,
    pub ward_id: String,
}

/// Snapshot of a Sensor's attributes.
#[derive(Debug, Clone)]
pub struct SensorSnapshot {
    pub id: String,
    pub ward_id: String,
    pub sensor_type: SensorType,
}

/// Snapshot of a target User's attributes.
#[derive(Debug, Clone)]
pub struct TargetUserSnapshot {
    pub id: String,
    pub ward_id: String,
    pub roles: Vec<RoleKind>,
}

// ================================================================
// G-rules: Global preconditions (apply to every operation)
// ================================================================

/// G1: User must exist — validated via Option in PDP.
/// G2/G5: Subject must be active — validated via Option in PDP.

/// G3: Active role must belong to the user's assigned roles.
pub fn g3_role_valid_for_subject(subject: &SubjectSnapshot) -> bool {
    subject.user_roles.contains(&subject.active_role)
}

// ================================================================
// Operation-specific DYNAMO pre-clauses
// ================================================================

// ---- LOGIN -------------------------------------------------------

/// Login is allowed if the user exists (checked externally) and
/// the requested role is assigned to the user.
/// The session must not already exist (checked externally for uniqueness).
pub fn can_login(user_roles: &[RoleKind], requested_role: &RoleKind) -> bool {
    user_roles.contains(requested_role)
}

// ---- LOGOUT ------------------------------------------------------

/// Logout is always allowed when the subject exists (G2 already confirmed externally).
/// The subject must own the session (i.e., it is the same session — always true since
/// users can only log out their own session via the session-id header).
pub fn can_logout(_subject: &SubjectSnapshot) -> bool {
    true // G2 (session exists) is the only precondition
}

// ---- READ EHR ----------------------------------------------------

/// DYNAMO rule for read_ehr:
/// Pre: subject_exists(s1)              [G2 — checked externally]
///      AND ehr_exists(o1)              [G4 — checked externally]
///      AND NOT active_role_is(s1, "patient")
///      AND same_ward_ehr(s1, o1)       [attr_uw(sub_user(s1)) == attr_ow(o1)]
pub fn can_read_ehr(subject: &SubjectSnapshot, ehr: &EhrSnapshot) -> bool {
    // NOT patient
    let not_patient = subject.active_role != RoleKind::Patient;
    // Same ward
    let same_ward = subject.user_ward_id == ehr.ward_id;
    not_patient && same_ward
}

// ---- FETCH SENSOR ------------------------------------------------

/// DYNAMO Rule A — fetch_sensor_physician_nurse:
/// Pre: active_role ∈ {physician, nurse} AND same_ward_sensor(s1, sn1)
pub fn can_fetch_sensor_physician_nurse(subject: &SubjectSnapshot, sensor: &SensorSnapshot) -> bool {
    let role_ok = matches!(subject.active_role, RoleKind::Physician | RoleKind::Nurse);
    let same_ward = subject.user_ward_id == sensor.ward_id;
    role_ok && same_ward
}

/// DYNAMO Rule B — fetch_sensor_paramedic:
/// Pre: active_role == paramedic AND is_critical_sensor(sn1)
pub fn can_fetch_sensor_paramedic(subject: &SubjectSnapshot, sensor: &SensorSnapshot) -> bool {
    let role_ok = subject.active_role == RoleKind::Paramedic;
    let is_critical = sensor.sensor_type == SensorType::Critical;
    role_ok && is_critical
}

/// Combined fetch_sensor rule (Rule A OR Rule B).
pub fn can_fetch_sensor(subject: &SubjectSnapshot, sensor: &SensorSnapshot) -> bool {
    can_fetch_sensor_physician_nurse(subject, sensor)
        || can_fetch_sensor_paramedic(subject, sensor)
}

// ---- ADD USER ----------------------------------------------------

/// Pre: active_role ∈ {manager, clerk}
///      AND new username is unique [checked externally]
///      AND ward_exists(w1) [checked externally]
pub fn can_add_user(subject: &SubjectSnapshot) -> bool {
    matches!(subject.active_role, RoleKind::Manager | RoleKind::Clerk)
}

// ---- REMOVE USER -------------------------------------------------

/// Pre: active_role == manager
///      AND user_exists(u_target) [checked externally]
pub fn can_remove_user(subject: &SubjectSnapshot) -> bool {
    subject.active_role == RoleKind::Manager
}

// ---- CREATE EHR --------------------------------------------------

/// Pre: active_role ∈ {manager, clerk}
///      AND ehr id is unique [checked externally]
///      AND ward_exists(w1) [checked externally]
pub fn can_create_ehr(subject: &SubjectSnapshot) -> bool {
    matches!(subject.active_role, RoleKind::Manager | RoleKind::Clerk)
}

// ---- DELETE EHR --------------------------------------------------

/// Pre: active_role == manager
///      AND ehr_exists(o1) [checked externally]
pub fn can_delete_ehr(subject: &SubjectSnapshot) -> bool {
    subject.active_role == RoleKind::Manager
}

// ---- ASSIGN ROLE -------------------------------------------------

/// DYNAMO Rule A — Manager can assign any role.
pub fn can_assign_role_manager(subject: &SubjectSnapshot) -> bool {
    subject.active_role == RoleKind::Manager
}

/// DYNAMO Rule B — Clerk can assign patient role only.
pub fn can_assign_role_clerk(subject: &SubjectSnapshot, target_role: &RoleKind) -> bool {
    subject.active_role == RoleKind::Clerk && *target_role == RoleKind::Patient
}

/// Combined assign_role rule.
pub fn can_assign_role(subject: &SubjectSnapshot, target_role: &RoleKind) -> bool {
    can_assign_role_manager(subject) || can_assign_role_clerk(subject, target_role)
}

// ---- CHANGE WARD -------------------------------------------------

/// DYNAMO Rule A — Manager can change any user's ward.
pub fn can_change_ward_manager(subject: &SubjectSnapshot) -> bool {
    subject.active_role == RoleKind::Manager
}

/// DYNAMO Rule B — Physician can only change patients' ward.
/// Pre: active_role == physician AND user_has_role(u_target, "patient")
pub fn can_change_ward_physician(subject: &SubjectSnapshot, target: &TargetUserSnapshot) -> bool {
    subject.active_role == RoleKind::Physician && target.roles.contains(&RoleKind::Patient)
}

/// Combined change_ward rule.
pub fn can_change_ward(subject: &SubjectSnapshot, target: &TargetUserSnapshot) -> bool {
    can_change_ward_manager(subject) || can_change_ward_physician(subject, target)
}
