/// Bridge module: re-exports the DYNAMO-generated rules and
/// provides any additional hand-written helper wrappers.
///
/// The actual rule predicates live in policy/generated/policy.rs
/// and are included here via path alias so the PDP has a single
/// import point.
pub use crate::policy_generated::*;
