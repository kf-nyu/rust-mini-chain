use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    Fungible,
    NonFungible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asset {
    pub asset_id: String,
    pub name: String,
    pub symbol: String,
    pub asset_type: AssetType,
    pub total_supply: u64,
}

impl Asset {
    pub fn new(
        asset_id: String,
        name: String,
        symbol: String,
        asset_type: AssetType,
        total_supply: u64,
    ) -> Self {
        Self {
            asset_id,
            name,
            symbol,
            asset_type,
            total_supply,
        }
    }

    pub fn is_fungible(&self) -> bool {
        self.asset_type == AssetType::Fungible
    }

    pub fn is_non_fungible(&self) -> bool {
        self.asset_type == AssetType::NonFungible
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetIssuance {
    pub asset: Asset,
    pub issuer: String,
}

impl AssetIssuance {
    pub fn new(asset: Asset, issuer: String) -> Self {
        Self { asset, issuer }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetOwnership {
    pub asset_id: String,
    pub owner: String,
    pub quantity: u64,
}

impl AssetOwnership {
    pub fn new(asset_id: String, owner: String, quantity: u64) -> Self {
        Self {
            asset_id,
            owner,
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetTransfer {
    pub asset_id: String,
    pub from: String,
    pub to: String,
    pub quantity: u64,
}

impl AssetTransfer {
    pub fn new(asset_id: String, from: String, to: String, quantity: u64) -> Self {
        Self {
            asset_id,
            from,
            to,
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetLedger {
    pub balances: HashMap<String, u64>,
}

impl AssetLedger {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    fn balance_key(asset_id: &str, owner: &str) -> String {
        format!("{asset_id}:{owner}")
    }

    pub fn credit(&mut self, asset_id: &str, owner: &str, quantity: u64) {
        let key = Self::balance_key(asset_id, owner);
        let balance = self.balances.entry(key).or_insert(0);
        *balance += quantity;
    }

    pub fn balance_of(&self, asset_id: &str, owner: &str) -> u64 {
        let key = Self::balance_key(asset_id, owner);
        *self.balances.get(&key).unwrap_or(&0)
    }

    pub fn apply_transfer(&mut self, transfer: &AssetTransfer) -> bool {
        let sender_balance = self.balance_of(&transfer.asset_id, &transfer.from);

        if sender_balance < transfer.quantity {
            return false;
        }

        let sender_key = Self::balance_key(&transfer.asset_id, &transfer.from);

        let receiver_key = Self::balance_key(&transfer.asset_id, &transfer.to);

        *self.balances.entry(sender_key).or_insert(0) -= transfer.quantity;
        *self.balances.entry(receiver_key).or_insert(0) += transfer.quantity;

        true
    }
}
