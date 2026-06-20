use crate::network_message::NetworkMessage;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

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
