use crate::block::Block;
use crate::blockchain::Blockchain;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    Block(Block),
    ChainRequest,
    ChainResponse(Blockchain),
}
