use ed25519_dalek::{
    Signature,
    Signer,
    Verifier,
    SigningKey,
    VerifyingKey,
};
    use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from:   String,
    pub to:     String,
    pub amount: u64,

    #[serde(skip)]
    pub signature: Option<Signature>,
}

impl Transaction {
    fn message(&self) -> String {
        format!(
            "{}{}{}",
            self.from,
            self.to,
            self.amount,
        )
    }

    pub fn sign(
        &mut self,
        signing_key: &SigningKey,
    ) {
        let signature =
            signing_key.sign(
                self.message().as_bytes()
            );

        self.signature = Some(signature);
      }

    pub fn verify(
        &self,
        verifying_key: &VerifyingKey,
    ) -> bool {
        match &self.signature {
            Some(sig) => verifying_key
                .verify(
                    self.message().as_bytes(),
                    sig,
                )
                .is_ok(),

            None => false,
        }
    }
}
