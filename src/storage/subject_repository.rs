use std::collections::HashMap;
use crate::models::Subject;

/// In-memory repository for active Subject (session) records.
/// No permission checks — pure data access.
pub struct SubjectRepository {
    subjects: HashMap<String, Subject>, // keyed by session_id
    by_user: HashMap<String, Vec<String>>, // user_id -> [session_ids]
}

impl SubjectRepository {
    pub fn new() -> Self {
        Self {
            subjects: HashMap::new(),
            by_user: HashMap::new(),
        }
    }

    pub fn save(&mut self, subject: Subject) {
        let uid = subject.user_id.clone();
        let sid = subject.session_id.clone();
        self.subjects.insert(sid.clone(), subject);
        self.by_user.entry(uid).or_default().push(sid);
    }

    pub fn find_by_session_id(&self, session_id: &str) -> Option<&Subject> {
        self.subjects.get(session_id)
    }

    pub fn exists(&self, session_id: &str) -> bool {
        self.subjects.contains_key(session_id)
    }

    pub fn delete(&mut self, session_id: &str) -> Option<Subject> {
        let subject = self.subjects.remove(session_id)?;
        if let Some(sessions) = self.by_user.get_mut(&subject.user_id) {
            sessions.retain(|s| s != session_id);
        }
        Some(subject)
    }

    pub fn sessions_for_user(&self, user_id: &str) -> Vec<&Subject> {
        self.by_user
            .get(user_id)
            .map(|ids| ids.iter().filter_map(|id| self.subjects.get(id)).collect())
            .unwrap_or_default()
    }
}
