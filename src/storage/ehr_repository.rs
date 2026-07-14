use std::collections::HashMap;
use crate::models::Ehr;

/// In-memory repository for EHR objects.
/// No permission checks — pure data access.
pub struct EhrRepository {
    ehrs: HashMap<String, Ehr>,
}

impl EhrRepository {
    pub fn new() -> Self {
        Self { ehrs: HashMap::new() }
    }

    pub fn save(&mut self, ehr: Ehr) {
        self.ehrs.insert(ehr.id.clone(), ehr);
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Ehr> {
        self.ehrs.get(id)
    }

    pub fn exists(&self, id: &str) -> bool {
        self.ehrs.contains_key(id)
    }

    pub fn delete(&mut self, id: &str) -> Option<Ehr> {
        self.ehrs.remove(id)
    }

    pub fn all(&self) -> Vec<&Ehr> {
        self.ehrs.values().collect()
    }
}
