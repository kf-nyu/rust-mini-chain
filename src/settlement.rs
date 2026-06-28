use crate::asset::{AssetLedger, AssetTransfer};
use crate::custody::CustodyRegistry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementEngine {
    instructions: HashMap<String, SettlementInstruction>,
}
impl SettlementEngine {
    pub fn new() -> Self {
        Self {
            instructions: HashMap::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: SettlementInstruction) -> bool {
        if self.instructions.contains_key(&instruction.settlement_id) {
            return false;
        }

        self.instructions
            .insert(instruction.settlement_id.clone(), instruction);

        true
    }

    pub fn get_instruction(&self, settlement_id: &str) -> Option<&SettlementInstruction> {
        self.instructions.get(settlement_id)
    }

    pub fn instruction_count(&self) -> usize {
        self.instructions.len()
    }

    pub fn execute_settlement(&mut self, settlement_id: &str, ledger: &mut AssetLedger) -> bool {
        let instruction = match self.instructions.get_mut(settlement_id) {
            Some(instruction) => instruction,
            None => return false,
        };

        if !instruction.is_pending() {
            return false;
        }

        let transfer = AssetTransfer::new(
            instruction.asset_id.clone(),
            instruction.from.clone(),
            instruction.to.clone(),
            instruction.quantity,
        );

        if ledger.apply_transfer(&transfer) {
            instruction.mark_settled();
            true
        } else {
            instruction.mark_failed();
            false
        }
    }

    pub fn execute_pending(&mut self, ledger: &mut AssetLedger) -> usize {
        let pending_ids: Vec<String> = self
            .instructions
            .iter()
            .filter(|(_, instruction)| instruction.is_pending())
            .map(|(settlement_id, _)| settlement_id.clone())
            .collect();

        let mut settled_count = 0;

        for settlement_id in pending_ids {
            if self.execute_settlement(&settlement_id, ledger) {
                settled_count += 1;
            }
        }

        settled_count
    }

    pub fn pending_count(&self) -> usize {
        self.instructions
            .values()
            .filter(|instruction| instruction.is_pending())
            .count()
    }

    pub fn settled_count(&self) -> usize {
        self.instructions
            .values()
            .filter(|instruction| instruction.is_settled())
            .count()
    }

    pub fn failed_count(&self) -> usize {
        self.instructions
            .values()
            .filter(|instruction| instruction.is_failed())
            .count()
    }

    pub fn pending_instructions(&self) -> Vec<&SettlementInstruction> {
        self.instructions
            .values()
            .filter(|instruction| instruction.is_pending())
            .collect()
    }

    pub fn settled_instructions(&self) -> Vec<&SettlementInstruction> {
        self.instructions
            .values()
            .filter(|instruction| instruction.is_settled())
            .collect()
    }

    pub fn failed_instructions(&self) -> Vec<&SettlementInstruction> {
        self.instructions
            .values()
            .filter(|instruction| instruction.is_failed())
            .collect()
    }

    pub fn execute_settlement_with_custody(
        &mut self,
        settlement_id: &str,
        ledger: &mut AssetLedger,
        custody_registry: &CustodyRegistry,
    ) -> bool {
        let instruction = match self.instructions.get_mut(settlement_id) {
            Some(instruction) => instruction,
            None => return false,
        };

        if !instruction.is_pending() {
            return false;
        }

        let from_account = match custody_registry.get_account(&instruction.from) {
            Some(account) => account,
            None => {
                instruction.mark_failed();
                return false;
            }
        };

        let to_account = match custody_registry.get_account(&instruction.to) {
            Some(account) => account,
            None => {
                instruction.mark_failed();
                return false;
            }
        };

        if !from_account.is_active() || !to_account.is_active() {
            instruction.mark_failed();
            return false;
        }

        let transfer = AssetTransfer::new(
            instruction.asset_id.clone(),
            instruction.from.clone(),
            instruction.to.clone(),
            instruction.quantity,
        );

        if ledger.apply_transfer(&transfer) {
            instruction.mark_settled();
            true
        } else {
            instruction.mark_failed();
            false
        }
    }
}
