use crate::block::Block;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn start_node(port: u16) {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    println!("Node listening on {port}");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = String::new();

        stream.read_to_string(&mut buffer).unwrap();

        let block: Block = serde_json::from_str(&buffer).unwrap();

        println!("Received block {}", block.index);
        println!("Hash: {}", block.hash);
        println!("Previous hash: {}", block.previous_hash);
        println!("Merkle root: {}", block.merkle_root);
        println!("Transactions: {}", block.transactions.len());
    }
}

pub fn send_block(address: &str, block: &Block) {
    let mut stream = TcpStream::connect(address).unwrap();

    let json = serde_json::to_string(block).unwrap();

    stream.write_all(json.as_bytes()).unwrap();

    println!("Sent block {} to {}", block.index, address);
}
