use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::network_message::NetworkMessage;
use crate::transaction::Transaction;
use crate::tx_output::TxOutput;
use crate::wallet::Wallet;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Sends a network message asynchronously using Tokio TCP.
pub async fn send_async_message(
    address: &str,
    message: &NetworkMessage,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let json = serde_json::to_string(message)?;

    let mut stream = TcpStream::connect(address).await?;

    stream.write_all(json.as_bytes()).await?;

    Ok(())
}

/// Starts an asynchronous TCP node using Tokio.
pub async fn start_async_node(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;

    println!("Async node listening on {port}");

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            match read_message(&mut stream).await {
                Ok(NetworkMessage::Block(block)) => {
                    println!("Async received block {}", block.index);
                }
                Ok(NetworkMessage::ChainRequest) => {
                    println!("Async received chain request");

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

                    let response = NetworkMessage::ChainResponse(blockchain);

                    if let Err(error) = write_message(&mut stream, &response).await {
                        eprintln!("Failed to send async chain response: {error}");
                    }
                }
                Ok(NetworkMessage::ChainResponse(blockchain)) => {
                    println!(
                        "Async received chain response with {} blocks",
                        blockchain.chain.len()
                    );
                }
                Err(error) => {
                    eprintln!("Failed to parse async message: {error}");
                }
            }
        });
    }
}

/// Reads and deserialized a network message from an async TCP stream.
pub async fn read_message(
    stream: &mut TcpStream,
) -> Result<NetworkMessage, Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer).await?;

    let message = serde_json::from_str(&buffer)?;

    Ok(message)
}

/// Writes and a serialized a network message to an async TCP stream.
pub async fn write_message(
    stream: &mut TcpStream,
    message: &NetworkMessage,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let json = serde_json::to_string(message)?;

    stream.write_all(json.as_bytes()).await?;

    Ok(())
}

/// Sends a block asynchronously using the network message protocol.
pub async fn send_async_block(
    address: &str,
    block: &Block,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let message = NetworkMessage::Block(block.clone());

    send_async_message(address, &message).await?;

    println!("Sent async block {} to {address}", block.index);

    Ok(())
}

/// Requests a blockchain from a peer asynchronously.
pub async fn send_async_chain_request(
    address: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut stream = TcpStream::connect(address).await?;

    let request = NetworkMessage::ChainRequest;

    let json = serde_json::to_string(&request)?;

    stream.write_all(json.as_bytes()).await?;

    stream.shutdown().await?;

    let response = read_message(&mut stream).await?;

    match response {
        NetworkMessage::ChainResponse(blockchain) => {
            println!(
                "Async received chain response with {} blocks",
                blockchain.chain.len()
            );
        }
        other => {
            println!("Unexpectd response: {other:?}");
        }
    }

    Ok(())
}
