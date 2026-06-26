use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::node_identity::NodeIdentity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    Hello(NodeIdentity),
    Block(Block),
    ChainRequest,
    ChainResponse(Blockchain),
}
