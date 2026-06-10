use rust_mini_chain::blockchain::Blockchain;
use rust_mini_chain::network;
use rust_mini_chain::transaction::Transaction;
use rust_mini_chain::wallet::Wallet;
use rust_mini_chain::tx_input::TxInput;
use rust_mini_chain::tx_output::TxOutput;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 3 && args[1] == "node" {
        let port = args[2].parse::<u16>().unwrap();

        network::start_node(port);

        return;
    }

    if args.len() >= 3 && args[1] == "send" {
        let mut blockchain = Blockchain::new(4);

        let alice = Wallet::new();
        let bob = Wallet::new();

       // let mut tx = Transaction {
       //     from: "Alice".to_string(),
       //     to: "Bob".to_string(),
       //     amount: 10,
       //     sender_public_key: alice.public_key_hex(),
       //     signature: None,
       // };

    let mut tx = Transaction::new(
        vec![TxInput {
            previous_tx_id: "genesis".to_string(),
            output_index: 0,
            sender_public_key: alice.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 10,
        }],

    );
        tx.sign(&alice.signing_key);

        blockchain.add_block(vec![tx]);

        let block = blockchain.chain.last().unwrap();

        network::send_block(&args[2], block);

        return;
    }

    let start = Instant::now();

    let alice = Wallet::new();
    let bob = Wallet::new();
    let carol = Wallet::new();

    #[cfg(debug_assertions)]
    {
        println!("Alice pubkey: {:?}", alice.verifying_key);

        println!("Bob pubkey: {:?}", bob.verifying_key);
    }
    println!("Alice Hex pubkey: {}", alice.public_key_hex());
    println!("Bob Hex pubkey  : {}", bob.public_key_hex());

    let difficulty = 4; //Added by Step 2
    let mut blockchain = Blockchain::new(difficulty);

    let mut tx1 = Transaction::new(
        vec![TxInput {
            previous_tx_id: "genesis".to_string(),
            output_index: 0,
            sender_public_key: alice.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 10,
        }],
    );
    //    from: "Alice".to_string(),
    //    to: "Bob".to_string(),
    //    amount: 10,
    //    sender_public_key: alice.public_key_hex(),
    //    signature: None,

    tx1.sign(&alice.signing_key);

    println!("Valid Signature: {}", tx1.verify());

    let mut tx2 = Transaction::new(
        vec![TxInput {
            previous_tx_id: tx1.id.clone(),
            output_index: 0,
            sender_public_key: bob.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: carol.public_key_hex(),
            amount: 5,
        }],
    );
    //    from: "Bob".to_string(),
    //    to: "Carol".to_string(),
    //    amount: 5,
    //    sender_public_key: bob.public_key_hex(),
    //    signature: None,

    tx2.sign(&bob.signing_key);

    println!("Valid Signature: {}", tx2.verify());

    blockchain.add_block(vec![tx1]);
    blockchain.add_block(vec![tx2]);

    println!("{:#?}", blockchain);
    println!("Blockchain valid: {}", blockchain.is_valid());

    let elapsed = start.elapsed();
    println!("Total execution time: {:.3?}", elapsed);
}
