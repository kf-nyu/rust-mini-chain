use std::time::Instant;
use rust_mini_chain::blockchain::Blockchain;

fn main() {
    let start = Instant::now();

    let difficulty = 4; //Added by Step 2
    let mut blockchain = Blockchain::new(difficulty);

    blockchain.add_block("Alice pays Bob 10 coins".to_string());
    blockchain.add_block("Bob pays Carol 5 coins".to_string());

    println!("{:#?}", blockchain);
    println!("Blockchain valid: {}", blockchain.is_valid());

    let elapsed = start.elapsed();
    println!("Total execution time: {:.3?}", elapsed);
}
