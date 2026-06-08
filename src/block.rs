use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index:         u64,
    pub timestamp:     DateTime<Utc>,
    pub data:          String,
    pub previous_hash: String,
    pub hash:          String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now();

        let mut block = Self {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash
        );

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());

        hex::encode(hasher.finalize())
    }
}
