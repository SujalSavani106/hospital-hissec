use crate::models::RoleKind;

/// The operation being requested.
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Login,
    Logout,
    ReadEhr,
    FetchSensor,
    AddUser,
    RemoveUser,
    CreateEhr,
    DeleteEhr,
    AssignRole,
    ChangeWard,
}

impl Operation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Operation::Login        => "login",
            Operation::Logout       => "logout",
            Operation::ReadEhr      => "read_ehr",
            Operation::FetchSensor  => "fetch_sensor",
            Operation::AddUser      => "add_user",
            Operation::RemoveUser   => "remove_user",
            Operation::CreateEhr    => "create_ehr",
            Operation::DeleteEhr    => "delete_ehr",
            Operation::AssignRole   => "assign_role",
            Operation::ChangeWard   => "change_ward",
        }
    }
}

/// All contextual data the PDP needs to make a decision.
/// The PEP collects this from repositories and packages it here.
/// The PDP is stateless — it reads only from this struct.
#[derive(Debug, Clone)]
pub struct PolicyRequest {
    // ---- Requesting subject fields (G2) ----
    /// Session ID (None = not logged in).
    pub session_id: Option<String>,
    /// User ID of the requesting subject.
    pub requester_user_id: Option<String>,
    /// Active role of the requesting subject (G3).
    pub active_role: Option<RoleKind>,
    /// Ward of the requesting user.
    pub requester_ward_id: Option<String>,
    /// All roles assigned to the requesting user (for G3 validation).
    pub requester_all_roles: Vec<RoleKind>,

    // ---- Operation ----
    pub operation: Operation,

    // ---- Target object fields (context-dependent) ----
    /// Target EHR ward (for read_ehr, delete_ehr, create_ehr).
    pub target_ehr_ward_id: Option<String>,
    /// Target EHR exists flag (G4).
    pub target_ehr_exists: bool,

    /// Target sensor ward (for fetch_sensor).
    pub target_sensor_ward_id: Option<String>,
    /// Target sensor is critical.
    pub target_sensor_is_critical: Option<bool>,
    /// Target sensor exists flag (G4).
    pub target_sensor_exists: bool,

    /// Target user id (for remove_user, assign_role, change_ward).
    pub target_user_id: Option<String>,
    /// Target user ward (for change_ward).
    pub target_user_ward_id: Option<String>,
    /// Target user roles (for change_ward physician rule).
    pub target_user_roles: Vec<RoleKind>,
    /// Target user exists flag (G4).
    pub target_user_exists: bool,

    /// Target ward id (for add_user, create_ehr, change_ward).
    pub target_ward_id: Option<String>,
    /// Target ward exists flag (G6).
    pub target_ward_exists: bool,

    /// Role to assign (for assign_role).
    pub role_to_assign: Option<RoleKind>,

    // ---- Login-specific ----
    /// Requested role at login time.
    pub login_requested_role: Option<RoleKind>,
    /// User's roles at login time.
    pub login_user_roles: Vec<RoleKind>,
    /// Whether a session for this user+role already exists.
    pub login_session_already_exists: bool,
}

impl PolicyRequest {
    /// Minimal constructor — fill only what's needed per operation.
    pub fn new(operation: Operation) -> Self {
        Self {
            session_id: None,
            requester_user_id: None,
            active_role: None,
            requester_ward_id: None,
            requester_all_roles: vec![],
            operation,
            target_ehr_ward_id: None,
            target_ehr_exists: false,
            target_sensor_ward_id: None,
            target_sensor_is_critical: None,
            target_sensor_exists: false,
            target_user_id: None,
            target_user_ward_id: None,
            target_user_roles: vec![],
            target_user_exists: false,
            target_ward_id: None,
            target_ward_exists: false,
            role_to_assign: None,
            login_requested_role: None,
            login_user_roles: vec![],
            login_session_already_exists: false,
        }
    }
}
