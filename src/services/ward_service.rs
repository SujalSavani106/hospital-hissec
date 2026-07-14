use std::sync::MutexGuard;
use crate::services::AppState;
use crate::models::Ward;

pub struct WardService;

impl WardService {
    /// List all available wards (no permission check — public reference data).
    pub fn list_wards(state: &MutexGuard<AppState>) -> Vec<Ward> {
        state.wards.all().into_iter().cloned().collect()
    }
}
