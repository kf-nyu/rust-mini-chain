use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Debug)]
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

    pub fn add_block(
        &mut self,
        transactions: Vec<Transaction>,
        )  {
        let previous_block = self.chain.last().expect("A blockchain should have its genesis block.");

        let new_block = Block::new(
            previous_block.index + 1,
            transactions,
            previous_block.hash.clone(),
            self.difficulty,
        );

        self.chain.push(new_block);
    }
    // Hash validation from 2nd block to the latest block for no break.
    // Assume all block has the same difficulty.
    pub fn is_valid(&self) -> bool {
        let target = "0".repeat(self.difficulty);

        for i in 1..self.chain.len() {
            let current  = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if !current.hash.starts_with(&target) {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }

        true
    }
}
