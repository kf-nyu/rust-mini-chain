use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub sender_public_key: String,
    // #[serde(skip)] changed from Option<Signature>
    pub signature: Option<String>,
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
        self.signature = Some(hex::encode(signature.to_bytes()));
    }

    pub fn verify(&self) -> bool {
        let Some(signature_hex) = &self.signature else {
            return false;
        };

        let Ok(signature_bytes) = hex::decode(signature_hex) else {
            return false;
        };

        let Ok(signature_array) = signature_bytes.try_into() else {
            return false;
        };

        let signature = Signature::from_bytes(&signature_array);

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
            .verify(self.message().as_bytes(), &signature)
            .is_ok()
    }
}
