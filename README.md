# Digital Asset Ledger

*A Rust-based research and engineering prototype for blockchain infrastructure, distributed systems, and enterprise DLT architecture.*

## Overview

Digital Asset Ledger is a Rust implementation of core blockchain and distributed-ledger components developed from first principles. The project currently includes proof-of-work blocks, Merkle root construction, Ed25519 signatures, a Bitcoin-style UTXO transaction model, TCP-based networking, block propagation, chain synchronization, and blockchain validation.

The system is designed as an incremental prototype platform: each release introduces a focused capability while preserving clarity around architecture, validation rules, and engineering trade-offs. This approach supports both technical depth and extensibility as the project evolves.

Beyond public-blockchain mechanics, the roadmap extends toward enterprise Digital Asset and Distributed Ledger Technology (DLT) use cases, including permissioned networks, asset tokenization, settlement workflows, custody controls, and compliance-oriented infrastructure.

Rust was selected for its memory safety, strong type system, concurrency model, and systems-level performance, all of which are well aligned with security-sensitive distributed systems and digital asset infrastructure.

## Quick Start

### Run the local demo

```bash
cargo run --release
```

### Run tests

```bash
cargo test
```

### Generate Rust documentation

```bash
cargo doc --open
```

### Start a node

```bash
cargo run -- node 6000
```

### Send a sample block to a node

```bash
cargo run -- send 127.0.0.1:6000
```

### Send a chain synchronization request

```bash
cargo run -- request 127.0.0.1:6000
```

## Current Features

Current release: `v3.0.0`

### Blockchain Core

- [x] Block and blockchain data structures
- [x] SHA-256 hashing
- [x] Proof-of-Work (PoW) mining
- [x] Merkle root calculation
- [x] Blockchain integrity validation

### Cryptography

- [x] Ed25519 keypair generation
- [x] Digital signatures and signature verification
- [x] Transaction authentication

### Transaction Processing

- [x] Bitcoin-style UTXO transaction model
- [x] Transaction inputs and outputs
- [x] UTXO ownership validation
- [x] Change-output generation
- [x] Double-spend detection

### Networking & Synchronization

- [x] TCP networking and block propagation
- [x] Validation of received blocks before acceptance
- [x] Network message protocol
- [x] ChainRequest / ChainResponse messaging
- [x] Peer chain validation
- [x] Longest-chain replacement rule
- [x] Chain synchronization CLI demo

### Engineering

- [x] Modular Rust project structure
- [x] Unit and integration tests
- [x] Rustdoc documentation

### Planned

- [ ] Persistent storage (`v4.0`)
- [ ] Transaction mempool (`v5.0`)
- [ ] Tokio async networking (`v6.0`)

### Future Track A: Enterprise DLT

- [ ] Permissioned network
- [ ] Asset tokenization
- [ ] Settlement engine
- [ ] Custody controls
- [ ] Compliance layer
- [ ] Audit & reporting

### Future Track B: Public Blockchain

- [ ] Difficulty adjustment
- [ ] Mining rewards and fees
- [ ] Fork handling
- [ ] Smart contracts
- [ ] Light clients (SPV)

## Architecture

### High-Level Flow

```text
Wallet
  ↓ signs
Transaction
  ↓ included in
Block
  ↓ linked by previous_hash
Blockchain
```

Wallets create Ed25519 keypairs, transactions spend prior outputs into new outputs, blocks package transactions with a Merkle root, and the blockchain links blocks through hashes and proof of work.

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

Transactions are hashed and combined recursively until a single Merkle root remains. That root is stored in the block header and contributes to the block hash.

## Validation Model

Blockchain validation currently checks:

- Block hashes are reproducible from block contents
- Each block satisfies the configured proof-of-work target
- `previous_hash` references match the preceding block
- Transaction signatures verify against current transaction contents
- Referenced UTXOs exist before they are spent
- Inputs belong to the signing public key
- Input totals cover output totals
- Double-spend attempts are rejected by rebuilding UTXO state in chain order

Chain synchronization validation additionally checks:

- Received peer chains pass full blockchain validation
- Invalid chains are rejected before synchronization
- Only valid longer chains may replace the local chain
- Shorter chains are rejected by the longest-chain rule

Validation is performed during:

- Local blockchain verification
- Received block verification over the network
- Peer chain synchronization requests and responses

## Roadmap

### Version Roadmap

```text
v1.0 ✓ Blockchain Fundamentals
        │
v1.1 ✓ TCP Networking
        │
v2.0 ✓ UTXO Model
        │
v2.1 ✓ Documentation
        │
        ▼
v3.0 ✓ Chain Synchronization
        │
v4.0 Persistence
        │
v5.0 Transaction Mempool
        │
v6.0 Tokio Async Networking
        │
        ├─────────────────────┐
        │                     │
        ▼                     ▼

Track A                 Track B
Enterprise DLT          Public Blockchain

v7A Permissioned        v7B Difficulty
    Network                 Adjustment

v8A Asset               v8B Rewards
    Tokenization            & Fees

v9A Settlement          v9B Fork
    Engine                  Handling

v10A Custody            v10B Smart
     Controls               Contracts

v11A Compliance         v11B Light
     Layer                  Clients

v12A Audit &
     Reporting
```

### Release Timeline

```text
2026
        Completed
        ─────────────────
June    ✓ v1.0 Blockchain Fundamentals
        ✓ v1.1 TCP Networking
        ✓ v2.0 Bitcoin-style UTXO Model
        ✓ v2.1 Documentation & Rustdoc
        ✓ v3.0 Chain Synchronization

        Core Platform
        ─────────────
        v4.0 Persistence
            ↓
        v5.0 Transaction Mempool
            ↓
July    v6.0 Tokio Async Networking

        Track A: Enterprise DLT (Primary)
        ─────────────────────────────────
        v7A Permissioned Network
            ↓
        v8A Asset Tokenization
            ↓
        v9A Settlement Engine
            ↓
        v10A Custody Controls
            ↓
        v11A Compliance Layer
            ↓
August  v12A Audit & Reporting

        Track B: Public Blockchain (Secondary)
        ──────────────────────────────────────
        v7B Difficulty Adjustment
            ↓
        v8B Mining Rewards & Fees
            ↓
        v9B Fork Handling
            ↓
        v10B Smart Contracts
            ↓
Sep     v11B Light Clients (SPV)
```

## Limitations

This implementation is still evolving. The current codebase intentionally omits several systems that would be required for a fuller blockchain or enterprise ledger platform.

### Current Gaps

- No persistent blockchain storage yet (`v4.0`)
- No transaction mempool yet (`v5.0`)
- No asynchronous networking via Tokio yet (`v6.0`)
- No permissioned network controls yet
- No asset tokenization or settlement workflows yet
- No custody or compliance layers yet
- No smart contracts or advanced consensus features yet

### Prototype Simplifications

- Coinbase-style transactions are currently represented as transactions with no inputs
- Chain synchronization currently uses a simplified longest-chain replacement model
- Peer discovery and automatic synchronization are not yet implemented
- Proof-of-work difficulty is treated as a simple configurable prefix target
- The wallet is in-memory only and does not persist keys

## Testing

Run the full test suite with:

```bash
cargo test
```

Current tests cover:

### Blockchain Validation

- Valid blockchain acceptance
- Block hash tampering detection
- Previous hash tampering detection
- Transaction tampering detection

### UTXO Validation

- Valid spend acceptance
- Ownership enforcement
- Overspending rejection
- Non-existent input rejection
- Double-spend detection

### Transaction Processing

- Change-output generation
- Change-output spending
- UTXO balance tracking

### Chain Synchronization

- Valid longer chain acceptance
- Shorter chain rejection
- Invalid chain rejection

## Repository Structure

- `src/block.rs` - proof-of-work block type and block-level validation
- `src/blockchain.rs` - blockchain container and chain-wide validation
- `src/merkle.rs` - Merkle hashing helpers
- `src/network_message.rs` - protocol messages exchanged between peers
- `src/network.rs` - TCP networking, chain requests, responses, and synchronization
- `src/transaction.rs` - UTXO transaction creation, signing, and verification
- `src/tx_input.rs` - transaction inputs that reference prior outputs
- `src/tx_output.rs` - spendable transaction outputs
- `src/utxo.rs` - in-memory unspent output tracking and validation
- `src/wallet.rs` - Ed25519 wallet/keypair handling
- `tests/blockchain_tests.rs` - integration coverage for blockchain and UTXO behavior

## Technologies

### Language

- Rust

### Cryptography

- SHA-256
- Ed25519

### Serialization

- Serde
- Serde JSON

### Date and Time

- Chrono

### Networking

- TCP sockets via `std::net`

### Testing

- Rust unit tests
- Rust integration tests

## Implementation Status

Digital Asset Ledger is an incremental research and engineering prototype implemented in Rust to explore blockchain, distributed systems, cybersecurity, and digital asset infrastructure concepts.

The current implementation includes:

- Proof-of-Work blockchain validation
- Ed25519 wallet and transaction authentication
- Bitcoin-style UTXO transaction processing
- Merkle tree verification
- TCP peer-to-peer networking
- Peer chain synchronization
- Longest-chain replacement

The project is suitable for architectural review, technical discussion, and continued engineering development, but it is not intended for production deployment.

Future releases will focus on persistent storage, transaction mempool management, asynchronous networking with Tokio, and enterprise DLT capabilities including permissioned networks, asset tokenization, settlement workflows, custody controls, and compliance frameworks.

## References

### Blockchain Foundations

1. Ralph C. Merkle, *Protocols for Public Key Cryptosystems*, Proceedings of the IEEE Symposium on Security and Privacy, 1980.
2. Satoshi Nakamoto, *Bitcoin: A Peer-to-Peer Electronic Cash System*, 2008.
3. Andreas M. Antonopoulos, *Mastering Bitcoin: Programming the Open Blockchain*, O'Reilly Media.

### Distributed Systems

4. Martin Kleppmann, *Designing Data-Intensive Applications*, O'Reilly Media.
5. Andrew S. Tanenbaum and Maarten van Steen, *Distributed Systems: Principles and Paradigms*.

### Rust

6. Steve Klabnik and Carol Nichols, *The Rust Programming Language*, No Starch Press.
7. Jon Gjengset, *Rust for Rustaceans*, No Starch Press.

### Enterprise Digital Assets and DLT

8. The Linux Foundation, *Hyperledger Fabric Documentation*.
9. Digital Asset, *Canton Architecture and Synchronizer Protocol Documentation*.
10. DTCC, *The Digital Asset Securities Control Principles (DASCP)*.
11. BIS (Bank for International Settlements), *Project Agora*, *Project Helvetia*, and related reports on tokenized financial market infrastructure.

### Cryptography

12. Jonathan Katz and Yehuda Lindell, *Introduction to Modern Cryptography*.
13. Christof Paar and Jan Pelzl, *Understanding Cryptography*.

## Acknowledgments

This project was developed as an independent research and engineering effort focused on blockchain systems, distributed ledgers, digital asset infrastructure, cryptography, and systems programming in Rust.

The implementation draws inspiration from:

- Ralph Merkle's pioneering work on authenticated data structures and Merkle trees
- Bitcoin's UTXO model, proof-of-work consensus mechanism, and decentralized ledger architecture
- The Rust open-source ecosystem and community
- Research and industry developments in Distributed Ledger Technology (DLT), digital assets, custody, settlement, and tokenization
- Graduate coursework at NYU Tandon School of Engineering, including blockchain, operating systems, cryptography, privacy, machine/deep learning, big data and application security

Any errors, omissions, simplifications, or design decisions remain solely the responsibility of the author.

## License

Released under the MIT License.
