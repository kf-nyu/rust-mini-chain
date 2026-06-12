use ed25519_dalek::{SigningKey, VerifyingKey};

/// Local wallet holding the Ed25519 keypair used to sign transactions.
pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Wallet {
    pub fn new() -> Self {
        // This prototype generates an in-memory keypair on demand
        // rather than persisting wallet state to disk.
        let secret = rand::random::<[u8; 32]>();

        let signing_key = SigningKey::from_bytes(&secret);
        let verifying_key = signing_key.verifying_key();

        Self {
            signing_key,
            verifying_key,
        }
    }

    pub fn public_key_hex(&self) -> String {
        // The public key is hex-encoded so it can be embedded directly
        // in transactions and compared against output recipients.
        hex::encode(self.verifying_key.to_bytes())
    }
}
