use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditAction {
    SettlementSubmitted,
    ComplianceApproved,
    ComplianceRejected,
    PolicyApproved,
    PolicyRejected,
    CustodyApproved,
    CustodyREjected,
    SettlementCompleted,
    SettlementFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditStatus {
    Success,
    Failure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub event_id: String,
    pub settlement_id: String,
    pub action: AuditAction,
    pub status: AuditStatus,
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl AuditEvent {
    pub fn new(
        event_id: String,
        settlement_id: String,
        action: AuditAction,
        status: AuditStatus,
        reason: Option<String>,
    ) -> Self {
        Self {
            event_id,
            settlement_id,
            action,
            status,
            reason,
            timestamp: Utc::now(),
        }
    }

    pub fn is_success(&self) -> bool {
        self.status == AuditStatus::Success
    }

    pub fn is_failure(&self) -> bool {
        self.status == AuditStatus::Failure
    }
}
