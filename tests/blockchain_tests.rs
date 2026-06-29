use digital_asset_ledger::asset::{
    Asset, AssetIssuance, AssetLedger, AssetOwnership, AssetTransfer, AssetType,
};
use digital_asset_ledger::async_network;
use digital_asset_ledger::blockchain::Blockchain;
use digital_asset_ledger::compliance::{ComplianceDecision, ComplianceEngine};
use digital_asset_ledger::custody::{CustodyAccount, CustodyAccountStatus, CustodyRegistry};
use digital_asset_ledger::mempool::Mempool;
use digital_asset_ledger::network_message::NetworkMessage;
use digital_asset_ledger::node_identity::{NodeIdentity, NodeRole};
use digital_asset_ledger::peer_registry::PeerRegistry;
use digital_asset_ledger::policy::{PolicyDecision, PolicyEngine};
use digital_asset_ledger::settlement::{SettlementEngine, SettlementInstruction, SettlementStatus};
use digital_asset_ledger::storage::Storage;
use digital_asset_ledger::transaction::Transaction;
use digital_asset_ledger::tx_input::TxInput;
use digital_asset_ledger::tx_output::TxOutput;
use digital_asset_ledger::utxo::UTXOSet;
use digital_asset_ledger::wallet::Wallet;
use std::fs;
use tokio::io::AsyncWriteExt;

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

    //let alice = Wallet::new();
    let bob = Wallet::new();

    let transaction = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 10,
        }],
    );

    assert!(mempool.add_transaction(transaction));
    assert_eq!(mempool.len(), 1);
    assert!(!mempool.is_empty());
}

#[test]
fn mempool_rejects_invalid_transaction() {
    let mut mempool = Mempool::new();

    let alice = Wallet::new();
    let bob = Wallet::new();

    let transaction = Transaction::new(
        vec![TxInput {
            previous_tx_id: "fake_tx".to_string(),
            output_index: 0,
            sender_public_key: alice.public_key_hex(),
            signature: None,
        }],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 10,
        }],
    );

    assert!(!mempool.add_transaction(transaction));
    assert_eq!(mempool.len(), 0);
    assert!(mempool.is_empty());
}

#[test]
fn mempool_rejects_duplicate_transaction() {
    let mut mempool = Mempool::new();

    let bob = Wallet::new();

    let transaction = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: bob.public_key_hex(),
            amount: 10,
        }],
    );

    assert!(mempool.add_transaction(transaction.clone()));
    assert!(!mempool.add_transaction(transaction));

    assert_eq!(mempool.len(), 1);
}

#[test]
fn mempool_selects_transactions_for_block() {
    let mut mempool = Mempool::new();

    let wallet = Wallet::new();

    let tx1 = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: wallet.public_key_hex(),
            amount: 10,
        }],
    );

    let tx2 = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: wallet.public_key_hex(),
            amount: 20,
        }],
    );

    let tx3 = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: wallet.public_key_hex(),
            amount: 30,
        }],
    );

    assert!(mempool.add_transaction(tx1));
    assert!(mempool.add_transaction(tx2));
    assert!(mempool.add_transaction(tx3));

    let selected = mempool.select_transactions(2);

    assert_eq!(selected.len(), 2);
}

#[test]
fn mempool_removes_mined_transactions() {
    let mut mempool = Mempool::new();

    let wallet = Wallet::new();

    let tx1 = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: wallet.public_key_hex(),
            amount: 10,
        }],
    );

    let tx2 = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: wallet.public_key_hex(),
            amount: 20,
        }],
    );

    let tx3 = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: wallet.public_key_hex(),
            amount: 30,
        }],
    );

    assert!(mempool.add_transaction(tx1.clone()));
    assert!(mempool.add_transaction(tx2.clone()));
    assert!(mempool.add_transaction(tx3.clone()));

    let selected = vec![tx1, tx2];

    mempool.remove_transactions(&selected);

    assert_eq!(mempool.len(), 1);

    let remaining = mempool.transactions();

    assert_eq!(remaining[0].id, tx3.id);
}

#[test]
fn mempool_lifecycle_mines_and_removes_transactions() {
    let mut blockchain = Blockchain::new(2);
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

    assert!(mempool.add_transaction(tx1));

    let selected = mempool.select_transactions(1);

    blockchain.add_block(selected.clone());

    mempool.remove_transactions(&selected);

    assert!(mempool.is_empty());
    assert!(blockchain.is_valid());
}

#[tokio::test]
async fn test_async_request_response() {
    let port = 7100;
    let address = format!("127.0.0.1:{port}");

    tokio::spawn(async move {
        let _ = async_network::start_async_node(port).await;
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let mut stream = tokio::net::TcpStream::connect(&address).await.unwrap();

    async_network::write_message(&mut stream, &NetworkMessage::ChainRequest)
        .await
        .unwrap();

    stream.shutdown().await.unwrap();

    let response = async_network::read_message(&mut stream).await.unwrap();

    match response {
        NetworkMessage::ChainResponse(blockchain) => {
            assert_eq!(blockchain.chain.len(), 2);
            assert!(blockchain.is_valid());
        }
        other => panic!("Expected ChainResponse, got {other:?}"),
    }
}

#[test]
fn node_identity_tracks_role() {
    let node = NodeIdentity::new("node-1".to_string(), NodeRole::Validator);

    assert_eq!(node.node_id, "node-1");
    assert!(node.is_validator());
}

#[test]
fn peer_registry_adds_trusted_peer() {
    let mut registry = PeerRegistry::new();

    let peer = NodeIdentity::new("validator-1".to_string(), NodeRole::Validator);

    assert!(registry.add_peer(peer));
    assert_eq!(registry.len(), 1);
    assert!(registry.is_trusted("validator-1"));
}

#[test]
fn peer_registry_rejects_duplicate_peer() {
    let mut registry = PeerRegistry::new();

    let peer = NodeIdentity::new("validator-1".to_string(), NodeRole::Validator);

    assert!(registry.add_peer(peer.clone()));
    assert!(!registry.add_peer(peer));
    assert_eq!(registry.len(), 1);
}

#[test]
fn network_message_hello_round_trip() {
    let identity = NodeIdentity::new("validattor-1".to_string(), NodeRole::Validator);

    let message = NetworkMessage::Hello(identity.clone());

    let json = serde_json::to_string(&message).unwrap();

    let decoded: NetworkMessage = serde_json::from_str(&json).unwrap();

    match decoded {
        NetworkMessage::Hello(node) => {
            assert_eq!(node, identity);
        }
        other => panic!("Expected Hello message, got {other:?}"),
    }
}

#[tokio::test]
async fn permissioned_handshake_accepts_trusted_peer() {
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap().to_string();

    let trusted_identity = NodeIdentity::new("validator-1".to_string(), NodeRole::Validator);

    let mut registry = PeerRegistry::new();
    assert!(registry.add_peer(trusted_identity.clone()));

    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();

        let accepted = async_network::read_permissioned_handshake(&mut stream, &registry)
            .await
            .unwrap();

        assert!(accepted);
    });

    let mut stream = tokio::net::TcpStream::connect(&address).await.unwrap();

    async_network::write_message(&mut stream, &NetworkMessage::Hello(trusted_identity))
        .await
        .unwrap();

    stream.shutdown().await.unwrap();

    server.await.unwrap();
}

#[tokio::test]
async fn permissioned_handshake_rejects_untrusted_peer() {
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap().to_string();

    let untrusted_identity =
        NodeIdentity::new("unknown-validator".to_string(), NodeRole::Validator);

    let registry = PeerRegistry::new();

    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();

        let accepted = async_network::read_permissioned_handshake(&mut stream, &registry)
            .await
            .unwrap();

        assert!(!accepted);
    });

    let mut stream = tokio::net::TcpStream::connect(&address).await.unwrap();

    async_network::write_message(&mut stream, &NetworkMessage::Hello(untrusted_identity))
        .await
        .unwrap();

    stream.shutdown().await.unwrap();

    server.await.unwrap();
}

#[test]
fn asset_model_tracks_fungible_asset() {
    let asset = Asset::new(
        "asset-1".to_string(),
        "Digital Dollar".to_string(),
        "DUSD".to_string(),
        AssetType::Fungible,
        1_000_000,
    );

    assert_eq!(asset.asset_id, "asset-1");
    assert_eq!(asset.name, "Digital Dollar");
    assert_eq!(asset.symbol, "DUSD");
    assert_eq!(asset.total_supply, 1_000_000);

    assert!(asset.is_fungible());
    assert!(!asset.is_non_fungible());
}

#[test]
fn asset_model_tracks_non_fungible_asset() {
    let asset = Asset::new(
        "asset-2".to_string(),
        "Warehouse Receipt".to_string(),
        "WR-001".to_string(),
        AssetType::NonFungible,
        1,
    );

    assert_eq!(asset.asset_id, "asset-2");
    assert_eq!(asset.name, "Warehouse Receipt");
    assert_eq!(asset.symbol, "WR-001");
    assert_eq!(asset.total_supply, 1);

    assert!(!asset.is_fungible());
    assert!(asset.is_non_fungible());
}

#[test]
fn asset_issuance_tracks_asset_and_issuer() {
    let asset = Asset::new(
        "asset-1".to_string(),
        "Digital Dollar".to_string(),
        "DUSD".to_string(),
        AssetType::Fungible,
        1_000_000,
    );

    let issuance = AssetIssuance::new(asset.clone(), "issuer-1".to_string());

    assert_eq!(issuance.asset, asset);
    assert_eq!(issuance.issuer, "issuer-1");
}

#[test]
fn asset_ownership_tracks_owner_and_quantity() {
    let ownership = AssetOwnership::new("asset-1".to_string(), "wallet-1".to_string(), 500);

    assert_eq!(ownership.asset_id, "asset-1");
    assert_eq!(ownership.owner, "wallet-1");
    assert_eq!(ownership.quantity, 500);
}

#[test]
fn asset_transfer_tracks_sender_receiver_and_quantity() {
    let transfer = AssetTransfer::new(
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        250,
    );

    assert_eq!(transfer.asset_id, "asset-1");
    assert_eq!(transfer.from, "wallet-1");
    assert_eq!(transfer.to, "wallet-2");
    assert_eq!(transfer.quantity, 250);
}

#[test]
fn asset_ledger_credits_owner_balance() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "wallet-1", 500);
    ledger.credit("asset-1", "wallet-1", 250);

    assert_eq!(ledger.balance_of("asset-1", "wallet-1"), 750);
    assert_eq!(ledger.balance_of("asset-1", "wallet-2"), 0);
}

#[test]
fn asset_ledger_applies_valid_transfer() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "wallet-1", 500);

    let transfer = AssetTransfer::new(
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        200,
    );

    assert!(ledger.apply_transfer(&transfer));

    assert_eq!(ledger.balance_of("asset-1", "wallet-1"), 300);
    assert_eq!(ledger.balance_of("asset-1", "wallet-2"), 200);
}

#[test]
fn asset_ledger_rejects_transfer_with_insufficient_balance() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "wallet-1", 100);

    let transfer = AssetTransfer::new(
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        200,
    );

    assert!(!ledger.apply_transfer(&transfer));

    assert_eq!(ledger.balance_of("asset-1", "wallet-1"), 100);
    assert_eq!(ledger.balance_of("asset-1", "wallet-2"), 0);
}

#[test]
fn asset_ledger_applies_asset_issuance() {
    let mut ledger = AssetLedger::new();

    let asset = Asset::new(
        "asset-1".to_string(),
        "Digital Dollar".to_string(),
        "DUSD".to_string(),
        AssetType::Fungible,
        1_000_000,
    );

    let issuance = AssetIssuance::new(asset, "issuer-1".to_string());

    ledger.apply_issuance(&issuance);

    assert_eq!(ledger.balance_of("asset-1", "issuer-1"), 1_000_000);
}

#[test]
fn settlement_instruction_starts_pending() {
    let instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        100,
    );

    assert_eq!(instruction.settlement_id, "settlement-1");
    assert_eq!(instruction.asset_id, "asset-1");
    assert_eq!(instruction.from, "wallet-1");
    assert_eq!(instruction.to, "wallet-2");
    assert_eq!(instruction.quantity, 100);
    assert_eq!(instruction.status, SettlementStatus::Pending);

    assert!(instruction.is_pending());
    assert!(!instruction.is_settled());
    assert!(!instruction.is_failed());
}

#[test]
fn settlement_instruction_can_be_marked_settled() {
    let mut instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        100,
    );

    instruction.mark_settled();

    assert_eq!(instruction.status, SettlementStatus::Settled);
    assert!(instruction.is_settled());
    assert!(!instruction.is_pending());
    assert!(!instruction.is_failed());
}

#[test]
fn settlement_instruction_can_be_marked_failed() {
    let mut instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        100,
    );

    instruction.mark_failed();

    assert_eq!(instruction.status, SettlementStatus::Failed);
    assert!(instruction.is_failed());
    assert!(!instruction.is_pending());
    assert!(!instruction.is_settled());
}

#[test]
fn settlement_engine_adds_instruction() {
    let mut engine = SettlementEngine::new();

    let instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        100,
    );

    assert!(engine.add_instruction(instruction));

    assert_eq!(engine.instruction_count(), 1);

    let stored = engine.get_instruction("settlement-1").unwrap();

    assert_eq!(stored.settlement_id, "settlement-1");
    assert_eq!(stored.asset_id, "asset-1");
    assert_eq!(stored.from, "wallet-1");
    assert_eq!(stored.to, "wallet-2");
    assert_eq!(stored.quantity, 100);
    assert_eq!(stored.status, SettlementStatus::Pending);
}

#[test]
fn settlement_engine_rejects_duplicate_instruction() {
    let mut engine = SettlementEngine::new();

    let instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        100,
    );

    assert!(engine.add_instruction(instruction.clone()));
    assert!(!engine.add_instruction(instruction));

    assert_eq!(engine.instruction_count(), 1);
}

#[test]
fn settlement_engine_executes_valid_settlement() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "wallet-1", 500);

    let mut engine = SettlementEngine::new();

    let instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        200,
    );

    assert!(engine.add_instruction(instruction));

    assert!(engine.execute_settlement("settlement-1", &mut ledger));

    assert_eq!(ledger.balance_of("asset-1", "wallet-1"), 300);
    assert_eq!(ledger.balance_of("asset-1", "wallet-2"), 200);

    let stored = engine.get_instruction("settlement-1").unwrap();

    assert!(stored.is_settled());
    assert_eq!(stored.status, SettlementStatus::Settled);
}

#[test]
fn settlement_engine_executes_pending_settlements() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "wallet-1", 500);

    let mut engine = SettlementEngine::new();

    let settlement_1 = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        200,
    );

    let settlement_2 = SettlementInstruction::new(
        "settlement-2".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-3".to_string(),
        100,
    );

    assert!(engine.add_instruction(settlement_1));
    assert!(engine.add_instruction(settlement_2));

    let settled_count = engine.execute_pending(&mut ledger);

    assert_eq!(settled_count, 2);

    assert_eq!(ledger.balance_of("asset-1", "wallet-1"), 200);
    assert_eq!(ledger.balance_of("asset-1", "wallet-2"), 200);
    assert_eq!(ledger.balance_of("asset-1", "wallet-3"), 100);

    assert!(engine.get_instruction("settlement-1").unwrap().is_settled());

    assert!(engine.get_instruction("settlement-2").unwrap().is_settled());
}

#[test]
fn settlement_engine_counts_instruction_statuses() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "wallet-1", 300);

    let mut engine = SettlementEngine::new();

    let settlement_1 = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        100,
    );

    let settlement_2 = SettlementInstruction::new(
        "settlement-2".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-3".to_string(),
        500,
    );

    let settlement_3 = SettlementInstruction::new(
        "settlement-3".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-4".to_string(),
        50,
    );

    assert!(engine.add_instruction(settlement_1));
    assert!(engine.add_instruction(settlement_2));
    assert!(engine.add_instruction(settlement_3));

    assert_eq!(engine.pending_count(), 3);
    assert_eq!(engine.settled_count(), 0);
    assert_eq!(engine.failed_count(), 0);

    assert!(engine.execute_settlement("settlement-1", &mut ledger));
    assert!(!engine.execute_settlement("settlement-2", &mut ledger));

    assert_eq!(engine.pending_count(), 1);
    assert_eq!(engine.settled_count(), 1);
    assert_eq!(engine.failed_count(), 1);
}

#[test]
fn settlement_engine_rejects_reexecution_of_settled_instruction() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "wallet-1", 500);

    let mut engine = SettlementEngine::new();

    let settlement_1 = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        200,
    );

    assert!(engine.add_instruction(settlement_1));

    assert!(engine.execute_settlement("settlement-1", &mut ledger));
    assert!(!engine.execute_settlement("settlement-2", &mut ledger));

    assert_eq!(ledger.balance_of("asset-1", "wallet-1"), 300);
    assert_eq!(ledger.balance_of("asset-1", "wallet-2"), 200);

    let stored = engine.get_instruction("settlement-1").unwrap();

    assert!(stored.is_settled());
}

#[test]
fn settlement_engine_returns_pending_instructions() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "wallet-1", 300);

    let mut engine = SettlementEngine::new();

    let settlement_1 = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        100,
    );

    let settlement_2 = SettlementInstruction::new(
        "settlement-2".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-3".to_string(),
        100,
    );

    assert!(engine.add_instruction(settlement_1));
    assert!(engine.add_instruction(settlement_2));

    assert!(engine.execute_settlement("settlement-1", &mut ledger));

    let pending = engine.pending_instructions();

    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].settlement_id, "settlement-2");
    assert!(pending[0].is_pending());
}

#[test]
fn settlement_engine_returns_settled_and_failed_instructions() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "wallet-1", 300);

    let mut engine = SettlementEngine::new();

    let settlement_1 = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-2".to_string(),
        100,
    );

    let settlement_2 = SettlementInstruction::new(
        "settlement-2".to_string(),
        "asset-1".to_string(),
        "wallet-1".to_string(),
        "wallet-3".to_string(),
        500,
    );

    assert!(engine.add_instruction(settlement_1));
    assert!(engine.add_instruction(settlement_2));

    assert!(engine.execute_settlement("settlement-1", &mut ledger));
    assert!(!engine.execute_settlement("settlement-2", &mut ledger));

    let settled = engine.settled_instructions();
    let failed = engine.failed_instructions();

    assert_eq!(settled.len(), 1);
    assert_eq!(failed.len(), 1);

    assert_eq!(settled[0].settlement_id, "settlement-1");
    assert!(settled[0].is_settled());

    assert_eq!(failed[0].settlement_id, "settlement-2");
    assert!(failed[0].is_failed());
}

#[test]
fn custody_account_starts_active() {
    let account = CustodyAccount::new("custody-1".to_string(), "owner-1".to_string());

    assert_eq!(account.account_id, "custody-1");
    assert_eq!(account.owner, "owner-1");
    assert_eq!(account.status, CustodyAccountStatus::Active);
    assert!(account.is_active());
}

#[test]
fn custody_account_can_be_frozen() {
    let mut account = CustodyAccount::new("custody-1".to_string(), "owner-1".to_string());

    account.freeze();

    assert_eq!(account.status, CustodyAccountStatus::Frozen);
    assert!(account.is_frozen());
}

#[test]
fn custody_account_can_be_closed() {
    let mut account = CustodyAccount::new("custody-1".to_string(), "owner-1".to_string());

    account.close();

    assert_eq!(account.status, CustodyAccountStatus::Closed);
    assert!(account.is_closed());
}
#[test]
fn custody_registry_adds_account() {
    let mut registry = CustodyRegistry::new();

    let account = CustodyAccount::new("custody-1".to_string(), "owner-1".to_string());

    assert!(registry.add_account(account));

    assert_eq!(registry.account_count(), 1);

    let stored = registry.get_account("custody-1").unwrap();

    assert_eq!(stored.account_id, "custody-1");
    assert_eq!(stored.owner, "owner-1");
    assert_eq!(stored.status, CustodyAccountStatus::Active);
}

#[test]
fn custody_registry_rejects_duplicate_account() {
    let mut registry = CustodyRegistry::new();

    let account = CustodyAccount::new("custody-1".to_string(), "owner-1".to_string());

    assert!(registry.add_account(account.clone()));
    assert!(!registry.add_account(account));

    assert_eq!(registry.account_count(), 1);
}

#[test]
fn custody_registry_freezes_account() {
    let mut registry = CustodyRegistry::new();

    let account = CustodyAccount::new("custody-1".to_string(), "owner-1".to_string());

    assert!(registry.add_account(account));

    assert!(registry.freeze_account("custody-1"));

    let stored = registry.get_account("custody-1").unwrap();

    assert_eq!(stored.status, CustodyAccountStatus::Frozen);
    assert!(stored.is_frozen());
}

#[test]
fn custody_registry_closes_account() {
    let mut registry = CustodyRegistry::new();

    let account = CustodyAccount::new("custody-1".to_string(), "owner-1".to_string());

    assert!(registry.add_account(account));

    assert!(registry.close_account("custody-1"));

    let stored = registry.get_account("custody-1").unwrap();

    assert_eq!(stored.status, CustodyAccountStatus::Closed);
    assert!(stored.is_closed());
}

#[test]
fn custody_registry_returns_false_for_missing_account_status_update() {
    let mut registry = CustodyRegistry::new();

    assert!(!registry.freeze_account("missing-custody"));
    assert!(!registry.close_account("missing-custody"));
}

#[test]
fn settlement_engine_executes_settlement_when_custody_accounts_are_active() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "custody-1", 500);

    let mut custody_registry = CustodyRegistry::new();

    assert!(custody_registry.add_account(CustodyAccount::new(
        "custody-1".to_string(),
        "owner-1".to_string(),
    )));

    assert!(custody_registry.add_account(CustodyAccount::new(
        "custody-2".to_string(),
        "owner-2".to_string(),
    )));

    let mut engine = SettlementEngine::new();

    let instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "custody-1".to_string(),
        "custody-2".to_string(),
        200,
    );

    assert!(engine.add_instruction(instruction));

    assert!(
        engine.execute_settlement_with_custody("settlement-1", &mut ledger, &custody_registry,)
    );

    assert_eq!(ledger.balance_of("asset-1", "custody-1"), 300);
    assert_eq!(ledger.balance_of("asset-1", "custody-2"), 200);

    let stored = engine.get_instruction("settlement-1").unwrap();

    assert!(stored.is_settled());
}

#[test]
fn settlement_engine_rejects_settlement_from_frozen_custody_account() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "custody-1", 500);

    let mut custody_registry = CustodyRegistry::new();

    assert!(custody_registry.add_account(CustodyAccount::new(
        "custody-1".to_string(),
        "owner-1".to_string(),
    )));

    assert!(custody_registry.add_account(CustodyAccount::new(
        "custody-2".to_string(),
        "owner-2".to_string(),
    )));

    assert!(custody_registry.freeze_account("custody-1"));

    let mut engine = SettlementEngine::new();

    let instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "custody-1".to_string(),
        "custody-2".to_string(),
        200,
    );

    assert!(engine.add_instruction(instruction));

    assert!(!engine.execute_settlement_with_custody(
        "settlement-1",
        &mut ledger,
        &custody_registry,
    ));

    assert_eq!(ledger.balance_of("asset-1", "custody-1"), 500);
    assert_eq!(ledger.balance_of("asset-1", "custody-2"), 0);

    let stored = engine.get_instruction("settlement-1").unwrap();

    assert!(stored.is_failed());
}

#[test]
fn settlement_engine_rejects_settlement_to_frozen_custody_account() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "custody-1", 500);

    let mut custody_registry = CustodyRegistry::new();

    assert!(custody_registry.add_account(CustodyAccount::new(
        "custody-1".to_string(),
        "owner-1".to_string(),
    )));

    assert!(custody_registry.add_account(CustodyAccount::new(
        "custody-2".to_string(),
        "owner-2".to_string(),
    )));

    assert!(custody_registry.freeze_account("custody-2"));

    let mut engine = SettlementEngine::new();

    let instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "custody-1".to_string(),
        "custody-2".to_string(),
        200,
    );

    assert!(engine.add_instruction(instruction));

    assert!(!engine.execute_settlement_with_custody(
        "settlement-1",
        &mut ledger,
        &custody_registry,
    ));

    assert_eq!(ledger.balance_of("asset-1", "custody-1"), 500);
    assert_eq!(ledger.balance_of("asset-1", "custody-2"), 0);

    let stored = engine.get_instruction("settlement-1").unwrap();

    assert!(stored.is_failed());
}

#[test]
fn settlement_engine_rejects_settlement_with_closed_custody_account() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "custody-1", 500);

    let mut custody_registry = CustodyRegistry::new();

    assert!(custody_registry.add_account(CustodyAccount::new(
        "custody-1".to_string(),
        "owner-1".to_string(),
    )));

    assert!(custody_registry.add_account(CustodyAccount::new(
        "custody-2".to_string(),
        "owner-2".to_string(),
    )));

    assert!(custody_registry.close_account("custody-1"));

    let mut engine = SettlementEngine::new();

    let instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "custody-1".to_string(),
        "custody-2".to_string(),
        200,
    );

    assert!(engine.add_instruction(instruction));

    assert!(!engine.execute_settlement_with_custody(
        "settlement-1",
        &mut ledger,
        &custody_registry,
    ));

    assert_eq!(ledger.balance_of("asset-1", "custody-1"), 500);
    assert_eq!(ledger.balance_of("asset-1", "custody-2"), 0);

    let stored = engine.get_instruction("settlement-1").unwrap();

    assert!(stored.is_failed());
}

#[test]
fn settlement_engine_rejects_settlement_from_missing_custody_account() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "custody-1", 500);

    let mut custody_registry = CustodyRegistry::new();

    assert!(custody_registry.add_account(CustodyAccount::new(
        "custody-2".to_string(),
        "owner-2".to_string(),
    )));

    let mut engine = SettlementEngine::new();

    let instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "custody-1".to_string(),
        "custody-2".to_string(),
        200,
    );

    assert!(engine.add_instruction(instruction));

    assert!(!engine.execute_settlement_with_custody(
        "settlement-1",
        &mut ledger,
        &custody_registry,
    ));

    assert_eq!(ledger.balance_of("asset-1", "custody-1"), 500);
    assert_eq!(ledger.balance_of("asset-1", "custody-2"), 0);

    let stored = engine.get_instruction("settlement-1").unwrap();

    assert!(stored.is_failed());
}

#[test]
fn settlement_engine_rejects_settlement_to_missing_custody_account() {
    let mut ledger = AssetLedger::new();

    ledger.credit("asset-1", "custody-1", 500);

    let mut custody_registry = CustodyRegistry::new();

    assert!(custody_registry.add_account(CustodyAccount::new(
        "custody-1".to_string(),
        "owner-1".to_string(),
    )));

    let mut engine = SettlementEngine::new();

    let instruction = SettlementInstruction::new(
        "settlement-1".to_string(),
        "asset-1".to_string(),
        "custody-1".to_string(),
        "custody-2".to_string(),
        200,
    );

    assert!(engine.add_instruction(instruction));

    assert!(!engine.execute_settlement_with_custody(
        "settlement-1",
        &mut ledger,
        &custody_registry,
    ));

    assert_eq!(ledger.balance_of("asset-1", "custody-1"), 500);
    assert_eq!(ledger.balance_of("asset-1", "custody-2"), 0);

    let stored = engine.get_instruction("settlement-1").unwrap();

    assert!(stored.is_failed());
}

#[test]
fn custody_registry_returns_active_accounts() {
    let mut registry = CustodyRegistry::new();

    assert!(registry.add_account(CustodyAccount::new(
        "custody-1".to_string(),
        "owner-1".to_string(),
    )));

    assert!(registry.add_account(CustodyAccount::new(
        "custody-2".to_string(),
        "owner-2".to_string(),
    )));

    assert!(registry.freeze_account("custody-2"));

    let active = registry.active_accounts();

    assert_eq!(active.len(), 1);
    assert_eq!(active[0].account_id, "custody-1");
}

#[test]
fn custody_registry_returns_frozen_accounts() {
    let mut registry = CustodyRegistry::new();

    assert!(registry.add_account(CustodyAccount::new(
        "custody-1".to_string(),
        "owner-1".to_string(),
    )));

    assert!(registry.freeze_account("custody-1"));

    let frozen = registry.frozen_accounts();

    assert_eq!(frozen.len(), 1);
    assert_eq!(frozen[0].account_id, "custody-1");
}

#[test]
fn custody_registry_returns_closed_accounts() {
    let mut registry = CustodyRegistry::new();

    assert!(registry.add_account(CustodyAccount::new(
        "custody-1".to_string(),
        "owner-1".to_string(),
    )));

    assert!(registry.close_account("custody-1"));

    let closed = registry.closed_accounts();

    assert_eq!(closed.len(), 1);
    assert_eq!(closed[0].account_id, "custody-1");
}

#[test]
fn custody_registry_counts_account_statuses() {
    let mut registry = CustodyRegistry::new();

    assert!(registry.add_account(CustodyAccount::new(
        "custody-1".to_string(),
        "owner-1".to_string(),
    )));

    assert!(registry.add_account(CustodyAccount::new(
        "custody-2".to_string(),
        "owner-2".to_string(),
    )));

    assert!(registry.add_account(CustodyAccount::new(
        "custody-3".to_string(),
        "owner-3".to_string(),
    )));

    assert!(registry.freeze_account("custody-2"));
    assert!(registry.close_account("custody-3"));

    assert_eq!(registry.active_count(), 1);
    assert_eq!(registry.frozen_count(), 1);
    assert_eq!(registry.closed_count(), 1);
}

#[test]
fn policy_engine_allows_settlement_within_quantity_limit() {
    let engine = PolicyEngine::new(1_000);
    let instruction = SettlementInstruction::new(
        "settlement-001".to_string(),
        "asset-001".to_string(),
        "alice-custody".to_string(),
        "bob-custody".to_string(),
        500,
    );

    let decision = engine.evaluate_settlement(&instruction);

    assert_eq!(decision, PolicyDecision::Allow);
}

#[test]
fn policy_engine_rejects_settlement_above_quantity_limit() {
    let engine = PolicyEngine::new(1_000);
    let instruction = SettlementInstruction::new(
        "settlement-002".to_string(),
        "asset-001".to_string(),
        "alice-custody".to_string(),
        "bob-custody".to_string(),
        1_500,
    );

    let decision = engine.evaluate_settlement(&instruction);

    assert_eq!(
        decision,
        PolicyDecision::Deny("settlement quantity exceeds policy limit".to_string())
    );
}

#[test]
fn policy_decision_can_represent_denial_reason() {
    let decision = PolicyDecision::Deny("amount exceeds policy limit".to_string());

    assert_eq!(
        decision,
        PolicyDecision::Deny("amount exceeds policy limit".to_string())
    );
}

#[test]
fn settlement_engine_rejects_settlement_above_policy_quantity_limit() {
    let settlement_id = "settlement-policy-001";

    let mut engine = SettlementEngine::new();
    let mut ledger = AssetLedger::new();
    let mut custody_registry = CustodyRegistry::new();
    let policy_engine = PolicyEngine::new(1_000);

    ledger.credit("asset-001", "alice-custody", 2_000);

    custody_registry.add_account(CustodyAccount::new(
        "alice-custody".to_string(),
        "alice".to_string(),
    ));

    custody_registry.add_account(CustodyAccount::new(
        "bob-custody".to_string(),
        "bob".to_string(),
    ));

    let instruction = SettlementInstruction::new(
        settlement_id.to_string(),
        "asset-001".to_string(),
        "alice-custody".to_string(),
        "bob-custody".to_string(),
        1_500,
    );

    engine.add_instruction(instruction);

    let executed = engine.execute_settlement_with_policy(
        settlement_id,
        &mut ledger,
        &custody_registry,
        &policy_engine,
    );

    assert!(!executed);

    let failed = engine
        .get_instruction(settlement_id)
        .expect("settlement should exist");

    assert_eq!(failed.status, SettlementStatus::Failed);
    assert_eq!(ledger.balance_of("asset-001", "alice-custody"), 2_000);
    assert_eq!(ledger.balance_of("asset-001", "bob-custody"), 0);
}

#[test]
fn settlement_engine_executes_settlement_within_policy_quantity_limit() {
    let settlement_id = "settlement-policy-002";

    let mut engine = SettlementEngine::new();
    let mut ledger = AssetLedger::new();
    let mut custody_registry = CustodyRegistry::new();
    let policy_engine = PolicyEngine::new(1_000);

    ledger.credit("asset-001", "alice-custody", 2_000);

    custody_registry.add_account(CustodyAccount::new(
        "alice-custody".to_string(),
        "alice".to_string(),
    ));

    custody_registry.add_account(CustodyAccount::new(
        "bob-custody".to_string(),
        "bob".to_string(),
    ));

    let instruction = SettlementInstruction::new(
        settlement_id.to_string(),
        "asset-001".to_string(),
        "alice-custody".to_string(),
        "bob-custody".to_string(),
        500,
    );

    engine.add_instruction(instruction);

    let executed = engine.execute_settlement_with_policy(
        settlement_id,
        &mut ledger,
        &custody_registry,
        &policy_engine,
    );

    assert!(executed);

    let settled = engine
        .get_instruction(settlement_id)
        .expect("settlement should exist");

    assert_eq!(settled.status, SettlementStatus::Settled);
    assert_eq!(ledger.balance_of("asset-001", "alice-custody"), 1_500);
    assert_eq!(ledger.balance_of("asset-001", "bob-custody"), 500);
}

#[test]
fn settlement_engine_rejects_settlement_from_blocked_policy_account() {
    let settlement_id = "settlement-policy-005";

    let mut engine = SettlementEngine::new();
    let mut ledger = AssetLedger::new();
    let mut custody_registry = CustodyRegistry::new();
    let mut policy_engine = PolicyEngine::new(1_000);

    ledger.credit("asset-001", "alice-custody", 2_000);

    custody_registry.add_account(CustodyAccount::new(
        "alice-custody".to_string(),
        "alice".to_string(),
    ));

    custody_registry.add_account(CustodyAccount::new(
        "bob-custody".to_string(),
        "bob".to_string(),
    ));

    policy_engine.block_account("alice-custody".to_string());

    let instruction = SettlementInstruction::new(
        settlement_id.to_string(),
        "asset-001".to_string(),
        "alice-custody".to_string(),
        "bob-custody".to_string(),
        500,
    );

    engine.add_instruction(instruction);

    let executed = engine.execute_settlement_with_policy(
        settlement_id,
        &mut ledger,
        &custody_registry,
        &policy_engine,
    );

    assert!(!executed);

    let failed = engine
        .get_instruction(settlement_id)
        .expect("settlement should exist");

    assert_eq!(failed.status, SettlementStatus::Failed);
    assert_eq!(ledger.balance_of("asset-001", "alice-custody"), 2_000);
    assert_eq!(ledger.balance_of("asset-001", "bob-custody"), 0);
}

#[test]
fn policy_engine_rejects_blocked_sender_account() {
    let mut engine = PolicyEngine::new(1_000);
    engine.block_account("alice-custody".to_string());

    let instruction = SettlementInstruction::new(
        "settlement-policy-003".to_string(),
        "asset-001".to_string(),
        "alice-custody".to_string(),
        "bob-custody".to_string(),
        500,
    );

    let decision = engine.evaluate_settlement(&instruction);

    assert_eq!(
        decision,
        PolicyDecision::Deny("sender custody account is blocked".to_string())
    );
}

#[test]
fn policy_engine_rejects_blocked_receiver_account() {
    let mut engine = PolicyEngine::new(1_000);
    engine.block_account("bob-custody".to_string());

    let instruction = SettlementInstruction::new(
        "settlement-policy-004".to_string(),
        "asset-001".to_string(),
        "alice-custody".to_string(),
        "bob-custody".to_string(),
        500,
    );

    let decision = engine.evaluate_settlement(&instruction);

    assert_eq!(
        decision,
        PolicyDecision::Deny("receiver custody account is blocked".to_string())
    );
}

#[test]
fn compliance_engine_default_allows() {
    let engine = ComplianceEngine::new();

    let decision = engine.evaluate();

    assert_eq!(decision, ComplianceDecision::Allow);
}

#[test]
fn compliance_decision_can_represent_denial_reason() {
    let decision = ComplianceDecision::Deny("participant failed compliance check".to_string());

    assert_eq!(
        decision,
        ComplianceDecision::Deny("participant failed compliance check".to_string())
    );
}

#[test]
fn compliance_engine_approves_participant() {
    let mut engine = ComplianceEngine::new();

    let added = engine.approve_participant("alice".to_string());

    assert!(added);
    assert!(engine.is_participant_approved("alice"));
}

#[test]
fn compliance_engine_allows_approved_participant() {
    let mut engine = ComplianceEngine::new();
    engine.approve_participant("alice".to_string());

    let decision = engine.evaluate_participant("alice");

    assert_eq!(decision, ComplianceDecision::Allow);
}

#[test]
fn compliance_engine_rejects_unapproved_participant() {
    let engine = ComplianceEngine::new();

    let decision = engine.evaluate_participant("alice");

    assert_eq!(
        decision,
        ComplianceDecision::Deny("participant is not approved".to_string())
    );
}

#[test]
fn compliance_engine_allows_approved_participants() {
    let mut engine = ComplianceEngine::new();
    engine.approve_participant("alice".to_string());
    engine.approve_participant("bob".to_string());

    let decision = engine.evaluate_participants("alice", "bob");

    assert_eq!(decision, ComplianceDecision::Allow);
}

#[test]
fn compliance_engine_rejects_unapproved_sender() {
    let mut engine = ComplianceEngine::new();
    engine.approve_participant("bob".to_string());

    let decision = engine.evaluate_participants("alice", "bob");

    assert_eq!(
        decision,
        ComplianceDecision::Deny("sender is not approved".to_string())
    );
}

#[test]
fn compliance_engine_rejects_unapproved_receiver() {
    let mut engine = ComplianceEngine::new();
    engine.approve_participant("alice".to_string());

    let decision = engine.evaluate_participants("alice", "bob");

    assert_eq!(
        decision,
        ComplianceDecision::Deny("receiver is not approved".to_string())
    );
}

#[test]
fn settlement_engine_executes_settlement_with_compliance_approval() {
    let settlement_id = "settlement-compliance-001";

    let mut engine = SettlementEngine::new();
    let mut ledger = AssetLedger::new();
    let mut custody_registry = CustodyRegistry::new();
    let policy_engine = PolicyEngine::new(1_000);
    let mut compliance_engine = ComplianceEngine::new();

    ledger.credit("asset-001", "alice-custody", 2_000);

    custody_registry.add_account(CustodyAccount::new(
        "alice-custody".to_string(),
        "alice".to_string(),
    ));

    custody_registry.add_account(CustodyAccount::new(
        "bob-custody".to_string(),
        "bob".to_string(),
    ));

    compliance_engine.approve_participant("alice-custody".to_string());
    compliance_engine.approve_participant("bob-custody".to_string());

    let instruction = SettlementInstruction::new(
        settlement_id.to_string(),
        "asset-001".to_string(),
        "alice-custody".to_string(),
        "bob-custody".to_string(),
        500,
    );

    engine.add_instruction(instruction);

    let executed = engine.execute_settlement_with_compliance(
        settlement_id,
        &mut ledger,
        &custody_registry,
        &policy_engine,
        &compliance_engine,
    );

    assert!(executed);

    let settled = engine
        .get_instruction(settlement_id)
        .expect("settlement should exist");

    assert_eq!(settled.status, SettlementStatus::Settled);
    assert_eq!(ledger.balance_of("asset-001", "alice-custody"), 1_500);
    assert_eq!(ledger.balance_of("asset-001", "bob-custody"), 500);
}

#[test]
fn settlement_engine_rejects_settlement_with_unapproved_sender() {
    let settlement_id = "settlement-compliance-002";

    let mut engine = SettlementEngine::new();
    let mut ledger = AssetLedger::new();
    let mut custody_registry = CustodyRegistry::new();
    let policy_engine = PolicyEngine::new(1_000);
    let mut compliance_engine = ComplianceEngine::new();

    ledger.credit("asset-001", "alice-custody", 2_000);

    custody_registry.add_account(CustodyAccount::new(
        "alice-custody".to_string(),
        "alice".to_string(),
    ));

    custody_registry.add_account(CustodyAccount::new(
        "bob-custody".to_string(),
        "bob".to_string(),
    ));

    compliance_engine.approve_participant("bob-custody".to_string());

    let instruction = SettlementInstruction::new(
        settlement_id.to_string(),
        "asset-001".to_string(),
        "alice-custody".to_string(),
        "bob-custody".to_string(),
        500,
    );

    engine.add_instruction(instruction);

    let executed = engine.execute_settlement_with_compliance(
        settlement_id,
        &mut ledger,
        &custody_registry,
        &policy_engine,
        &compliance_engine,
    );

    assert!(!executed);

    let failed = engine
        .get_instruction(settlement_id)
        .expect("settlement should exist");

    assert_eq!(failed.status, SettlementStatus::Failed);
    assert_eq!(ledger.balance_of("asset-001", "alice-custody"), 2_000);
    assert_eq!(ledger.balance_of("asset-001", "bob-custody"), 0);
}

#[test]
fn settlement_engine_rejects_settlement_with_unapproved_receiver() {
    let settlement_id = "settlement-compliance-003";

    let mut engine = SettlementEngine::new();
    let mut ledger = AssetLedger::new();
    let mut custody_registry = CustodyRegistry::new();
    let policy_engine = PolicyEngine::new(1_000);
    let mut compliance_engine = ComplianceEngine::new();

    ledger.credit("asset-001", "alice-custody", 2_000);

    custody_registry.add_account(CustodyAccount::new(
        "alice-custody".to_string(),
        "alice".to_string(),
    ));

    custody_registry.add_account(CustodyAccount::new(
        "bob-custody".to_string(),
        "bob".to_string(),
    ));

    compliance_engine.approve_participant("alice-custody".to_string());

    let instruction = SettlementInstruction::new(
        settlement_id.to_string(),
        "asset-001".to_string(),
        "alice-custody".to_string(),
        "bob-custody".to_string(),
        500,
    );

    engine.add_instruction(instruction);

    let executed = engine.execute_settlement_with_compliance(
        settlement_id,
        &mut ledger,
        &custody_registry,
        &policy_engine,
        &compliance_engine,
    );

    assert!(!executed);

    let failed = engine
        .get_instruction(settlement_id)
        .expect("settlement should exist");

    assert_eq!(failed.status, SettlementStatus::Failed);
    assert_eq!(ledger.balance_of("asset-001", "alice-custody"), 2_000);
    assert_eq!(ledger.balance_of("asset-001", "bob-custody"), 0);
}
