use sha2::{Digest, Sha256};

pub fn hash(data: &str) -> String {
    // Helper for producing a SHA-256 hash as a hex string.
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());

    hex::encode(hasher.finalize())
}

pub fn merkle_root(transactions: &[String]) -> String {
    if transactions.is_empty() {
        // Define the empty-tree Merkle root deterministically for blocks
        // that contain no transactions, such as the genesis block here.
        return hash("");
    }

    let mut hashes: Vec<String> = transactions.iter().map(|tx| hash(tx)).collect();

    while hashes.len() > 1 {
        if hashes.len() % 2 == 1 {
            // Duplicate the final hash when a level has an odd number of leaves
            // so every parent node is formed from a pair.
            let last = hashes.last().unwrap().clone();
            hashes.push(last);
        }

        // Collapse one Merkle tree level by hashing adjacent pairs together.
        hashes = hashes
            .chunks(2)
            .map(|pair| {
                //hash(&format!("{}{}", pair[0],pair[1])
                let combined = format!("{}{}", pair[0], pair[1]);
                hash(&combined)
            })
            .collect();
    }

    hashes[0].clone()
}
