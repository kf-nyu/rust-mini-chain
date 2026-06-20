use crate::block::Block;
use crate::network_message::NetworkMessage;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Sends a network message asynchronously using Tokio TCP.
pub async fn send_async_message(
    address: &str,
    message: &NetworkMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(message)?;

    let mut stream = TcpStream::connect(address).await?;

    stream.write_all(json.as_bytes()).await?;

    Ok(())
}

/// Starts an asynchronous TCP node using Tokio.
pub async fn start_async_node(port: u16) -> Result<(), Box<dyn std::error::Error>> {
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
) -> Result<NetworkMessage, Box<dyn std::error::Error>> {
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer).await?;

    let message = serde_json::from_str(&buffer)?;

    Ok(message)
}

/// Sends a block asynchronously using the network message protocol.
pub async fn send_async_block(
    address: &str,
    block: &Block,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = NetworkMessage::Block(block.clone());

    send_async_message(address, &message).await?;

    println!("Sent async block {} to {address}", block.index);

    Ok(())
}
