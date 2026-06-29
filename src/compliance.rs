use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplianceDecision {
    Allow,
    Deny(String),
}

#[derive(Debug, Default)]
pub struct ComplianceEngine {
    approved_participants: HashSet<String>,
}

impl ComplianceEngine {
    pub fn new() -> Self {
        Self {
            approved_participants: HashSet::new(),
        }
    }

    pub fn approve_participant(&mut self, participant_id: String) -> bool {
        self.approved_participants.insert(participant_id)
    }

    pub fn is_participant_approved(&self, participant_id: &str) -> bool {
        self.approved_participants.contains(participant_id)
    }

    pub fn evaluate_participant(&self, participant_id: &str) -> ComplianceDecision {
        if self.is_participant_approved(participant_id) {
            ComplianceDecision::Allow
        } else {
            ComplianceDecision::Deny("participant is not approved".to_string())
        }
    }

    pub fn evaluate(&self) -> ComplianceDecision {
        ComplianceDecision::Allow
    }
}
