use std::time::Instant;
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
    pub nonce:         u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now();

        let mut block = Self {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };

        //block.hash = block.calculate_hash();
        block.mine(difficulty);
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());

        hex::encode(hasher.finalize())
    }

    pub fn mine(&mut self, difficulty: usize) {
        let start = Instant::now();

        let target = "0".repeat(difficulty);

        loop{
            self.hash = self.calculate_hash();

            if self.hash.starts_with(&target) {
                break;
            }

            self.nonce += 1;
        }
        
        println!(
            "Block {} mined in {:.3?}, nonce = {}, hash = {}",
            self.index,
            start.elapsed(),
            self.nonce,
            self.hash
        );
    }

}
