use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustodyRegistry {
    accounts: HashMap<String, CustodyAccount>,
}

impl CustodyRegistry {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn add_account(&mut self, account: CustodyAccount) -> bool {
        if self.accounts.contains_key(&account.account_id) {
            return false;
        }

        self.accounts.insert(account.account_id.clone(), account);

        true
    }

    pub fn get_account(&self, account_id: &str) -> Option<&CustodyAccount> {
        self.accounts.get(account_id)
    }

    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }

    pub fn freeze_account(&mut self, account_id: &str) -> bool {
        match self.accounts.get_mut(account_id) {
            Some(account) => {
                account.freeze();
                true
            }
            None => false,
        }
    }

    pub fn close_account(&mut self, account_id: &str) -> bool {
        match self.accounts.get_mut(account_id) {
            Some(account) => {
                account.close();
                true
            }
            None => false,
        }
    }

    pub fn active_accounts(&self) -> Vec<&CustodyAccount> {
        self.accounts
            .values()
            .filter(|account| account.is_active())
            .collect()
    }

    pub fn frozen_accounts(&self) -> Vec<&CustodyAccount> {
        self.accounts
            .values()
            .filter(|account| account.is_frozen())
            .collect()
    }

    pub fn closed_accounts(&self) -> Vec<&CustodyAccount> {
        self.accounts
            .values()
            .filter(|account| account.is_closed())
            .collect()
    }

    pub fn active_count(&self) -> usize {
        self.active_accounts().len()
    }

    pub fn frozen_count(&self) -> usize {
        self.frozen_accounts().len()
    }

    pub fn closed_count(&self) -> usize {
        self.closed_accounts().len()
    }
}
