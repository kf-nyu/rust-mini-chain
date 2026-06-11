use rust_mini_chain::blockchain::Blockchain;
use rust_mini_chain::transaction::Transaction;
use rust_mini_chain::tx_input::TxInput;
use rust_mini_chain::tx_output::TxOutput;
use rust_mini_chain::utxo::UTXOSet;
use rust_mini_chain::wallet::Wallet;

fn signed_transaction(
    previous_tx_id: &str,
    output_index: usize,
    sender: &Wallet,
    recipient: &Wallet,
    amount: u64,
) -> Transaction {
    let mut tx = Transaction::new(
        vec![TxInput {
            previous_tx_id: previous_tx_id.to_string(),
            output_index,
            sender_public_key: sender.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: recipient.public_key_hex(),
            amount,
        }],
    );

    tx.sign(&sender.signing_key);
    tx
}

#[test]
fn valid_blockchain_passes_validation() {
    let alice = Wallet::new();
    let bob = Wallet::new();
    let carol = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut spend1 = Transaction::new(
        vec![TxInput {
            previous_tx_id: coinbase.id.clone(),
            output_index: 0,
            sender_public_key: alice.public_key_hex(),
            signature: None,
        }],
        vec![
            TxOutput {
                recipient: bob.public_key_hex(),
                amount: 10,
            },
            TxOutput {
                recipient: alice.public_key_hex(),
                amount: 40,
            },
        ],
    );

    spend1.sign(&alice.signing_key);

    let mut spend2 = Transaction::new(
        vec![TxInput {
            previous_tx_id: coinbase.id.clone(),
            output_index: 0,
            sender_public_key: bob.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: carol.public_key_hex(),
            amount: 5,
        }],
    );

    spend2.sign(&alice.signing_key);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![coinbase]);
    blockchain.add_block(vec![spend1]);
    blockchain.add_block(vec![spend2]);

    assert!(!blockchain.is_valid());
}

#[test]
fn tampered_transaction_fails_validation() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut tx = signed_transaction(&coinbase.id, 0, &alice, &bob, 10);

    tx.outputs[0].amount = 1000;

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![coinbase]);
    blockchain.add_block(vec![tx]);

    assert!(!blockchain.is_valid());
}

#[test]
fn tampered_previous_hash_fails_validation() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let tx = signed_transaction(&coinbase.id, 0, &alice, &bob, 10);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![coinbase]);
    blockchain.add_block(vec![tx]);

    blockchain.chain[1].previous_hash = "bad_hash".to_string();

    assert!(!blockchain.is_valid());
}

#[test]
fn tampered_block_hash_fails_validation() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );
    let tx = signed_transaction(&coinbase.id, 0, &alice, &bob, 10);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![coinbase]);
    blockchain.add_block(vec![tx]);

    blockchain.chain[1].hash = "bad_hash".to_string();

    assert!(!blockchain.is_valid());
}

#[test]
fn utxo_set_tracks_unspent_outputs() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );
    let tx = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 10,
        }],
    );

    let mut utxo_set = UTXOSet::new();
    utxo_set.add_transaction(&tx);

    assert!(utxo_set.contains(&tx.id, 0));
    assert_eq!(utxo_set.balance_of(&bob.public_key_hex()), 10);
}

#[test]
fn utxo_validation_accepts_valid_spend() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut utxo_set = UTXOSet::new();
    utxo_set.add_transaction(&coinbase);

    let mut spend = Transaction::new(
        vec![TxInput {
            previous_tx_id: coinbase.id.clone(),
            output_index: 0,
            sender_public_key: alice.public_key_hex(),
            signature: None,
        }],
        vec![
            TxOutput {
                recipient: bob.public_key_hex(),
                amount: 10,
            },
            TxOutput {
                recipient: alice.public_key_hex(),
                amount: 40,
            },
        ],
    );

    spend.sign(&alice.signing_key);

    assert!(utxo_set.validate_transaction(&spend));

    utxo_set.add_transaction(&spend);

    assert_eq!(utxo_set.balance_of(&bob.public_key_hex()), 10);
    assert_eq!(utxo_set.balance_of(&alice.public_key_hex()), 40);
}

#[test]
fn utxo_validation_rejects_nonexistent_input() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let mut spend = Transaction::new(
        vec![TxInput {
            previous_tx_id: "missing".to_string(),
            output_index: 0,
            sender_public_key: alice.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 10,
        }],
    );

    spend.sign(&alice.signing_key);

    let utxo_set = UTXOSet::new();

    assert!(!utxo_set.validate_transaction(&spend));
}

#[test]
fn utxo_validation_rejects_overspending() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut utxo_set = UTXOSet::new();
    utxo_set.add_transaction(&coinbase);

    let mut spend = Transaction::new(
        vec![TxInput {
            previous_tx_id: coinbase.id.clone(),
            output_index: 0,
            sender_public_key: alice.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 100,
        }],
    );

    spend.sign(&alice.signing_key);

    assert!(!utxo_set.validate_transaction(&spend));
}

#[test]
fn utxo_validation_rejects_wrong_owner() {
    let alice = Wallet::new();
    let bob = Wallet::new();
    let mallory = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut utxo_set = UTXOSet::new();
    utxo_set.add_transaction(&coinbase);

    let mut spend = Transaction::new(
        vec![TxInput {
            previous_tx_id: coinbase.id.clone(),
            output_index: 0,
            sender_public_key: mallory.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 10,
        }],
    );

    spend.sign(&mallory.signing_key);

    assert!(!utxo_set.validate_transaction(&spend));
}

#[test]
fn blockchain_rejects_double_spend() {
    let alice = Wallet::new();
    let bob = Wallet::new();
    let carol = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut spend1 = Transaction::new(
        vec![TxInput {
            previous_tx_id: coinbase.id.clone(),
            output_index: 0,
            sender_public_key: alice.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 50,
        }],
    );

    spend1.sign(&alice.signing_key);

    let mut spend2 = Transaction::new(
        vec![TxInput {
            previous_tx_id: coinbase.id.clone(),
            output_index: 0,
            sender_public_key: alice.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: carol.public_key_hex(),
            amount: 50,
        }],
    );

    spend2.sign(&alice.signing_key);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![coinbase]);
    blockchain.add_block(vec![spend1]);
    blockchain.add_block(vec![spend2]);

    assert!(!blockchain.is_valid());
}
