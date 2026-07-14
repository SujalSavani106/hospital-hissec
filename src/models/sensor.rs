use serde::{Deserialize, Serialize};
use crate::models::SensorType;

/// A medical sensor in the hospital.
/// Sensors are located in a ward and have a criticality level.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sensor {
    /// Unique sensor identifier.
    pub id: String,
    /// Ward where this sensor is installed (attr_sw in DYNAMO).
    pub ward_id: String,
    /// Criticality level (attr_sc in DYNAMO).
    pub sensor_type: SensorType,
    /// Human-readable description.
    pub description: Option<String>,
}

impl Sensor {
    pub fn new(id: String, ward_id: String, sensor_type: SensorType, description: Option<String>) -> Self {
        Self { id, ward_id, sensor_type, description }
    }

    /// Returns true if this sensor is critical (DYNAMO: attr_sc(sn) == true).
    pub fn is_critical(&self) -> bool {
        self.sensor_type.is_critical()
    }
}
