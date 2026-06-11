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
            #[cfg(debug_assertions)]
            println!("ADD UTXO; {key}");
            self.outputs.insert(key, output.clone());
        }

        for input in &transaction.inputs {
            let key = Self::key(&input.previous_tx_id, input.output_index);
            #[cfg(debug_assertions)]
            println!("REMOVE UTXO; {key}");
            self.outputs.remove(&key);
        }
    }

    pub fn find_output(&self, tx_id: &str, output_index: usize) -> Option<&TxOutput> {
        let key = Self::key(tx_id, output_index);
        self.outputs.get(&key)
    }

    pub fn contains(&self, tx_id: &str, output_index: usize) -> bool {
        self.find_output(tx_id, output_index).is_some()
    }

    pub fn balance_of(&self, owner: &str) -> u64 {
        self.outputs
            .values()
            .filter(|output| output.recipient == owner)
            .map(|output| output.amount)
            .sum()
    }

    pub fn validate_transaction(&self, transaction: &Transaction) -> bool {
        //Coinbase-style transaction: creates initial coins.
        //We keep this simple for the prototype.
        if transaction.inputs.is_empty() {
            return true;
        }

        if !transaction.verify() {
            return false;
        }

        let mut input_total = 0;

        for input in &transaction.inputs {
            #[cfg(debug_assertions)]
            {
                let key = Self::key(&input.previous_tx_id, input.output_index);
                println!("CHECK UTXO: {key}");
            }
            let Some(output) = self.find_output(&input.previous_tx_id, input.output_index) else {
                return false;
            };

            if output.recipient != input.sender_public_key {
                #[cfg(debug_assertions)]
                println!("WRONG OWNER");
                return false;
            }

            input_total += output.amount;
        }

        let output_total: u64 = transaction.outputs.iter().map(|output| output.amount).sum();

        input_total >= output_total
    }
}
