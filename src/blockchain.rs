use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: vec![Self::genesis_block()],
        }
    }

    fn genesis_block() -> Block {
        Block::new(
            0,
            "Genesis Block".to_string(),
            "0".to_string(),
        )
    }

    pub fn add_block(&mut self, data: String)  {
        let previous_block = self.chain.last().expect("A blockchain should have its genesis block.");

        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
        );

        self.chain.push(new_block);
    }
    // Hash validation from 2nd block to the latest block for no break.
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current  = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }

        true
    }
}
