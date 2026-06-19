/// Proof-of-work block type and block-level validation.
pub mod block;
/// Blockchain container and chain-wide validation rules.
pub mod blockchain;
/// Merkle tree hashing helpers for block transaction roots.
pub mod merkle;
/// Simple TCP networking helpers for sending and receiving blocks.
pub mod network;
/// UTXO-style transaction creation, signing, and verification.
pub mod transaction;
/// Transaction inputs that reference previously created outputs.
pub mod tx_input;
/// Transaction outputs representing spendable value.
pub mod tx_output;
/// In-memory set of currently unspent transaction outputs.
pub mod utxo;
/// Wallet keypair generation and public key formatting.
pub mod wallet;
/// Defines the network protocol used for communication and synchronization between blockchain nodes.
pub mod network_message;
