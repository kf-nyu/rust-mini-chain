use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::network_message::NetworkMessage;
use crate::transaction::Transaction;
use crate::tx_output::TxOutput;
use crate::wallet::Wallet;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

/// Listen for incoming TCP connections and validate received blocks.
pub fn start_node(port: u16) {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    println!("Node listening on {port}");

    let mut blockchain = Blockchain::new(4);

    let alice = Wallet::new();

    let coinbase = Transaction::new(
        vec![],
        vec![TxOutput {
            recipient: alice.public_key_hex(),
            amount: 50,
        }],
    );

    blockchain.add_block(vec![coinbase]);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = String::new();

        stream.read_to_string(&mut buffer).unwrap();

        let message: NetworkMessage = serde_json::from_str(&buffer).unwrap();

        match message {
            NetworkMessage::Hello(identity) => {
                println!(
                    "Received hello from node {} with role {:?}",
                    identity.node_id, identity.role
                );
            }

            NetworkMessage::Block(block) => {
                println!("Received block {}", block.index);
                println!("Hash: {}", block.hash);
                println!("Previous hash: {}", block.previous_hash);
                println!("Merkle root: {}", block.merkle_root);
                println!("Transactions: {}", block.transactions.len());

                let difficulty = 4;

                if block.is_valid(difficulty) {
                    println!("Block validation: accepted");
                } else {
                    println!("Block validation: rejected");
                }
            }

            NetworkMessage::ChainRequest => {
                println!("Received chain request");

                let response = NetworkMessage::ChainResponse(blockchain.clone());
                let json = serde_json::to_string(&response).unwrap();

                stream.write_all(json.as_bytes()).unwrap();

                println!("Sent chain response with {} blocks", blockchain.chain.len());
            }

            NetworkMessage::ChainResponse(blockchain) => {
                println!(
                    "Received chain response with {} blocks",
                    blockchain.chain.len()
                );
            }
        }
    }
}

/// Serialize and transmit a block to a peer node.
pub fn send_block(address: &str, block: &Block) {
    let message = NetworkMessage::Block(block.clone());

    send_message(address, &message);

    println!("Sent block {} to {}", block.index, address);
}

/// Serialize and transmit a message to a peer node.
pub fn send_message(address: &str, message: &NetworkMessage) {
    let json = serde_json::to_string(message).unwrap();

    let mut stream = TcpStream::connect(address).unwrap();

    stream.write_all(json.as_bytes()).unwrap();

    println!("Sent network message to {address}");
}

/// Sends a blockchain synchronization request to a peer node.
/// If the peer returns a valid longer chain, the lcaol demo chain is replaced.
pub fn send_chain_request(address: &str) {
    let message = NetworkMessage::ChainRequest;
    let json = serde_json::to_string(&message).unwrap();

    let mut stream = TcpStream::connect(address).unwrap();

    stream.write_all(json.as_bytes()).unwrap();
    stream.shutdown(Shutdown::Write).unwrap();

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();

    let response: NetworkMessage = serde_json::from_str(&buffer).unwrap();

    match response {
        NetworkMessage::ChainResponse(blockchain) => {
            println!(
                "Received chain response with {} blocks",
                blockchain.chain.len()
            );

            let mut local_chain = Blockchain::new(blockchain.difficulty);

            println!(
                "Local chain before sync: {} blocks",
                local_chain.chain.len()
            );

            if local_chain.replace_chain_if_longer(blockchain) {
                println!("Local chain after sync: {} blocks", local_chain.chain.len());
                println!("Chain synchronization: accepted");
            } else {
                println!("Chain synchronization: rejected");
            }
        }
        other => {
            println!("Unexpected response: {other:?}");
        }
    }
}
