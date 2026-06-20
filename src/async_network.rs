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

///Starts an asynchronous TCP node using Tokio.
pub async fn start_async_node(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;

    println!("Async node listening on {port}");

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = String::new();

            if let Err(error) = stream.read_to_string(&mut buffer).await {
                eprintln!("Failed to read async message: {error}");
                return;
            }

            let message: Result<NetworkMessage, _> = serde_json::from_str(&buffer);

            match message {
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
