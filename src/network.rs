use crate::block::Block;
use crate::network_message::NetworkMessage;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn start_node(port: u16) {
    // Listen for incoming TCP connections and validate received blocks.
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    println!("Node listening on {port}");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = String::new();

        stream.read_to_string(&mut buffer).unwrap();

        let message: NetworkMessage = serde_json::from_str(&buffer).unwrap();

        match message {
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

pub fn send_block(address: &str, block: &Block) {
    // Serialize and transmit a block to a peer node.
    let message = NetworkMessage::Block(block.clone());

    send_message(address, &message);

    println!("Sent block {} to {}", block.index, address);
}

pub fn send_message(address: &str, message: &NetworkMessage) {
    // Serialize and transmit a message to a peer node.
    let json = serde_json::to_string(message).unwrap();

    let mut stream = TcpStream::connect(address).unwrap();

    stream.write_all(json.as_bytes()).unwrap();

    println!("Sent network message to {address}");
}

pub fn send_chain_request(address: &str) {
    //Sends a blockchain synchronization request to a peer node.
    //The receiving node may respond with its current blockchain state.
    let message = NetworkMessage::ChainRequest;

    send_message(address, &message);

    println!("Sent chain request to {address}");
}
