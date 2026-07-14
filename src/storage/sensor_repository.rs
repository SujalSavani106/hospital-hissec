use std::collections::HashMap;
use crate::models::Sensor;

/// In-memory repository for Sensor objects.
/// No permission checks — pure data access.
pub struct SensorRepository {
    sensors: HashMap<String, Sensor>,
}

impl SensorRepository {
    pub fn new() -> Self {
        Self { sensors: HashMap::new() }
    }

    pub fn save(&mut self, sensor: Sensor) {
        self.sensors.insert(sensor.id.clone(), sensor);
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Sensor> {
        self.sensors.get(id)
    }

    pub fn exists(&self, id: &str) -> bool {
        self.sensors.contains_key(id)
    }

    pub fn delete(&mut self, id: &str) -> Option<Sensor> {
        self.sensors.remove(id)
    }

    pub fn all(&self) -> Vec<&Sensor> {
        self.sensors.values().collect()
    }
}
