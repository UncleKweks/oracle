# 📦 **Oracle Anchor — v1**

**Simple Owner-Controlled On-Chain Price Oracle (Solana / Anchor)**

---

## 📝 Overview

`oracle-anchor` is a minimal on-chain price oracle built using [Anchor](https://www.anchor-lang.com) for Solana.
It demonstrates the core mechanisms behind a trusted price feed:

* a dedicated on-chain account that stores a price
* strict access-control on price updates
* validation that prevents unauthorized writes

This version (`v1`) focuses on basic correctness and authority enforcement, forming a foundation for future expansions (multi-asset support, feeders, confidence intervals, governance, etc).

---

## 🎯 Features

* **Single owner authority**
* **On-chain price storage**
* **Strict signer + ownership validation**
* **Upgradeable price via secure instruction**
* **Test suite included**

---

## 🔐 Security Model

This oracle assumes a **trusted owner model**:

* only the `owner` public key can update the oracle price
* updates require a valid transaction signature
* price consumers trust the owner to post truthful data

Future versions may introduce:

* multiple assets
* feeder sets
* governance/multisig control
* confidence metrics
* deviation limits
* staking or incentives

---

## 🧱 Architecture

### **Account: `Oracle`**

Stores:

| Field   | Type     | Description                 |
| ------- | -------- | --------------------------- |
| `owner` | `Pubkey` | Authorized updater          |
| `price` | `i64`    | Price value posted by owner |

### **Instruction: `initialize(initial_price: i64)`**

* Creates a new Oracle account
* Assigns the owner and initial price

### **Instruction: `update(new_price: i64)`**

* Requires:

  * the owner to be the signer
  * the owner matches `oracle.owner`
* Updates the price

---

## 🧪 Testing

The project ships with a test suite exercising:

* initialization
* successful owner price updates
* failure when unauthorized accounts attempt updates

Run tests with:

```bash
anchor test
```

---

## 🛠 Development

### **Build**

```bash
anchor build
```

### **Local Validator**

In another shell:

```bash
solana-test-validator
```

---

## 📂 Project Layout

```
programs/oracle/
  src/
    lib.rs          # core logic & accounts
    instructions.rs # update logic
tests/
  oracle.ts         # Anchor test suite
Anchor.toml         # workspace config
```

---

## 🚀 Deployment (localnet)

(Optional, for local experimentation)

```bash
anchor deploy
anchor keys sync
```

---

## 🗺 Future Roadmap

Planned `v2 — Multi-Asset Oracle`:

* per-asset PDAs (e.g. "SOL/USD", "BTC/USD")
* last update slot → staleness checks
* exponent/scaling fields
* event emissions for indexers

Planned `v3+` (optional advanced direction):

* feeder sets & aggregation
* confidence intervals
* deviation circuit breakers
* multisig governance authority
* off-chain poster service
* client SDK for integration

---

## 🏁 Status

**Version:** `v1 – Trusted single-asset oracle` <br>
**Maturity:** Educational / starter infrastructure

---

## 🧾 Oracle (Solana / Anchor) — V2

A lightweight, trusted price oracle built using Anchor on Solana.  
Supports multiple price feeds (e.g. `SOL/USD`, `BTC/USD`) with metadata for integration and staleness checks.

This project models the **on-chain delivery layer** of real-world oracle systems such as Pyth and Chainlink, and sets up a foundation for more advanced aggregation and governance models in future versions.

---

## ✨ Features (V2)

**✔ Multi-Asset Support**  
Each price feed is represented as a PDA keyed by `(owner, symbol)`:

```

PDA = seeds("oracle", owner, symbol)

```

This enables deterministic discovery and unbounded scaling across assets without global registries.

**✔ Structured Price Data**  
Each feed stores:

| Field                   | Purpose                            |
| ----------------------- | ---------------------------------- |
| `price: i64`            | base price value                   |
| `expo: i32`             | decimal scaling exponent           |
| `confidence: u64`       | error range around price           |
| `symbol: String`        | market identifier (e.g. `SOL/USD`) |
| `last_update_slot: u64` | freshness tracking                 |
| `owner: Pubkey`         | authorized price poster            |

**✔ Staleness Tracking**  
`last_update_slot` allows consumers to reject stale updates (important for lending protocols, liquidations, etc.)

**✔ Event Emission for Indexers**  
`PriceUpdated` events are emitted on every update, making it easy to index with:

- RPC WebSockets
- Helius / Triton
- Geyser plugins
- custom off-chain pipelines

**✔ PDA-Based Access Model**  
Consumers can derive oracle addresses without storing them.

---

## 🧱 Architecture

### PDA Derivation

```

("oracle", owner, symbol)

````

This design allows:

- multiple assets per owner
- multiple owners per asset (future feeder sets)
- deterministic lookups

Example consumer lookup:

```ts
findProgramAddress([
  Buffer.from("oracle"),
  owner,
  Buffer.from("SOL/USD"),
])
````

---

## 📡 Instructions

### `initialize(symbol, price, expo, confidence)`

Creates a new oracle PDA for a market and sets an initial price.

* only one `(owner, symbol)` oracle can exist
* payer funds the account creation
* `owner` is the trusted updater

### `update(new_price, new_confidence)`

Updates the price & confidence and stamps the slot.
Protected by:

```
has_one = owner
owner: Signer
```

---

## 🧪 Tests

Tests validate:

* initialization flow
* deterministic PDA derivation
* update flow with slot bump
* non-owner failure
* multi-market support for same owner

Run:

```bash
anchor test
```

---

## 🗺 Future Roadmap

### **Planned V3 — Safety & Consumer APIs**

* `get_price_checked(max_age_slots)` for on-chain staleness enforcement
* deviation checks
* circuit breakers
* view helpers for dApps & lending protocols

### **Planned V4 — Multi-Feeder & Aggregation**

* authorized feeder sets
* median / weighted aggregation
* slashing conditions
* governance control

### **Planned V5 — Off-Chain Poster & Data Layer**

* off-chain ingestion from exchanges (Binance, Coinbase, Pyth, etc.)
* off-chain normalization & validation
* poster service relaying prices on-chain
* SDK for third-party integrations

---

## 🧩 Why This Matters

Many DeFi applications require reliable market data:

* lending & borrowing
* perpetual futures
* AMM rebalancing
* liquidations & LTV calculations
* structured volatility products

This oracle provides the foundation for plugging such data into Solana programs with safe access patterns.

---

## 🏗 Built With

* **Solana**
* **Anchor**
* **TypeScript**
* **Rust**

---

## 📂 Repository Structure

```
programs/oracle/      → on-chain program (Rust)
tests/                → Anchor Mocha tests
target/               → IDLs + build artifacts
Anchor.toml           → config
```

---

## License

MIT

