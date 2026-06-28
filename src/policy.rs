use crate::settlement::SettlementInstruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny(String),
}

#[derive(Debug)]
pub struct PolicyEngine {
    max_settlement_quantity: u64,
}

impl PolicyEngine {
    pub fn new(max_settlement_quantity: u64) -> Self {
        Self {
            max_settlement_quantity,
        }
    }

    pub fn evaluate_settlement(&self, instruction: &SettlementInstruction) -> PolicyDecision {
        if instruction.quantity > self.max_settlement_quantity {
            return PolicyDecision::Deny("settlement quantity exceeds policy limit".to_string());
        }

        PolicyDecision::Allow
    }
}
