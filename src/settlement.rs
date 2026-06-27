use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettlementStatus {
    Pending,
    Settled,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementInstruction {
    pub settlement_id: String,
    pub asset_id: String,
    pub from: String,
    pub to: String,
    pub quantity: u64,
    pub status: SettlementStatus,
}

impl SettlementInstruction {
    pub fn new(
        settlement_id: String,
        asset_id: String,
        from: String,
        to: String,
        quantity: u64,
    ) -> Self {
        Self {
            settlement_id,
            asset_id,
            from,
            to,
            quantity,
            status: SettlementStatus::Pending,
        }
    }

    pub fn mark_settled(&mut self) {
        self.status = SettlementStatus::Settled;
    }

    pub fn mark_failed(&mut self) {
        self.status = SettlementStatus::Failed;
    }

    pub fn is_pending(&self) -> bool {
        self.status == SettlementStatus::Pending
    }

    pub fn is_settled(&self) -> bool {
        self.status == SettlementStatus::Settled
    }

    pub fn is_failed(&self) -> bool {
        self.status == SettlementStatus::Failed
    }
}
