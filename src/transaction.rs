use crate::tx_input::TxInput;
use crate::tx_output::TxOutput;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// UTXO-style transaction that spends previous outputs into new outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    //pub from: String,
    //pub to: String,
    //pub amount: u64,
    //pub sender_public_key: String,
    // #[serde(skip)] changed from Option<Signature>
    //pub signature: Option<String>,
    // v2.0:UTXO
    pub id: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl Transaction {
    pub fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        let mut transaction = Self {
            id: String::new(),
            inputs,
            outputs,
        };

        transaction.id = transaction.calculate_hash();
        transaction
    }

    pub fn calculate_hash(&self) -> String {
        let input_data: Vec<String> = self
            .inputs
            .iter()
            .map(|input| {
                format!(
                    "{}{}{}",
                    input.previous_tx_id, input.output_index, input.sender_public_key
                )
            })
            .collect();

        let output_data: Vec<String> = self
            .outputs
            .iter()
            .map(|output| format!("{}{}", output.recipient, output.amount))
            .collect();

        let data = format!("{:?}{:?}", input_data, output_data);

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());

        hex::encode(hasher.finalize())
    }

    fn message(&self) -> String {
        //format!(
        //    "{}{}{}{}",
        //    self.from, self.to, self.amount, self.sender_public_key
        //)
        // Signatures cover the transaction contents rather than a stored id,
        // so verification stays tied to the current inputs and outputs.
        self.calculate_hash()
    }

    pub fn sign(&mut self, signing_key: &SigningKey) {
        // Sign the transaction message using Ed25519.
        // The signature is attached to each transaction input.
        let signature = signing_key.sign(self.message().as_bytes());
        let signature_hex = hex::encode(signature.to_bytes());

        for input in &mut self.inputs {
            input.signature = Some(signature_hex.clone());
        }

        self.id = self.calculate_hash();
    }

    pub fn verify(&self) -> bool {
        // Coinbase-style transactions have no inputs.
        // This prototype accepts them as value-creating transactions
        // until coinbase rules are modeled more explicitly.
        if self.inputs.is_empty() {
            return true;
        }

        // Verify that each input signature matches
        // the current transaction contents.
        for input in &self.inputs {
            let Some(signature_hex) = &input.signature else {
                return false;
            };

            let Ok(signature_bytes) = hex::decode(signature_hex) else {
                return false;
            };

            let Ok(signature_array) = signature_bytes.try_into() else {
                return false;
            };

            let signature = Signature::from_bytes(&signature_array);

            let Ok(public_key_bytes) = hex::decode(&input.sender_public_key) else {
                return false;
            };

            let Ok(public_key_array) = public_key_bytes.try_into() else {
                return false;
            };

            let Ok(verifying_key) = VerifyingKey::from_bytes(&public_key_array) else {
                return false;
            };

            if verifying_key
                .verify(self.message().as_bytes(), &signature)
                .is_err()
            {
                return false;
            }
        }

        true
    }

    pub fn new_utxo_spend(
        previous_tx_id: String,
        output_index: usize,
        sender_public_key: String,
        recipient: String,
        amount: u64,
        change_recipient: String,
        change_amount: u64,
    ) -> Self {
        // Helper for creating a spend transaction with a change output.
        let mut outputs = vec![TxOutput { recipient, amount }];

        if change_amount > 0 {
            outputs.push(TxOutput {
                recipient: change_recipient,
                amount: change_amount,
            });
        }

        Self::new(
            vec![TxInput {
                previous_tx_id,
                output_index,
                sender_public_key,
                signature: None,
            }],
            outputs,
        )
    }
}
