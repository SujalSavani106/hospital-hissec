use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a pseudo-unique ID without external crates.
/// Format: timestamp_nanos-counter (hex).
pub fn generate_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:016x}-{:08x}", nanos, count)
}

/// Hash a password using a simple deterministic hash.
/// NOTE: For a production system use bcrypt/argon2.
/// Using std::hash here avoids the cpufeatures/edition2024 issue
/// with older toolchains.
pub fn hash_password(password: &str) -> String {
    let mut hasher = DefaultHasher::new();
    "HISSEC_SALT_v1".hash(&mut hasher);
    password.hash(&mut hasher);
    format!("HASH:{:016x}", hasher.finish())
}

/// Verify a password against its stored hash.
pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    hash_password(password) == stored_hash
}
