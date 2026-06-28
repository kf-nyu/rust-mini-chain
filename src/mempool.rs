use crate::transaction::Transaction;

/// In-memory pool of pending transactions waiting to be mined.
#[derive(Debug, Clone, Default)]
pub struct Mempool {
    transactions: Vec<Transaction>,
}

impl Mempool {
    /// Creates an empty mempool.
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    /// Returns the number of pending transaction
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Returns true if the mempool has no pending transactions.
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Adds a transaction to the mempool if it passes basic signature validation
    /// and is not already present.
    pub fn add_transaction(&mut self, transaction: Transaction) -> bool {
        if !transaction.verify() {
            return false;
        }

        if self
            .transactions
            .iter()
            .any(|existing| existing.id == transaction.id)
        {
            return false;
        }

        self.transactions.push(transaction);

        true
    }

    /// Returns all pending transactions.
    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    /// Returns up to max_transactions pending transactions.
    pub fn select_transactions(&self, max_transactions: usize) -> Vec<Transaction> {
        self.transactions
            .iter()
            .take(max_transactions)
            .cloned()
            .collect()
    }

    /// Removes transactions from the mempool by transaction id.
    pub fn remove_transactions(&mut self, mined_transactions: &[Transaction]) {
        let mined_ids: Vec<String> = mined_transactions
            .iter()
            .map(|transaction| transaction.id.clone())
            .collect();

        self.transactions
            .retain(|transaction| !mined_ids.contains(&transaction.id));
    }
}
