use serde::{Deserialize, Serialize};

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
