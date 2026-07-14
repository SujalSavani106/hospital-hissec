use serde::{Deserialize, Serialize};
use std::fmt;

/// The six roles defined in HISSEC*.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RoleKind {
    Patient,
    Physician,
    Nurse,
    Paramedic,
    Manager,
    Clerk,
}

impl RoleKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "patient"   => Some(RoleKind::Patient),
            "physician" => Some(RoleKind::Physician),
            "nurse"     => Some(RoleKind::Nurse),
            "paramedic" => Some(RoleKind::Paramedic),
            "manager"   => Some(RoleKind::Manager),
            "clerk"     => Some(RoleKind::Clerk),
            _           => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RoleKind::Patient   => "patient",
            RoleKind::Physician => "physician",
            RoleKind::Nurse     => "nurse",
            RoleKind::Paramedic => "paramedic",
            RoleKind::Manager   => "manager",
            RoleKind::Clerk     => "clerk",
        }
    }
}

impl fmt::Display for RoleKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Sensor criticality level.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SensorType {
    Normal,
    Critical,
}

impl SensorType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Normal"   | "normal"   => Some(SensorType::Normal),
            "Critical" | "critical" => Some(SensorType::Critical),
            _                       => None,
        }
    }

    pub fn is_critical(&self) -> bool {
        matches!(self, SensorType::Critical)
    }
}

impl fmt::Display for SensorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorType::Normal   => write!(f, "Normal"),
            SensorType::Critical => write!(f, "Critical"),
        }
    }
}
