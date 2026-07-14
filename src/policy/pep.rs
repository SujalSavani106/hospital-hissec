//! # Policy Enforcement Point (PEP)
//!
//! The PEP is the **sole gateway** between the service layer and any
//! protected hospital resource. Every operation — login, EHR access,
//! sensor fetch, user management — must pass through [`Pep::enforce`]
//! before any business logic executes.
//!
//! ## Role in the Architecture
//!
//! The PEP sits between services and repositories. It intercepts each
//! request, assembles a complete [`PolicyRequest`] snapshot from the
//! current system state, and delegates the actual decision to the
//! [`Pdp`]. If the PDP returns `Allow`, the service proceeds; if it
//! returns `Deny`, the operation is immediately aborted and the reason
//! is surfaced to the caller.
//!
//! ```text
//! Service
//!   └─► PEP::enforce(PolicyRequest)
//!         └─► Pdp::evaluate(...)
//!               └─► DYNAMO rules  →  Allow / Deny
//!         └─► AuditLogger::log(...)   (G7 — mandatory)
//!         └─► returns PolicyResponse to Service
//! ```
//!
//! ## Security Guarantee
//!
//! Because the PEP wraps `Pdp` internally and is the only code path
//! available to services, it is architecturally impossible to bypass
//! the policy engine. The `AuditLogger` call inside [`Pep::enforce`]
//! is unconditional: **every** decision (including `Allow`) is
//! persisted to the audit trail, satisfying rule **G7**.
//!
//! ## Example
//!
//! The PEP is not called directly by application code. It is injected
//! into each service at startup:
//!
//! ```rust,ignore
//! let pep = Pep::new();
//!
//! // Build a PolicyRequest (done inside the service layer)
//! let mut req = PolicyRequest::new(Operation::ReadEhr);
//! req.session_id        = Some(session_id.into());
//! req.requester_user_id = Some(user_id.into());
//! req.active_role       = Some(RoleKind::Physician);
//! req.requester_ward_id = Some("ward-icu".into());
//! req.target_ehr_exists    = true;
//! req.target_ehr_ward_id   = Some("ward-icu".into());
//!
//! match pep.enforce(&req) {
//!     PolicyResponse::Allow       => { /* proceed with EHR access */ }
//!     PolicyResponse::Deny(reason) => { /* surface error to user  */ }
//! }
//! ```

use crate::policy::pdp::Pdp;
use crate::policy::requests::PolicyRequest;
use crate::policy::responses::PolicyResponse;
use crate::utils::logger::AuditLogger;

/// The Policy Enforcement Point — the sole gatekeeper for every protected operation.
///
/// `Pep` owns a [`Pdp`] and an [`AuditLogger`]. On every call to
/// [`enforce`](Pep::enforce) it:
///
/// 1. Passes the [`PolicyRequest`] to the [`Pdp`] for a stateless rule evaluation.
/// 2. Logs the resulting [`PolicyResponse`] unconditionally (rule **G7**).
/// 3. Returns the response to the caller.
///
/// The struct holds no mutable state and is safe to share across threads.
pub struct Pep {
    pdp: Pdp,
    logger: AuditLogger,
}

impl Pep {
    /// Create a new `Pep` instance with a default [`Pdp`] and [`AuditLogger`].
    ///
    /// Typically constructed once at application startup and passed by
    /// reference into every service that performs protected operations.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let pep = Pep::new();
    /// ```
    pub fn new() -> Self {
        Self {
            pdp: Pdp::new(),
            logger: AuditLogger::new(),
        }
    }

    /// Evaluate a [`PolicyRequest`] and return the access decision.
    ///
    /// This is the single enforcement call that all services must make
    /// before executing any protected operation. The call:
    ///
    /// - Delegates evaluation to the internal [`Pdp`].
    /// - Logs the decision to the audit trail (**G7** — unconditional).
    /// - Returns the [`PolicyResponse`] so the service can act on it.
    ///
    /// # Arguments
    ///
    /// * `req` — A fully populated [`PolicyRequest`] built by the
    ///   service layer. The request must contain all context fields
    ///   relevant to the operation (session, role, target object flags,
    ///   etc.). Missing fields cause the PDP to issue a `Deny`.
    ///
    /// # Returns
    ///
    /// - [`PolicyResponse::Allow`] — the operation may proceed.
    /// - [`PolicyResponse::Deny(reason)`] — the operation is blocked;
    ///   `reason` contains a human-readable explanation.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let response = pep.enforce(&req);
    ///
    /// if response.is_allowed() {
    ///     // safe to perform the operation
    /// } else {
    ///     eprintln!("Access denied: {}", response);
    /// }
    /// ```
    pub fn enforce(&self, req: &PolicyRequest) -> PolicyResponse {
        let response = self.pdp.evaluate(req);

        // G7: Every decision must be logged — unconditional.
        self.logger.log(req, &response);

        response
    }
}

