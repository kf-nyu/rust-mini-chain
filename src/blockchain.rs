use crate::block::Block;
use crate::transaction::Transaction;
use crate::utxo::UTXOSet;
use serde::{Deserialize, Serialize};

/// Simplified blockchain containing proof-of-work blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        Self {
            chain: vec![Self::genesis_block(difficulty)],
            difficulty,
        }
    }

    fn genesis_block(difficulty: usize) -> Block {
        Block::new(
            0,
            vec![],
            // "Genesis Block".to_string(),
            "0".to_string(),
            difficulty,
        )
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self
            .chain
            .last()
            .expect("A blockchain should have its genesis block.");

        let new_block = Block::new(
            previous_block.index + 1,
            transactions,
            previous_block.hash.clone(),
            self.difficulty,
        );

        self.chain.push(new_block);
    }
    pub fn is_valid(&self) -> bool {
        // Rebuild UTXO state from genesis forward while validating
        // every block and transaction in chain order.
        if self.chain.is_empty() {
            return false;
        }

        let target = "0".repeat(self.difficulty);
        // Tracks currently spendable outputs as validation progresses.
        let mut utxo_set = UTXOSet::new();

        for i in 0..self.chain.len() {
            let current = &self.chain[i];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if !current.hash.starts_with(&target) {
                return false;
            }

            if i > 0 {
                let previous = &self.chain[i - 1];

                if current.previous_hash != previous.hash {
                    return false;
                }
            }

            // Validate transactions against the spendable output state built
            // from all earlier blocks before applying their effects.
            for transaction in &current.transactions {
                if !utxo_set.validate_transaction(transaction) {
                    return false;
                }

                utxo_set.add_transaction(transaction);
            }
        }

        true
    }

    pub fn is_acceptable(&self) -> bool {
        self.is_valid()
    }

    /// Synchronizes the local chain with a peer chain.
    /// A valid longer chain replaces the current chain.
    pub fn replace_chain_if_longer(&mut self, candidate: Blockchain) -> bool {
        if !candidate.is_valid() {
            return false;
        }

        if candidate.chain.len() <= self.chain.len() {
            return false;
        }

        self.chain = candidate.chain;
        self.difficulty = candidate.difficulty;

        true
    }
}
