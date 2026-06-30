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

#[derive(Debug, Default)]
pub struct AuditEngine {
    events: Vec<AuditEvent>,
}

impl AuditEngine {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    pub fn record_event(&mut self, event: AuditEvent) {
        self.events.push(event);
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn latest_event(&self) -> Option<&AuditEvent> {
        self.events.last()
    }

    pub fn success_count(&self) -> usize {
        self.events
            .iter()
            .filter(|event| event.status == AuditStatus::Success)
            .count()
    }

    pub fn failure_count(&self) -> usize {
        self.events
            .iter()
            .filter(|event| event.status == AuditStatus::Failure)
            .count()
    }

    pub fn events_for_settlement(&self, settlement_id: &str) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|event| event.settlement_id == settlement_id)
            .collect()
    }

    pub fn events_by_action(&self, action: &AuditAction) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|event| &event.action == action)
            .collect()
    }

    pub fn events_by_status(&self, status: &AuditStatus) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|event| &event.status == status)
            .collect()
    }
}
