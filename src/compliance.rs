#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplianceDecision {
    Allow,
    Deny(String),
}

#[derive(Debug, Default)]
pub struct ComplianceEngine;

impl ComplianceEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self) -> ComplianceDecision {
        ComplianceDecision::Allow
    }
}
