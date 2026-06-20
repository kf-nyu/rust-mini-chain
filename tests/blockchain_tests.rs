use rust_mini_chain::blockchain::Blockchain;
use rust_mini_chain::mempool::Mempool;
use rust_mini_chain::storage::Storage;
use rust_mini_chain::transaction::Transaction;
use rust_mini_chain::tx_input::TxInput;
use rust_mini_chain::tx_output::TxOutput;
use rust_mini_chain::utxo::UTXOSet;
use rust_mini_chain::wallet::Wallet;
use std::fs;

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
    //let alice = Wallet::new();
    let bob = Wallet::new();

    //let coinbase = Transaction::new(
    //    vec![],
    //    vec![TxOutput {
    //        recipient: alice.public_key_hex(),
    //        amount: 50,
    //    }],
    //);
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

#[test]
fn utxo_spend_helper_creates_change_output() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let tx = Transaction::new_utxo_spend(
        coinbase.id.clone(),
        0,
        alice.public_key_hex(),
        bob.public_key_hex(),
        10,
        alice.public_key_hex(),
        40,
    );

    assert_eq!(tx.inputs.len(), 1);
    assert_eq!(tx.outputs.len(), 2);
    assert_eq!(tx.outputs[0].recipient, bob.public_key_hex());
    assert_eq!(tx.outputs[0].amount, 10);
    assert_eq!(tx.outputs[1].recipient, alice.public_key_hex());
    assert_eq!(tx.outputs[1].amount, 40);
}

#[test]
fn change_output_can_be_spent() {
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

    let mut spend1 = Transaction::new_utxo_spend(
        coinbase.id.clone(),
        0,
        alice.public_key_hex(),
        bob.public_key_hex(),
        10,
        alice.public_key_hex(),
        40,
    );

    spend1.sign(&alice.signing_key);

    let mut spend2 = Transaction::new_utxo_spend(
        spend1.id.clone(),
        1, //Alice's change output
        alice.public_key_hex(),
        carol.public_key_hex(),
        20,
        alice.public_key_hex(),
        20,
    );

    spend2.sign(&alice.signing_key);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![coinbase]);
    blockchain.add_block(vec![spend1]);
    blockchain.add_block(vec![spend2]);

    assert!(blockchain.is_valid());
}

#[test]
fn reused_change_output_is_rejected() {
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

    let mut spend1 = Transaction::new_utxo_spend(
        coinbase.id.clone(),
        0,
        alice.public_key_hex(),
        bob.public_key_hex(),
        10,
        alice.public_key_hex(),
        40,
    );

    spend1.sign(&alice.signing_key);

    let mut spend2 = Transaction::new_utxo_spend(
        spend1.id.clone(),
        1,
        alice.public_key_hex(),
        carol.public_key_hex(),
        20,
        alice.public_key_hex(),
        20,
    );

    spend2.sign(&alice.signing_key);

    let mut spend3 = Transaction::new_utxo_spend(
        spend1.id.clone(),
        1,
        alice.public_key_hex(),
        carol.public_key_hex(),
        5,
        alice.public_key_hex(),
        35,
    );

    spend3.sign(&alice.signing_key);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![coinbase]);
    blockchain.add_block(vec![spend1]);
    blockchain.add_block(vec![spend2]);
    blockchain.add_block(vec![spend3]);

    assert!(!blockchain.is_valid());
}

#[test]
fn utxo_balance_updates_after_spend() {
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

    assert_eq!(utxo_set.balance_of(&alice.public_key_hex()), 50);

    let mut spend = Transaction::new_utxo_spend(
        coinbase.id.clone(),
        0,
        alice.public_key_hex(),
        bob.public_key_hex(),
        10,
        alice.public_key_hex(),
        40,
    );

    spend.sign(&alice.signing_key);

    utxo_set.add_transaction(&spend);

    assert_eq!(utxo_set.balance_of(&bob.public_key_hex()), 10);

    assert_eq!(utxo_set.balance_of(&alice.public_key_hex()), 40);
}

#[test]
fn received_chain_validation_passes() {
    let blockchain = Blockchain::new(2);

    assert!(blockchain.is_valid());
}

#[test]
fn replaces_with_valid_longer_chain() {
    let mut local_chain = Blockchain::new(2);

    let alice = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut candidate_chain = Blockchain::new(2);
    candidate_chain.add_block(vec![coinbase]);

    assert!(candidate_chain.chain.len() > local_chain.chain.len());

    let replaced = local_chain.replace_chain_if_longer(candidate_chain);

    assert!(replaced);
    assert_eq!(local_chain.chain.len(), 2);
}

#[test]
fn rejects_chain_that_is_not_longer() {
    let mut local_chain = Blockchain::new(2);
    let candidate_chain = Blockchain::new(2);

    let replaced = local_chain.replace_chain_if_longer(candidate_chain);

    assert!(!replaced);
    assert_eq!(local_chain.chain.len(), 1);
}

#[test]
fn rejects_invalid_longer_chain() {
    let mut local_chain = Blockchain::new(2);

    let alice = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut candidate_chain = Blockchain::new(2);
    candidate_chain.add_block(vec![coinbase]);

    candidate_chain.chain[1].hash = "bad_hash".to_string();

    let replaced = local_chain.replace_chain_if_longer(candidate_chain);

    assert!(!replaced);
    assert_eq!(local_chain.chain.len(), 1);
}

#[test]
fn chain_sync_accepts_valid_longer_chain() {
    let mut local_chain = Blockchain::new(2);

    let alice = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut peer_chain = Blockchain::new(2);

    peer_chain.add_block(vec![coinbase]);

    assert!(local_chain.replace_chain_if_longer(peer_chain));

    assert_eq!(local_chain.chain.len(), 2);
}

#[test]
fn chain_sync_rejects_shorter_chain() {
    let alice = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut local_chain = Blockchain::new(2);

    local_chain.add_block(vec![coinbase]);

    let peer_chain = Blockchain::new(2);

    assert!(!local_chain.replace_chain_if_longer(peer_chain));

    assert_eq!(local_chain.chain.len(), 2);
}

#[test]
fn chain_sync_rejects_invalid_chain() {
    let alice = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut peer_chain = Blockchain::new(2);

    peer_chain.add_block(vec![coinbase]);

    peer_chain.chain[1].hash = "invalid".to_string();

    let mut local_chain = Blockchain::new(2);

    assert!(!local_chain.replace_chain_if_longer(peer_chain));

    assert_eq!(local_chain.chain.len(), 1);
}

#[test]
fn saved_blockchain_file_is_created() {
    let blockchain = Blockchain::new(2);
    let storage = Storage::new();
    let path = "test_chain_created.json";

    storage.save_blockchain(&blockchain, path).unwrap();

    assert!(fs::metadata(path).is_ok());

    fs::remove_file(path).unwrap();
}

#[test]
fn saved_blockchain_can_be_loaded() {
    let blockchain = Blockchain::new(2);
    let storage = Storage::new();
    let path = "test_chain_loaded.json";

    storage.save_blockchain(&blockchain, path).unwrap();

    let loaded = storage.load_blockchain(path).unwrap();

    assert_eq!(loaded.chain.len(), blockchain.chain.len());
    assert_eq!(loaded.difficulty, blockchain.difficulty);

    fs::remove_file(path).unwrap();
}

#[test]
fn loaded_blockchain_passes_validation() {
    let blockchain = Blockchain::new(2);
    let storage = Storage::new();
    let path = "test_chain_loaded.json";

    storage.save_blockchain(&blockchain, path).unwrap();

    let loaded = storage.load_blockchain(path).unwrap();

    assert!(loaded.is_valid());

    fs::remove_file(path).unwrap();
}

#[test]
fn mempool_accepts_transaction() {
    let mut mempool = Mempool::new();

    let alice = Wallet::new();
    let bob = Wallet::new();

    let transaction = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 10,
        }],
    );

    mempool.add_transaction(transaction);

    assert_eq!(mempool.len(), 1);
    assert!(!mempool.is_empty());
}
