# Digital Asset Ledger

![Rust](https://img.shields.io/badge/Rust-1.88+-orange)
![Version](https://img.shields.io/badge/version-v11.0.0-success)
![License](https://img.shields.io/badge/license-MIT-blue)
![Tests](https://img.shields.io/badge/tests-83%20passing-brightgreen)

*A Rust-based research and engineering prototype for blockchain infrastructure, distributed systems, and enterprise digital asset architecture.*

## Overview

Digital Asset Ledger is a Rust implementation of core blockchain and distributed-ledger components developed from first principles. The project currently includes proof-of-work blocks, Merkle root construction, Ed25519 signatures, a Bitcoin-style UTXO transaction model, TCP-based networking, block propagation, chain synchronization, permissioned networking, asset tokenization, settlement processing, custody controls, and enterprise policy enforcement.

The system is designed as an incremental prototype platform: each release introduces a focused capability while preserving clarity around architecture, validation rules, and engineering trade-offs. This approach supports both technical depth and extensibility as the project evolves.

Beyond public-blockchain mechanics, the roadmap extends toward enterprise digital asset and distributed ledger (DLT) use cases, including permissioned networks, asset tokenization, settlement processing, custody controls, and compliance-oriented infrastructure.

Rust was selected for its memory safety, strong type system, concurrency model, and systems-level performance, all of which are well aligned with security-sensitive distributed systems and digital asset infrastructure.

## Project Philosophy

This project is intentionally developed as an incremental engineering exercise. Each release introduces a focused capability while preserving code quality, testing, and documentation.

Rather than implementing a complete blockchain at once, each version builds on previous functionality to demonstrate architectural evolution, systems programming techniques, and enterprise distributed ledger design.

## Quick Start

### Core

```bash
cargo test
cargo run --release
cargo doc --open
cargo run -- mempool-demo
```

### Networking

```bash
cargo run -- node 6000
cargo run -- send 127.0.0.1:6000
cargo run -- request 127.0.0.1:6000
cargo run -- async-demo 127.0.0.1:7000
```

### Enterprise Digital Assets

```bash
cargo run -- permissioned-demo
cargo run -- asset-demo
cargo run -- settlement-demo
cargo run -- custody-demo
cargo run -- policy-demo
```

## Project Metrics

Current release: `v11.0.0`

The project is developed incrementally, with each release introducing a production-inspired capability while maintaining full test coverage and backward compatibility.

- ~4,100+ lines of Rust
- 20 Rust modules
- 83 integration tests
- 11 CLI demonstrations
- GitHub Actions CI workflow
- Modular architecture

## Implemented Features

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
- [x] Tokio asynchronous networking
- [x] Asynchronous block propagation
- [x] Asynchronous chain request/response
- [x] Permissioned node identity
- [x] Trusted peer registry
- [x] Permissioned handshake protocol
- [x] Chain synchronization CLI demo
- [x] Permissioned network CLI demo

### Persistence

- [x] JSON-based blockchain persistence
- [x] Save blockchain state to disk
- [x] Load blockchain state from disk
- [x] Validate loaded blockchain state

### Mempool

- [x] In-memory pending transaction pool
- [x] Transaction insertion with signature validation
- [x] Invalid transaction rejection
- [x] Duplicate transaction rejection
- [x] Transaction selection for block mining
- [x] Removal of mined transactions
- [x] Mempool lifecycle CLI demo

### Permissioned Networking

Permissioned networking means that nodes do not accept every peer by default. Each peer presents a node identity during the handshake, and the receiving node checks that identity against a trusted peer registry before accepting the connection.

- [x] Node identity abstraction
- [x] Validator, Observer, and Admin roles
- [x] Trusted peer registry
- [x] Hello(NodeIdentity) protocol message
- [x] Permissioned handshake validation
- [x] Trusted peer acceptance
- [x] Untrusted peer rejection

### Asset Tokenization

- [x] Fungible and non-fungible asset model
- [x] Asset issuance model
- [x] Asset ownership record
- [x] Asset transfer model
- [x] In-memory asset balance ledger
- [x] Asset issuance applied to ledger
- [x] Valid asset transfer processing
- [x] Insufficient-balance transfer rejection
- [x] Asset tokenization CLI demo

### Settlement Engine

- [x] Settlement instruction model
- [x] Pending, settled, and failed settlement states
- [x] In-memory settlement engine
- [x] Duplicate settlement rejection
- [x] Settlement execution against asset ledger
- [x] Pending settlement batch execution
- [x] Settlement status counts
- [x] Pending, settled, and failed settlement queries
- [x] Settlement CLI demo

### Custody Controls

- [x] Custody account model
- [x] Active, frozen, and closed custody account states
- [x] Custody account registry
- [x] Duplicate custody account rejection
- [x] Custody account freeze and close operations
- [x] Custody account status queries
- [x] Custody account status counts
- [x] Custody-aware settlement execution
- [x] Settlement rejection for frozen, closed, or missing custody accounts
- [x] Custody controls CLI demo

### Policy Engine

- [x] Policy decision model
- [x] Policy-aware settlement execution
- [x] Settlement quantity limit policy
- [x] Blocked custody account policy
- [x] Policy engine CLI demo

### Software Engineering

- [x] Modular Rust project structure
- [x] Unit and integration tests
- [x] Rustdoc documentation

### Enterprise Digital Assets Roadmap

- [x] Permissioned network
- [x] Asset tokenization
- [x] Settlement engine
- [x] Custody controls
- [x] Policy engine
- [ ] Compliance layer
- [ ] Audit & reporting

### Public Blockchain Roadmap

- [ ] Difficulty adjustment
- [ ] Mining rewards and fees
- [ ] Fork handling
- [ ] Smart contracts
- [ ] Light clients (SPV)

## Architecture

### System Architecture

```text
                        ┌──────────────────┐
                        │      Wallet      │
                        └──────────────────┘
                                 │ 
                           signs transactions
                                 │ 
                                 ▼
                     ┌───────────────────────┐
                     │   Transaction (UTXO)  │
                     └───────────────────────┘
                                 │ 
                                 ▼
                     ┌───────────────────────┐
                     │       Mempool         │
                     └───────────────────────┘
                                 │ 
                                 ▼
                     ┌───────────────────────┐
                     │        Mining         │
                     └───────────────────────┘
                                 │ 
                                 ▼
                     ┌───────────────────────┐
                     │      Blockchain       │
                     └───────────────────────┘
                         │              │ 
           Persistence   │              │  Networking
                         ▼              ▼
                 ┌──────────┐  ┌──────────────────┐
                 │ Storage  │  │ Permissioned P2P │
                 └──────────┘  └──────────────────┘
                                        │ 
                                        ▼
                             ┌──────────────────────┐
                             │     Asset Layer      │
                             ├──────────────────────┤
                             │ ├── Tokenization     │
                             │ ├── Settlement       │
                             │ │   ├── Custody      │
                             │ │   └── Policy       │
                             │ └── Future           │
                             │     ├── Compliance   │
                             │     └── Audit        │
                             └──────────────────────┘
```

Wallets create Ed25519 keypairs and sign UTXO transactions, valid transactions enter the mempool, mining packages them into blocks, and the blockchain links blocks through hashes and proof of work. Persistence stores chain state, permissioned networking synchronizes trusted peers, and the asset layer models issuance, ownership, balances, transfers, custody controls, policy authorization, and settlement execution.

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
Transactions in a block

  Tx1           Tx2           Tx3           Tx4
   │             │             │             │
 H(Tx1)        H(Tx2)        H(Tx3)        H(Tx4)
   └──── H12 ────┘             └──── H34 ────┘
          └─────── Merkle Root ───────┘
                         │
                  stored in block
```

Each block hashes its transactions into a Merkle root before mining. The root is stored on the block and included in the block hash, so transaction changes are detected during block and chain validation.

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

Permissioned networking additionally validates node identity against the trusted peer registry before accepting participation in the network.

Settlement execution first evaluates enterprise policy rules before custody validation. Current policy rules include settlement quantity limits and blocked custody accounts. Successful policy validation is followed by custody validation before asset balances are updated.

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
v3.0 ✓ Chain Synchronization
        │
v4.0 ✓ Persistence
        │
v5.0 ✓ Transaction Mempool
        │
v6.0 ✓ Tokio Async Networking
        │
        ├──────────────────────┐
        │                      │
        ▼                      ▼
Enterprise Digital   Public Blockchain Track
    Assets Track

v7A ✓ Permissioned      v7B Difficulty
    Network                 Adjustment

v8A ✓ Asset             v8B Rewards
    Tokenization &          & Fees
    Ledger

v9A ✓ Settlement        v9B Fork
    Engine                  Handling

v10A ✓ Custody          v10B Smart
     Controls               Contracts

v11A ✓ Policy          v11B Light
     Engine                Clients

v12A Compliance
     Layer

v13A Audit &
     Reporting
```

### Release Timeline

```text
2026
        Core Platform Completed
        ─────────────────
June    ✓ v1.0 Blockchain Fundamentals
        ✓ v1.1 TCP Networking
        ✓ v2.0 Bitcoin-style UTXO Model
        ✓ v2.1 Documentation & Rustdoc
        ✓ v3.0 Chain Synchronization
        ✓ v4.0 Persistence
        ✓ v5.0 Transaction Mempool
        ✓ v6.0 Tokio Async Networking
            ↓
        Track A: Enterprise Digital Assets (Primary)
        ─────────────────────────────────
        ✓ v7A Permissioned Network
            ↓
        ✓ v8A Asset Tokenization & Ledger
            ↓
        ✓ v9A Settlement Engine
            ↓
        ✓ v10A Custody Controls
            ↓
        ✓ v11A Policy Engine
            ↓
        v12A Compliance Layer
            ↓
        v13A Audit & Reporting

        Track B: Public Blockchain (Secondary)
        ──────────────────────────────────────
        v7B Difficulty Adjustment
            ↓
        v8B Mining Rewards & Fees
            ↓
July    v9B Fork Handling
            ↓
        v10B Smart Contracts
            ↓
      v11B Light Clients (SPV)
```

## Limitations

This implementation is still evolving. The current codebase intentionally omits several systems that would be required for a fuller blockchain or enterprise ledger platform.

### Current Gaps

- No compliance layer yet
- No audit/reporting layer yet
- No smart contracts or advanced consensus features yet

### Prototype Simplifications

- Coinbase-style transactions are currently represented as transactions with no inputs
- Chain synchronization currently uses a simplified longest-chain replacement model
- Peer discovery and automatic synchronization are not yet implemented
- Proof-of-work difficulty is treated as a simple configurable prefix target
- The wallet is in-memory only and does not persist keys
- Persistence currently uses a simple JSON file rather than a production database

## Testing

Run the full test suite with:

```bash
cargo test
```

Current test suite includes 83 integration tests covering:

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

### Persistence

- Blockchain file creation
- Saved blockchain loading
- Loaded blockchain validation

### Mempool

- Valid transaction insertion
- Invalid transaction rejection
- Duplicate transaction rejection
- Transaction selection for mining
- Mined transaction removal
- Full mempool lifecycle validation

### Async Networking

- Tokio async ChainRequest / ChainResponse validation
- Async TCP request handling
- Async network message serialization/deserialization
- Returned blockchain length validation
- Returned blockchain integrity validation

### Permissioned Networking

- Trusted peer registration
- Duplicate peer rejection
- Node identity validation
- Hello message serialization
- Trusted handshake acceptance
- Untrusted handshake rejection

### Asset Tokenization

- Fungible asset modeling
- Non-fungible asset modeling
- Asset issuance tracking
- Asset ownership tracking
- Asset transfer tracking
- Asset balance crediting
- Valid transfer application
- Insufficient-balance transfer rejection
- Issuance-to-ledger application

### Settlement Engine

- Settlement instruction creation
- Settlement status transitions
- Duplicate settlement rejection
- Valid settlement execution
- Pending settlement batch execution
- Settlement re-execution rejection
- Settlement status counts
- Pending, settled, and failed settlement queries
- Settlement engine CLI demonstration

### Custody Controls

- Custody account creation
- Custody account status transitions
- Duplicate custody account rejection
- Missing custody account status update rejection
- Custody account status queries
- Custody account status counts
- Custody-aware settlement execution
- Settlement rejection for frozen custody accounts
- Settlement rejection for closed custody accounts
- Settlement rejection for missing custody accounts

### Policy Engine

- Policy decision (Allow/Deny) evaluation
- Settlement quantity limit enforcement
- Blocked sender rejection
- Blocked receiver rejection
- Policy-aware settlement execution
- Policy rejection before custody execution

## Repository Structure

- `src/block.rs` - proof-of-work block type and block-level validation
- `src/blockchain.rs` - blockchain container and chain-wide validation
- `src/asset.rs` - asset tokenization models, issuance, ownership, transfers, and balance ledger
- `src/settlement.rs` - settlement instruction model, settlement lifecycle state, and settlement engine
- `src/custody.rs` - custody account model, custody registry, account lifecycle state, and custody control queries
- `src/policy.rs` - policy decision model, settlement authorization rules, and policy-aware settlement checks
- `src/merkle.rs` - Merkle hashing helpers
- `src/network_message.rs` - protocol messages exchanged between peers
- `src/node_identity.rs` - permissioned node identity and network roles
- `src/peer_registry.rs` - trusted peer membership registry
- `src/network.rs` - TCP networking, chain requests, responses, and synchronization
- `src/async_network.rs` - Tokio-based asynchronous networking, block propagation, and chain request/response handling
- `src/transaction.rs` - UTXO transaction creation, signing, and verification
- `src/tx_input.rs` - transaction inputs that reference prior outputs
- `src/tx_output.rs` - spendable transaction outputs
- `src/utxo.rs` - in-memory unspent output tracking and validation
- `src/wallet.rs` - Ed25519 wallet/keypair handling
- `src/storage.rs` - JSON-based blockchain persistence helpers
- `src/mempool.rs` - in-memory pending transaction pool
- `tests/blockchain_tests.rs` - integration coverage for blockchain, synchronization, persistence, mempool, async networking, permissioned networking, asset tokenization, settlement behavior, custody controls, and policy engine behavior

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
- Asynchronous networking via Tokio (`tokio::net`)

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
- Tokio asynchronous networking
- Permissioned node identity
- Trusted peer registry
- Permissioned network handshake
- Asset tokenization model
- Asset issuance and ownership tracking
- Asset balance ledger
- Asset transfer processing
- Custody account lifecycle controls
- Settlement instruction model
- Settlement lifecycle state
- Custody-aware settlement execution against the asset ledger
- Policy-driven settlement authorization
- Settlement status queries
- Peer chain synchronization
- Longest-chain replacement
- JSON-based blockchain persistence
- In-memory transaction mempool
- Mempool transaction selection and removal

The implementation prioritizes readability, correctness, modularity, and incremental evolution over feature completeness. Each version is designed as an educational milestone while remaining architecturally consistent with production distributed ledger systems.

The project is suitable for architectural review, technical discussion, and continued engineering development, but it is not intended for production deployment.

The current implementation includes policy-driven settlement authorization. Future releases will extend this foundation with enterprise compliance controls and audit/reporting capabilities.

## References

### Blockchain Foundations

- [1] R. C. Merkle, "Protocols for public key cryptosystems," in *Proc. 1980 IEEE Symposium on Security and Privacy*, Oakland, CA, USA, 1980, pp. 122-134, doi: [10.1109/SP.1980.10006](https://doi.org/10.1109/SP.1980.10006).
- [2] S. Nakamoto, "Bitcoin: A Peer-to-Peer Electronic Cash System," 2008. Available: https://www.bitcoin.org/bitcoin.pdf.
- [3] A. M. Antonopoulos and D. A. Harding, *Mastering Bitcoin: Programming the Open Blockchain*, 3rd ed. Sebastopol, CA, USA: O'Reilly Media, 2023. Available: https://github.com/bitcoinbook/bitcoinbook.

### Distributed Systems

- [4] M. Kleppmann, *Designing Data-Intensive Applications*. Sebastopol, CA, USA: O'Reilly Media, 2017. Available: https://dataintensive.net/.
- [5] M. van Steen and A. S. Tanenbaum, *Distributed Systems*, 4th ed. distributed-systems.net, 2023. Available: https://www.distributed-systems.net/index.php/books/ds4/.

### Rust

- [6] S. Klabnik, C. Nichols, and C. Krycho, *The Rust Programming Language*. The Rust Project Developers. Available: https://doc.rust-lang.org/stable/book/.
- [7] J. Gjengset, *Rust for Rustaceans: Idiomatic Programming for Experienced Developers*. San Francisco, CA, USA: No Starch Press, 2021. Available: https://nostarch.com/rust-rustaceans.
- [8] The Rust Project Developers, *The Rust Reference*. Available: https://doc.rust-lang.org/reference/.
- [9] The Rust Project Developers, *The Rustonomicon*. Available: https://doc.rust-lang.org/nomicon/.

### Enterprise Digital Assets and DLT

- [10] The Linux Foundation, *Hyperledger Fabric Documentation*. Available: https://hyperledger-fabric.readthedocs.io/en/latest/index.html.
- [11] Digital Asset, *Canton Protocol Specification* and *Synchronizer Overview*. Available: https://docs.canton.network/overview/reference/canton-protocol-specification and https://docs.canton.network/overview/reference/synchronizer-overview.
- [12] DTCC, Clearstream, Euroclear, and Boston Consulting Group, *Digital Asset Securities Control Principles: A Framework for Adoption*, 2024. Available: https://www.dtcc.com/-/media/DASCPWhitePaper.pdf.
- [13] Bank for International Settlements and Institute of International Finance, *Project Agora: A Shared Programmable Platform for Wholesale Cross-Border Payments*, BIS Innovation Hub Other Papers, no. 110, 2026. Available: https://www.bis.org/publ/othp110.htm.
- [14] Bank for International Settlements, Swiss National Bank, and SIX, *Project Helvetia Phase II: Settling Tokenised Assets in Wholesale CBDC*, BIS Innovation Hub, 2022. Available: https://www.bis.org/publ/othp45.pdf.

### Cryptography

- [15] J. Katz and Y. Lindell, *Introduction to Modern Cryptography*, rev. 3rd ed. Boca Raton, FL, USA: CRC Press, 2025. Available: https://www.routledge.com/Introduction-to-Modern-Cryptography-Revised-Third-Edition/Katz-Lindell/p/book/9781032496795.
- [16] C. Paar and J. Pelzl, *Understanding Cryptography: A Textbook for Students and Practitioners*. Berlin, Germany: Springer, 2010, doi: [10.1007/978-3-642-04101-3](https://doi.org/10.1007/978-3-642-04101-3).

## Acknowledgments

This project was developed as an independent research and engineering effort focused on blockchain systems, distributed ledgers, digital asset infrastructure, cryptography, and systems programming in Rust.

The implementation draws inspiration from:

- Ralph Merkle's pioneering work on authenticated data structures and Merkle trees
- Bitcoin's UTXO model, proof-of-work consensus mechanism, and decentralized ledger architecture
- The Rust open-source ecosystem and community
- Research and industry developments in Distributed Ledger Technology (DLT), digital assets, custody, settlement, and tokenization
- Graduate coursework at NYU Tandon School of Engineering, including blockchain, operating systems, cryptography, privacy, machine learning, deep learning, big data and application security

Any errors, omissions, simplifications, or design decisions remain solely the responsibility of the author.

## License

Released under the MIT License.
