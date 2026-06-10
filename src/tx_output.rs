use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutput {
    pub recipient: String,
    pub amount: u64,
}
