use std::collections::HashMap;
use crate::models::Ward;

/// In-memory repository for Wards.
/// Wards are static — no create/delete exposed.
pub struct WardRepository {
    wards: HashMap<String, Ward>,
}

impl WardRepository {
    pub fn new() -> Self {
        Self { wards: HashMap::new() }
    }

    /// Seed with initial wards.
    pub fn seed(mut self, wards: Vec<Ward>) -> Self {
        for w in wards {
            self.wards.insert(w.id.clone(), w);
        }
        self
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Ward> {
        self.wards.get(id)
    }

    pub fn exists(&self, id: &str) -> bool {
        self.wards.contains_key(id)
    }

    pub fn all(&self) -> Vec<&Ward> {
        self.wards.values().collect()
    }
}
