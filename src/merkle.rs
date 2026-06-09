use sha2::{Digest, Sha256};

pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());

    hex::encode(hasher.finalize())
}

pub fn merkle_root(transactions: &[String]) -> String {
    if transactions.is_empty() {
        return hash("");
    }

    let mut hashes: Vec<String> = transactions.iter().map(|tx| hash(tx)).collect();

    while hashes.len() > 1 {
        if hashes.len() % 2 == 1 {
            let last = hashes.last().unwrap().clone();
            hashes.push(last);
        }

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
