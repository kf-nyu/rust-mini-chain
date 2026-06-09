use std::time::Instant;
use rust_mini_chain::blockchain::Blockchain;
use rust_mini_chain::transaction::Transaction;
use rust_mini_chain::wallet::Wallet;

fn main() {
    let start = Instant::now();

    let alice = Wallet::new();
    let bob   = Wallet::new();

#[cfg(debug_assertions)]
{
    println!(
        "Alice pubkey: {:?}", alice.verifying_key
    );
    
    println!(
        "Bob pubkey: {:?}", bob.verifying_key
    );
}    
    println!("Alice Hex pubkey: {}", alice.public_key_hex());
    println!("Bob Hex pubkey  : {}", bob.public_key_hex());

    let difficulty = 4; //Added by Step 2
    let mut blockchain = Blockchain::new(difficulty);

    let tx1 = Transaction {
        from: "Alice".to_string(),
        to:     "Bob".to_string(),
        amount: 10,
    };

    let tx2 = Transaction {
        from: "Bob".to_string(),
        to: "Carol".to_string(),
        amount: 5,
    };

    blockchain.add_block(vec![tx1]);
    blockchain.add_block(vec![tx2]);

    println!("{:#?}", blockchain);
    println!("Blockchain valid: {}", blockchain.is_valid());

    let elapsed = start.elapsed();
    println!("Total execution time: {:.3?}", elapsed);
}
