use crate::merkle;
use crate::transaction::Transaction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::Instant;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub merkle_root: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        difficulty: usize,
    ) -> Self {
        let timestamp = Utc::now();

        let tx_strings: Vec<String> = transactions
            .iter()
            .map(|tx| serde_json::to_string(tx).unwrap())
            .collect();

        let merkle_root = merkle::merkle_root(&tx_strings);

        let mut block = Self {
            index,
            timestamp,
            transactions,
            merkle_root,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };

        //block.hash = block.calculate_hash();
        block.mine(difficulty);
        block
    }

    pub fn calculate_hash(&self) -> String {
        //let tx_json =
        //    serde_json::to_string(&self.transactions).unwrap();

        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.merkle_root, self.previous_hash, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());

        hex::encode(hasher.finalize())
    }

    pub fn mine(&mut self, difficulty: usize) {
        let start = Instant::now();

        let target = "0".repeat(difficulty);

        loop {
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

    pub fn is_valid(&self, difficulty: usize) -> bool {
        let target = "0".repeat(difficulty);

        if self.hash != self.calculate_hash() {
            return false;
        }

        if !self.hash.starts_with(&target) {
            return false;
        }

        for transaction in &self.transactions {
            if !transaction.verify() {
                return false;
            }
        }

        true
    }
}
