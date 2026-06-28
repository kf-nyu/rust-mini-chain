use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustodyAccountStatus {
    Active,
    Frozen,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustodyAccount {
    pub account_id: String,
    pub owner: String,
    pub status: CustodyAccountStatus,
}

impl CustodyAccount {
    pub fn new(account_id: String, owner: String) -> Self {
        Self {
            account_id,
            owner,
            status: CustodyAccountStatus::Active,
        }
    }

    pub fn freeze(&mut self) {
        self.status = CustodyAccountStatus::Frozen;
    }

    pub fn close(&mut self) {
        self.status = CustodyAccountStatus::Closed;
    }

    pub fn is_active(&self) -> bool {
        self.status == CustodyAccountStatus::Active
    }

    pub fn is_frozen(&self) -> bool {
        self.status == CustodyAccountStatus::Frozen
    }

    pub fn is_closed(&self) -> bool {
        self.status == CustodyAccountStatus::Closed
    }
}
