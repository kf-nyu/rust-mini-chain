use crate::blockchain::Blockchain;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Saves and loads blockchain state from disk.
///
/// This module will be expanded in v4.0 to support JSON-based
/// persistence so a node can restart without losing its blockchain state.
pub struct Storage;

impl Storage {
    /// Placeholder for future blockchain save logic.
    pub fn new() -> Self {
        Self
    }

    /// Saves a blockchain to a JSON file.
    pub fn save_blockchain<P: AsRef<Path>>(
        &self,
        blockchain: &Blockchain,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(blockchain)?;

        let mut file = File::create(path)?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }

    /// Loads a blockchain from a JSON file.
    pub fn load_blockchain<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Blockchain, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;

        let blockchain = serde_json::from_str(&json)?;

        Ok(blockchain)
    }
}
