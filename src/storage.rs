use crate::blockchain::Blockchain;

/// Saves and loads blockchain state from disk.
///
/// This module will be expanded in v4.0 to support JSON-based
/// persistence so a node can restart without losing its chain.
pub struct Storage;

impl Storage {
    /// Placeholder for future blockchain save logic.
    pub fn new() -> Self {
        Self
    }
}
