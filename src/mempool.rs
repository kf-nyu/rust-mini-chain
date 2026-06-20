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

    /// Adds a transaction to the mempool.
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
}
