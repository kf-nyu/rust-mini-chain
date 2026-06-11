# Rust Mini Chain

A simple prototype blockchain implementation written in Rust.

This project was developed to explore the fundamental building blocks of blockchain systems while deepening practical experience with Rust. The implementation focuses on cryptographic integrity, transaction validation, Merkle trees, proof-of-work mining, and blockchain verification.

---

## Features

* Block and blockchain data structures
* SHA-256 hashing
* Proof-of-Work (PoW) mining
* Ed25519 public/private key generation
* Digital signatures and signature verification
* Signed transactions
* Merkle root calculation
* Blockchain integrity validation
* Tamper-detection tests
* TCP networking and block propagation
* Validation of received blocks before acceptance
* Modular Rust project structure
---

## Recent Updates

### v2.0

- Refactored transactions from account-style transfers to an input-output model
- Added TxInput and TxOutput types
- Added UTXO set tracking
- Added UTXO spend validation
- Added ownership checks for spent outputs
- Added change-output support
- Added double-spend detection tests
- Updated demo flow to use coinbase and UTXO-based transactions

### v1.1

- Added validation of received blocks over TCP
- Serialized transaction signatures for network transmission
- Rejects invalid or tampered blocks before acceptance

---

## Architecture

```text
Wallet
  ↓ signs
Transaction
  ↓ included in
Block
  ↓ linked by previous_hash
Blockchain
```

### Block Structure

```text
Block
├── index
├── timestamp
├── transactions
├── merkle_root
├── previous_hash
├── nonce
└── hash
```

### Merkle Tree

```text
           Merkle Root
                │
        ┌───────┴───────┐
        │               │
      H12             H34
      │ │             │ │
      H1 H2           H3 H4
```

Transactions are hashed and combined recursively until a single Merkle Root remains. The Merkle Root is included in the block header and contributes to the block hash.

---

## Current Capabilities

### Wallets

* Generates Ed25519 key pairs
* Exports public keys as hexadecimal strings
* Signs transactions using private keys
* Verifies signatures using public keys

### Transactions

* Sender
* Receiver
* Amount
* Sender public key
* Digital signature

### Mining

Blocks are mined using a simplified Proof-of-Work algorithm.

A block is considered valid when:

```text
SHA256(block_header)
```

produces a hash beginning with a configurable number of leading zeros.

Example:

```text
0000be7f1a91fe86d564fe3a42b7fceb...
```

---

## Validation

The blockchain validation process verifies:

* Block hashes
* Previous block references
* Proof-of-Work requirements
* Transaction signatures
* Chain integrity

The test suite includes tampering scenarios such as:

* Modified transaction amounts
* Modified block hashes
* Modified previous hashes

---

## Running

Build and run:

```bash
cargo run --release
```

---

## Testing

Run all tests:

```bash
cargo test
```

Current integration tests verify:

* Valid blockchain acceptance
* Transaction tampering detection
* Previous hash tampering detection
* Block hash tampering detection

---

## Prototype Scope

The prototype now supports a simplified Bitcoin-style UTXO model, but it does not yet implement full peer-to-peer chain synchronization, persistent storage, mempools, mining rewards, or production-grade consensus.

This repository is intentionally designed as a prototype blockchain implementation.

The goal is to demonstrate core concepts including:

* Block construction
* Cryptographic hashing
* Proof-of-Work mining
* Public-key cryptography
* Digital signatures
* Merkle trees
* Blockchain validation
* Rust ownership and type safety

This project is **not intended for production use** and does not currently implement:

* Peer-to-peer networking protocols
* Peer discovery
* Multi-node consensus
* Mempools
* Persistent storage
* Smart contracts
* Account state management
* Economic incentives or mining rewards

---

## Technologies

* Rust
* SHA-256
* Ed25519
* Serde
* Chrono

---

## References

1. Ralph C. Merkle, *Protocols for Public Key Cryptosystems*, Proceedings of the 1980 IEEE Symposium on Security and Privacy, 1980.

2. Satoshi Nakamoto, *Bitcoin: A Peer-to-Peer Electronic Cash System*, 2008.

3. Andreas M. Antonopoulos, *Mastering Bitcoin: Programming the Open Blockchain*, O'Reilly Media.

4. Steve Klabnik and Carol Nichols, *The Rust Programming Language*, No Starch Press.

---

## Acknowledgments

This project was developed as a personal learning exercise while studying Rust, cryptography, distributed systems, and blockchain technologies.

The implementation draws inspiration from:

* Ralph Merkle's work on authenticated data structures and Merkle trees.
* Bitcoin's Proof-of-Work blockchain architecture.
* The Rust open-source ecosystem and community.
* Graduate coursework at NYU Tandon School of Engineering, including Blockchain, Cryptography, Privacy, and Big Data studies.

Any errors, omissions, or simplifications are solely the responsibility of the author.

---

## License

Released under the MIT License.
