use std::time::{SystemTime, UNIX_EPOCH};
use crate::policy::requests::PolicyRequest;
use crate::policy::responses::PolicyResponse;

/// Structured audit logger for PDP decisions (rule G7).
/// Every access decision (Allow or Deny) is logged with full context.
pub struct AuditLogger;

impl AuditLogger {
    pub fn new() -> Self {
        AuditLogger
    }

    pub fn log(&self, req: &PolicyRequest, response: &PolicyResponse) {
        let decision = if response.is_allowed() { "ALLOW" } else { "DENY" };
        let reason = response.deny_reason().unwrap_or("-");
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Print to stdout as structured audit entry (G7)
        println!(
            "[AUDIT] ts={} op={} session={} user={} role={} decision={} reason={}",
            ts,
            req.operation.as_str(),
            req.session_id.as_deref().unwrap_or("none"),
            req.requester_user_id.as_deref().unwrap_or("none"),
            req.active_role.as_ref().map(|r| r.as_str()).unwrap_or("none"),
            decision,
            reason
        );
    }
}
