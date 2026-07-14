use std::collections::HashMap;
use crate::models::{User, RoleKind};

/// In-memory repository for Users.
/// No permission checks — pure data access.
pub struct UserRepository {
    users: HashMap<String, User>,        // keyed by user id
    by_username: HashMap<String, String>, // username -> user id
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            by_username: HashMap::new(),
        }
    }

    pub fn save(&mut self, user: User) {
        self.by_username.insert(user.username.clone(), user.id.clone());
        self.users.insert(user.id.clone(), user);
    }

    pub fn find_by_id(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    pub fn find_by_username(&self, username: &str) -> Option<&User> {
        let id = self.by_username.get(username)?;
        self.users.get(id)
    }

    pub fn exists_by_id(&self, id: &str) -> bool {
        self.users.contains_key(id)
    }

    pub fn username_taken(&self, username: &str) -> bool {
        self.by_username.contains_key(username)
    }

    pub fn delete(&mut self, id: &str) -> Option<User> {
        let user = self.users.remove(id)?;
        self.by_username.remove(&user.username);
        Some(user)
    }

    pub fn assign_role(&mut self, user_id: &str, role: RoleKind) -> bool {
        if let Some(user) = self.users.get_mut(user_id) {
            if !user.roles.contains(&role) {
                user.roles.push(role);
            }
            true
        } else {
            false
        }
    }

    pub fn update_ward(&mut self, user_id: &str, new_ward_id: String) -> bool {
        if let Some(user) = self.users.get_mut(user_id) {
            user.ward_id = new_ward_id;
            true
        } else {
            false
        }
    }

    pub fn get_user_mut(&mut self, user_id: &str) -> Option<&mut User> {
        self.users.get_mut(user_id)
    }

    pub fn all(&self) -> Vec<&User> {
        self.users.values().collect()
    }
}
