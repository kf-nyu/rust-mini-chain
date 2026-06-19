use rust_mini_chain::blockchain::Blockchain;
use rust_mini_chain::network;
use rust_mini_chain::storage::Storage;
use rust_mini_chain::transaction::Transaction;
use rust_mini_chain::tx_output::TxOutput;
use rust_mini_chain::wallet::Wallet;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // CLI mode: run as a TCP node that listens for incoming blocks.
    if args.len() >= 3 && args[1] == "node" {
        let port = args[2].parse::<u16>().unwrap();

        network::start_node(port);

        return;
    }

    // CLI mode: send a chain request to another node.
    if args.len() >= 3 && args[1] == "request" {
        network::send_chain_request(&args[2]);

        return;
    }

    // CLI mode: build a small sample chain and send the latest block
    // to another node for manual networking tests.
    if args.len() >= 3 && args[1] == "send" {
        let mut blockchain = Blockchain::new(4);

        let alice = Wallet::new();
        let bob = Wallet::new();

        let coinbase = Transaction::new(
            vec![],
            vec![TxOutput {
                recipient: alice.public_key_hex(),
                amount: 50,
            }],
        );

        let mut tx1 = Transaction::new_utxo_spend(
            coinbase.id.clone(),
            0,
            alice.public_key_hex(),
            bob.public_key_hex(),
            10,
            alice.public_key_hex(),
            40,
        );
        tx1.sign(&alice.signing_key);

        blockchain.add_block(vec![coinbase]);
        blockchain.add_block(vec![tx1]);

        // Temporary persistence smoke test
        let storage = Storage::new();

        storage.save_blockchain(&blockchain, "chain.json").unwrap();
        let loaded = storage.load_blockchain("chain.json").unwrap();

        println!("Loaded chain with {} blocks", loaded.chain.len());

        println!("Loaded chain valid: {}", loaded.is_valid());
        // End of Temporary persistence smoke test

        let block = blockchain.chain.last().unwrap();

        network::send_block(&args[2], block);

        return;
    }

    // Default demo flow: create wallets, build a few transactions,
    // mine blocks locally, and validate the resulting chain.
    let start = Instant::now();

    let alice = Wallet::new();
    let bob = Wallet::new();
    let carol = Wallet::new();

    #[cfg(debug_assertions)]
    {
        println!("Alice pubkey: {:?}", alice.verifying_key);
        println!("Bob pubkey: {:?}", bob.verifying_key);
        println!("Carol pubkey: {:?}", carol.verifying_key);
    }
    println!("Alice Hex pubkey: {}", alice.public_key_hex());
    println!("Bob Hex pubkey  : {}", bob.public_key_hex());
    println!("Carol Hex pubkey  : {}", carol.public_key_hex());

    let difficulty = 4;
    let mut blockchain = Blockchain::new(difficulty);

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    let mut tx1 = Transaction::new_utxo_spend(
        coinbase.id.clone(),
        0,
        alice.public_key_hex(),
        bob.public_key_hex(),
        10,
        alice.public_key_hex(),
        40,
    );

    tx1.sign(&alice.signing_key);

    let mut tx2 = Transaction::new_utxo_spend(
        tx1.id.clone(),
        0,
        bob.public_key_hex(),
        carol.public_key_hex(),
        5,
        bob.public_key_hex(),
        5,
    );

    tx2.sign(&bob.signing_key);

    println!("Valid Signature tx1: {}", tx1.verify());
    println!("Valid Signature tx2: {}", tx2.verify());

    blockchain.add_block(vec![coinbase]);
    blockchain.add_block(vec![tx1]);
    blockchain.add_block(vec![tx2]);

    println!("{:#?}", blockchain);
    println!("Blockchain valid: {}", blockchain.is_valid());

    let elapsed = start.elapsed();
    println!("Total execution time: {:.3?}", elapsed);
}
