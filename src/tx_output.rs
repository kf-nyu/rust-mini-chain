use serde::{Deserialize, Serialize};

/// Spendable transaction output tracked by the UTXO set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutput {
    pub recipient: String,
    pub amount: u64,
}
