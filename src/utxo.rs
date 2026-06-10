use crate::transaction::Transaction;
use crate::tx_output::TxOutput;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UTXOSet {
    pub outputs: HashMap<String, TxOutput>,
}

impl UTXOSet {
    pub fn new() -> Self {
        Self {
            outputs: HashMap::new(),
        }
    }

    fn key(tx_id: &str, output_index: usize) -> String {
        format!("{}:{}", tx_id, output_index)
    }

    pub fn add_transaction(&mut self, transaction: &Transaction) {
        for (index, output) in transaction.outputs.iter().enumerate() {
            let key = Self::key(&transaction.id, index);
            self.outputs.insert(key, output.clone());
        }

        for input in &transaction.inputs {
            let key = Self::key(&input.previous_tx_id, input.output_index);
            self.outputs.remove(&key);
        }
    }

    pub fn find_output(
        &self,
        tx_id: &str,
        output_index: usize,
    ) -> Option<&TxOutput> {
        let key = Self::key(tx_id, output_index);
        self.outputs.get(&key)
    }

    pub fn contains(
        &self,
        tx_id: &str,
        output_index: usize,
    ) -> bool {
        self.find_output(tx_id, output_index).is_some()
    }

    pub fn balance_of(&self, owner: &str) -> u64 {
        self.outputs
            .values()
            .filter(|output| output.recipient == owner)
            .map(|output| output.amount)
            .sum()
    }
}

