use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeRole {
    Validator,
    Observer,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeIdentity {
    pub node_id: String,
    pub role: NodeRole,
}

impl NodeIdentity {
    pub fn new(node_id: String, role: NodeRole) -> Self {
        Self { node_id, role }
    }

    pub fn is_validator(&self) -> bool {
        self.role == NodeRole::Validator
    }
}
