# Digital Asset Ledger

*A Rust-based research and engineering prototype for blockchain infrastructure, distributed systems, and enterprise DLT architecture.*

## Overview

Digital Asset Ledger is a Rust implementation of core blockchain and distributed-ledger components developed from first principles. The project currently includes proof-of-work blocks, Merkle root construction, Ed25519 signatures, a Bitcoin-style UTXO transaction model, TCP-based block propagation, and chain validation.

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

## Current Features

Current release: `v2.1.0`

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

### Networking

- [x] TCP networking and block propagation
- [x] Validation of received blocks before acceptance

### Engineering

- [x] Modular Rust project structure
- [x] Unit and integration tests
- [x] Rustdoc documentation

### In Progress

- [ ] Chain synchronization (`v3.0`)
- [ ] Persistent storage (`v4.0`)
- [ ] Transaction mempool (`v5.0`)
- [ ] Tokio async networking (`v6.0`)

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

Chain validation currently checks:

- Block hashes are reproducible from block contents
- Each block satisfies the configured proof-of-work target
- `previous_hash` links match the preceding block
- Transaction signatures verify against current transaction contents
- Referenced UTXOs exist before they are spent
- Inputs belong to the signing public key
- Input totals cover output totals
- Double-spend attempts are rejected by rebuilding UTXO state in chain order

This validation runs when the blockchain is inspected locally and when received blocks are checked by the networking layer.

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
v3.0 Chain Synchronization
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
        ✓ v1.0 Blockchain Fundamentals
        ✓ v1.1 TCP Networking
        ✓ v2.0 Bitcoin-style UTXO Model
June 12 ✓ v2.1 Documentation & Rustdoc

        Core Platform
        ─────────────
June    v3.0 Chain Synchronization
            ↓
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

- No peer chain synchronization yet (`v3.0`)
- No persistent blockchain storage yet (`v4.0`)
- No transaction mempool yet (`v5.0`)
- No async Tokio networking yet (`v6.0`)
- No permissioned network controls yet
- No asset tokenization or settlement workflows yet
- No custody or compliance layers yet
- No smart contracts or advanced consensus features yet

### Prototype Simplifications

- Coinbase-style transactions are currently represented as transactions with no inputs
- Networking validates received blocks but does not yet merge or reconcile competing chains
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

## Repository Structure

- `src/block.rs` - proof-of-work block type and block-level validation
- `src/blockchain.rs` - blockchain container and chain-wide validation
- `src/merkle.rs` - Merkle hashing helpers
- `src/network.rs` - TCP block send/receive helpers
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

Digital Asset Ledger is an incremental research and engineering prototype implemented in Rust to demonstrate core blockchain and distributed-systems design.

The current implementation is appropriate for architectural review and technical discussion, but it is not production-ready. Roadmap items such as chain synchronization, persistence, mempool management, and asynchronous networking are planned for subsequent releases.

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
- Graduate coursework at NYU Tandon School of Engineering, including blockchain, cryptography, privacy, machine learning, and big data studies

Any errors, omissions, simplifications, or design decisions remain solely the responsibility of the author.

## License

Released under the MIT License.
