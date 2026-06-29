use crate::settlement::SettlementInstruction;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny(String),
}

#[derive(Debug)]
pub struct PolicyEngine {
    max_settlement_quantity: u64,
    blocked_accounts: HashSet<String>,
}

impl PolicyEngine {
    pub fn new(max_settlement_quantity: u64) -> Self {
        Self {
            max_settlement_quantity,
            blocked_accounts: HashSet::new(),
        }
    }

    pub fn block_account(&mut self, account_id: String) -> bool {
        self.blocked_accounts.insert(account_id)
    }

    pub fn is_account_blocked(&self, account_id: &str) -> bool {
        self.blocked_accounts.contains(account_id)
    }

    pub fn evaluate_settlement(&self, instruction: &SettlementInstruction) -> PolicyDecision {
        if self.is_account_blocked(&instruction.from) {
            return PolicyDecision::Deny("sender custody account is blocked".to_string());
        }

        if self.is_account_blocked(&instruction.to) {
            return PolicyDecision::Deny("receiver custody account is blocked".to_string());
        }

        if instruction.quantity > self.max_settlement_quantity {
            return PolicyDecision::Deny("settlement quantity exceeds policy limit".to_string());
        }

        PolicyDecision::Allow
    }
}
