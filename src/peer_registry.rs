use crate::node_identity::NodeIdentity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerRegistry {
    trusted_peers: Vec<NodeIdentity>,
}

impl PeerRegistry {
    pub fn new() -> Self {
        Self {
            trusted_peers: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, peer: NodeIdentity) -> bool {
        if self.is_trusted(&peer.node_id) {
            return false;
        }

        self.trusted_peers.push(peer);
        true
    }

    pub fn is_trusted(&self, node_id: &str) -> bool {
        self.trusted_peers
            .iter()
            .any(|peer| peer.node_id == node_id)
    }

    pub fn len(&self) -> usize {
        self.trusted_peers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.trusted_peers.is_empty()
    }
}
