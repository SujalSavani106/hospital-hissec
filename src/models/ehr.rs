use serde::{Deserialize, Serialize};

/// An Electronic Health Record (EHR) object.
/// Corresponds to the DYNAMO `O` set (object identifiers).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ehr {
    /// Unique EHR identifier.
    pub id: String,
    /// Ward this EHR belongs to (attr_ow in DYNAMO).
    pub ward_id: String,
    /// Optional associated patient user ID.
    pub patient_id: Option<String>,
    /// Clinical notes.
    pub notes: Option<String>,
}

impl Ehr {
    pub fn new(id: String, ward_id: String, patient_id: Option<String>, notes: Option<String>) -> Self {
        Self { id, ward_id, patient_id, notes }
    }
}
