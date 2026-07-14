use std::fmt;

/// The outcome of a PDP evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyResponse {
    /// Access is permitted.
    Allow,
    /// Access is denied with a reason string.
    Deny(String),
}

impl PolicyResponse {
    pub fn is_allowed(&self) -> bool {
        matches!(self, PolicyResponse::Allow)
    }

    pub fn deny_reason(&self) -> Option<&str> {
        match self {
            PolicyResponse::Deny(reason) => Some(reason),
            PolicyResponse::Allow        => None,
        }
    }
}

impl fmt::Display for PolicyResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolicyResponse::Allow        => write!(f, "ALLOW"),
            PolicyResponse::Deny(reason) => write!(f, "DENY: {}", reason),
        }
    }
}
