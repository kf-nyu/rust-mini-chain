use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInput {
    pub previous_tx_id: String,
    pub output_index: usize,
    pub sender_public_key: String,
    pub signature: Option<String>,
}
