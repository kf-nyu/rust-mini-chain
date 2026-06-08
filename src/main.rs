use std::time::Instant;
use rust_mini_chain::blockchain::Blockchain;
use rust_mini_chain::transaction::Transaction;

fn main() {
    let start = Instant::now();

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
