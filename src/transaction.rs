use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub sender_public_key: String,
    #[serde(skip)]
    pub signature: Option<Signature>,
}

impl Transaction {
    fn message(&self) -> String {
        format!(
            "{}{}{}{}",
            self.from, self.to, self.amount, self.sender_public_key
        )
    }

    pub fn sign(&mut self, signing_key: &SigningKey) {
        let signature = signing_key.sign(self.message().as_bytes());
        self.signature = Some(signature);
    }

    pub fn verify(&self) -> bool {
        let Some(signature) = &self.signature else {
            return false;
        };

        let Ok(public_key_bytes) = hex::decode(&self.sender_public_key) else {
            return false;
        };

        let Ok(public_key_array) = public_key_bytes.try_into() else {
            return false;
        };

        let Ok(verifying_key) = VerifyingKey::from_bytes(&public_key_array) else {
            return false;
        };

        verifying_key
            .verify(self.message().as_bytes(), signature)
            .is_ok()
    }
}
