use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from:   String,
    pub to:     String,
    pub amount: u64,
}
