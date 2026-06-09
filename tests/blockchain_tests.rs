use rust_mini_chain::blockchain::Blockchain;
use rust_mini_chain::transaction::Transaction;
use rust_mini_chain::wallet::Wallet;

fn signed_transaction(from: &str, to: &str, amount: u64, wallet: &Wallet) -> Transaction {
    let mut tx = Transaction {
        from: from.to_string(),
        to: to.to_string(),
        amount,
        sender_public_key: wallet.public_key_hex(),
        signature: None,
    };

    tx.sign(&wallet.signing_key);
    tx
}

#[test]
fn valid_blockchain_passes_validation() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    let tx1 = signed_transaction("Alice", "Bob", 10, &alice);
    let tx2 = signed_transaction("Bob", "Carol", 5, &bob);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![tx1]);
    blockchain.add_block(vec![tx2]);

    assert!(blockchain.is_valid());
}

#[test]
fn tampered_transaction_fails_validation() {
    let alice = Wallet::new();

    let mut tx = signed_transaction("Alice", "Bob", 10, &alice);

    //Tempering after signed the transaction
    // Change the amount from 10 -> 1000
    tx.amount = 1000;

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![tx]);

    assert!(!blockchain.is_valid());
}

#[test]
fn tampered_previous_hash_fails_validation() {
    let alice = Wallet::new();

    let tx = signed_transaction("Alice", "Bob", 10, &alice);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![tx]);

    blockchain.chain[1].previous_hash = "bad_hash".to_string();

    assert!(!blockchain.is_valid());
}

#[test]
fn tampered_block_hash_fails_validation() {
    let alice = Wallet::new();

    let tx = signed_transaction("Alice", "Bob", 10, &alice);

    let mut blockchain = Blockchain::new(4);
    blockchain.add_block(vec![tx]);

    blockchain.chain[1].hash = "bad_hash".to_string();

    assert!(!blockchain.is_valid());
}
