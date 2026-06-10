use rust_mini_chain::blockchain::Blockchain;
use rust_mini_chain::transaction::Transaction;
use rust_mini_chain::wallet::Wallet;
use rust_mini_chain::tx_input::TxInput;
use rust_mini_chain::tx_output::TxOutput;
use rust_mini_chain::utxo::UTXOSet;



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

    let tx1 = signed_transaction("genesis", 0, &alice, &bob, 10);
    let tx2 = signed_transaction(&tx1.id, 0, &bob, &carol, 5);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![tx1]);
    blockchain.add_block(vec![tx2]);

    assert!(blockchain.is_valid());
}

#[test]
fn tampered_transaction_fails_validation() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let mut tx = signed_transaction("genesis", 0, &alice, &bob, 10);

    //Tempering after signed the transaction
    // Change the amount from 10 -> 1000
    tx.outputs[0].amount = 1000;

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![tx]);

    assert!(!blockchain.is_valid());
}

#[test]
fn tampered_previous_hash_fails_validation() {
    let alice = Wallet::new();
    let bob  = Wallet::new();

    let tx = signed_transaction("genesis", 0, &alice, &bob, 10);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![tx]);

    blockchain.chain[1].previous_hash = "bad_hash".to_string();

    assert!(!blockchain.is_valid());
}

#[test]
fn tampered_block_hash_fails_validation() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let tx = signed_transaction("genesis", 0, &alice, &bob, 10);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![tx]);

    blockchain.chain[1].hash = "bad_hash".to_string();

    assert!(!blockchain.is_valid());
}

#[test]
fn utxo_set_tracks_unspent_outputs() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let tx = signed_transaction("genesis", 0, &alice, &bob, 10);

    let mut utxo_set = UTXOSet::new();
    utxo_set.add_transaction(&tx);

    assert!(utxo_set.contains(&tx.id, 0));
    assert_eq!(utxo_set.balance_of(&bob.public_key_hex()), 10);
}
