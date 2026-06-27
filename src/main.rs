use rust_mini_chain::asset::{Asset, AssetIssuance, AssetLedger, AssetTransfer, AssetType};
use rust_mini_chain::async_network;
use rust_mini_chain::blockchain::Blockchain;
use rust_mini_chain::mempool::Mempool;
use rust_mini_chain::network;
use rust_mini_chain::node_identity::{NodeIdentity, NodeRole};
use rust_mini_chain::peer_registry::PeerRegistry;
use rust_mini_chain::transaction::Transaction;
use rust_mini_chain::tx_output::TxOutput;
use rust_mini_chain::wallet::Wallet;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // CLI mode: run as an async TCP node that listens for incoming blocks.
    if args.len() >= 3 && args[1] == "async-node" {
        let port = args[2].parse::<u16>().unwrap();

        async_network::start_async_node(port).await.unwrap();

        return;
    }

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

    // CLI mode: Demo for async: async-node, async-send, async-request-chain together
    if args.len() >= 3 && args[1] == "async-demo" {
        let address = args[2].clone();

        let port: u16 = address.split(':').last().unwrap().parse().unwrap();

        tokio::spawn(async move {
            if let Err(error) = async_network::start_async_node(port).await {
                eprintln!("Async node failed: {error}");
            }
        });

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

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

        let block = blockchain.chain.last().unwrap();

        async_network::send_async_block(&args[2], block)
            .await
            .unwrap();

        async_network::send_async_chain_request(&address)
            .await
            .unwrap();

        return;
    }

    // CLI mode: build a small sample chain and send the latest block by async
    // to another node for manual networking tests.
    if args.len() >= 3 && args[1] == "async-send" {
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

        let block = blockchain.chain.last().unwrap();

        async_network::send_async_block(&args[2], block)
            .await
            .unwrap();

        return;
    }

    // CLI mode: send a async chain request to another node.
    if args.len() >= 3 && args[1] == "async-request-chain" {
        async_network::send_async_chain_request(&args[2])
            .await
            .unwrap();

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

        let block = blockchain.chain.last().unwrap();

        network::send_block(&args[2], block);

        return;
    }

    // CLI mode: mempool demo
    if args.len() >= 2 && args[1] == "mempool-demo" {
        let mut blockchain = Blockchain::new(4);
        let mut mempool = Mempool::new();

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

        println!("Mempool before add: {}", mempool.len());

        if mempool.add_transaction(tx1) {
            println!("Transaction added to mempool");
        }

        println!("Mempool after add: {}", mempool.len());

        let selected = mempool.select_transactions(1);

        println!("Selected {} transaction(s) for mining", selected.len());

        blockchain.add_block(selected.clone());

        mempool.remove_transactions(&selected);

        println!("Mempool after mining: {}", mempool.len());
        println!("Blockchain valid: {}", blockchain.is_valid());

        return;
    }

    //Demostrate a permissoned network by comparing a trusted peer
    //with an unregistered peer during membership verification.
    if args.len() >= 2 && args[1] == "permissioned-demo" {
        let trusted_identity = NodeIdentity::new("validator-1".to_string(), NodeRole::Validator);

        let untrusted_identity = NodeIdentity::new("validator-2".to_string(), NodeRole::Validator);

        let mut registry = PeerRegistry::new();

        println!("Permissioned network demo");
        println!("Registering trusted peer: {}", trusted_identity.node_id);

        registry.add_peer(trusted_identity.clone());

        println!(
            "Trusted peer accepted: {}",
            registry.is_trusted(&trusted_identity.node_id)
        );

        println!(
            "Untrusted peer accepted: {}",
            registry.is_trusted(&untrusted_identity.node_id)
        );

        println!("Permissoned demo complete");

        return;
    }

    // Demonstrates issuing a fungible aset and recording the initial supply
    // in the ledger before transferring a protion to another owner.
    if args.len() >= 2 && args[1] == "asset-demo" {
        println!("Asset tokenization demo");

        let asset = Asset::new(
            "asset-1".to_string(),
            "Digital Dallar".to_string(),
            "DUSD".to_string(),
            AssetType::Fungible,
            1_000_000,
        );

        let issuance = AssetIssuance::new(asset.clone(), "issuer-1".to_string());

        let mut ledger = AssetLedger::new();

        ledger.apply_issuance(&issuance);

        println!(
            "Issued {} {} to {}",
            asset.total_supply, asset.symbol, issuance.issuer
        );

        let transfer = AssetTransfer::new(
            asset.asset_id.clone(),
            "issuer-1".to_string(),
            "wallet-1".to_string(),
            250_000,
        );

        let transfer_accepted = ledger.apply_transfer(&transfer);

        println!("Transfer accepted: {transfer_accepted}");

        println!(
            "Issuer balance: {}",
            ledger.balance_of(&asset.asset_id, "issuer-1")
        );

        println!(
            "Wallet-1 balance: {}",
            ledger.balance_of(&asset.asset_id, "wallet-1")
        );

        println!("Asset tokenization demo complete");

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
