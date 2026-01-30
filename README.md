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

## 🛠 Toolchain Requirements (Tested Versions)

The oracle program has been developed and tested using the following toolchain versions:

```bash
Anchor CLI:      0.32.1
Solana CLI:      3.0.13
Rust (cargo):    1.78.0
```

To ensure reproducible builds and avoid unexpected behavior, it is recommended to use equal or newer compatible versions.

If using `rustup`, make sure you are on the latest stable toolchain:

```bash
rustup default stable
```

For Anchor, install/upgrade via:

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.32.1
avm use 0.32.1
```

---

### Minimum Supported Versions

- Anchor CLI ≥ 0.32.1
- Solana CLI ≥ 3.0.0
- Cargo ≥ 1.78.0

---

### Tested environment:

- OS: Ubuntu 22.04 (WSL2)
- CPU: x86_64
- Chain: Localnet test validator via `anchor test`

---

# If you want auto-pinning (optional)

You can also pin in `Anchor.toml`:

```toml
[toolchain]
anchor_version = "0.32.1"
```

Not required, but solves the annoying mismatch warnings for new contributors.

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

---

# 📦 **Oracle V3 — Design Overview**

## 🧠 What Are We Building?

Oracle v3 is a custom Solana price oracle inspired by Pyth / Switchboard / Chainlink, but simplified and developer-controlled. It provides:

✔ controlled price updates <br>
✔ deterministic feed discovery <br>
✔ metadata for safety (expo, confidence, last update slot) <br>
✔ consumer-side validation constraints

Its core purpose:

> **Store a price, its uncertainty, its decimals, and how recently it was updated — and allow other programs to verify whether this price is fresh and reliable enough for their usage.**

This enables downstream applications (e.g. swaps, liquidations, lending, pricing, risk checks) to make deterministic decisions based on on-chain data.

---

## 🧩 Oracle Mental Model

For each `(owner, symbol)` feed, the oracle stores:

| Field              | Purpose                                       |
| ------------------ | --------------------------------------------- |
| `price`            | raw integer price value (e.g. `103_000_000`)  |
| `expo`             | exponent / decimals (e.g. `-8` means `10^-8`) |
| `confidence`       | price uncertainty interval (±)                |
| `last_update_slot` | recency metadata                              |
| `owner`            | authorized signer for updates                 |
| `symbol`           | market string (e.g. `SOL/USD`)                |

This mirrors Pyth’s data model:

```
price: 103.00 USD
confidence: ±0.002
expo: -4
slot: 213845028
```

except ours is intentionally simpler, and fully controlled at the program layer.

---

## 🏗 Why These Design Choices?

### **① Integer Price + Exponent**

We store:

```
price: i64
expo: i32
```

instead of floating point. This avoids:

* floating point nondeterminism
* rounding errors
* mismatched decimal assumptions
* IEEE754 surprises

Example:

```
price = 123450000
expo  = -6
```

represents `123.45`.

This format supports:

✔ crypto pairs <br>
✔ FX pairs <br>
✔ metals <br>
✔ interest rates <br>
✔ any asset class with precision requirements

---

### **② Confidence Interval is Explicit**

Two prices with equal means can have radically different risk:

```
100,000 ± 50      → safe for liquidation
100,000 ± 25,000  → unusable for liquidation
```

Confidence empowers consumers to implement policies like:

```rust
if confidence <= threshold && is_fresh { allow_trade }
else { block_trade }
```

Key principle:

> The oracle does not enforce policy. Consumers do.

---

### **③ Slot-Based Staleness Checks**

We use `last_update_slot` instead of timestamps because:

* slots monotonically increase
* timestamps may stall
* slot finality reflects consensus time
* avoids clock drift assumptions

Consumers compute:

```
age = current_slot - last_update_slot
```

This defends against:

✔ stale feeds <br>
✔ halted update sources <br>
✔ isolation / censorship scenarios

---

### **④ PDA = (Owner, Symbol)**

We derive feeds via:

```
seeds = ["oracle", owner, symbol]
```

This yields:

**(a) Multi-market feeds per owner**

```
SOL/USD
BTC/USD
ETH/BTC
```

**(b) No global registry**

Feeds are self-organizing and decentralized.

**(c) Deterministic discovery**

Consumers compute addresses directly without lookup tables:

```rust
oracle_addr = derive(owner, "SOL/USD")
```

This matches how Chainlink/Pyth feeds are located on-chain today.

---

### **⑤ Owner-Gated Updates**

Only the `owner` signer may update:

```
has_one = owner
```

This is the simplest authority model and can be extended later to:

* multisig writers
* committee-based feeds
* off-chain attestation
* proxy publishers
* aggregation voting

For v3, single-owner = low-friction + clarity.

---

### **⑥ `check_price` is a Separate Instruction**

Updates publish data.
Checks validate data.

These are deliberately decoupled:

| Role          | Purpose                                        |
| ------------- | ---------------------------------------------- |
| `update`      | publish a new price & metadata                 |
| `check_price` | validate freshness + narrowness for a consumer |

Different consumers have different safety constraints:

| Consumer    | Needs                       |
| ----------- | --------------------------- |
| liquidator  | fresh + strict confidence   |
| swap router | wider confidence acceptable |
| analytics   | may tolerate stale data     |

Therefore:

```ts
checkPrice(maxStalenessSlots, maxConfidence)
```

lets **consumers enforce their own policy** instead of being forced into global constraints.

---

## 🧱 V3 in One Sentence

> **A deterministic PDA-based price feed with owner-controlled updates, built-in confidence intervals, slot recency metadata, and consumer-driven validation logic.**

Or simpler:

> **A customizable on-chain price feed primitive.**

---

## 🛠 Problems V3 Solves

| Oracle Failure Mode      | Defense in V3       |
| ------------------------ | ------------------- |
| stale price              | slot checks         |
| manipulated price        | confidence checks   |
| wrong decimals           | exponent            |
| feed discovery ambiguity | PDA derivation      |
| unauthorized updates     | owner gating        |
| consumer mismatch        | consumer validation |

These map directly to real-world DeFi risk vectors.

---

## 💡 Why This Matters

In DeFi:

> **Prices are the anchor that the entire system balances on.**

If prices are wrong, everything breaks:

* liquidations
* lending
* swaps
* AMMs
* perps
* risk engines

Even though v3 is intentionally small, it captures several foundational correctness themes used in production oracles.

---

## 📦 Oracle V4 — Internal Oracle Enhancements

Oracle V4 extends the V3 design to better support **internal protocol usage**, tighter safety guarantees, and simpler consumer integrations.

### ✨ New features in V4

- **Stored policy per feed**
  - `max_staleness_slots`
  - `max_confidence`  
  These are persisted directly in the oracle account and define the default safety constraints for consumers.

- **Policy management**
  - `set_policy(max_staleness_slots, max_confidence)`  
  Allows the oracle owner to update the stored validation policy for a feed.

- **Policy-based validation**
  - `check_price_stored()`  
  Performs a price validation using the oracle’s stored policy, removing the need for consumers to pass thresholds on every call.

- **Emergency controls**
  - `pause()` / `resume()`  
  Allows the oracle owner to temporarily disable or re-enable the feed.  
  All price checks fail while the oracle is paused.

- **Price sanity enforcement**
  - Updates reject negative prices (`price >= 0`), ensuring the oracle remains safe for spot-price usage within internal protocols.

- **Symbol length safety**
  - Enforces a maximum symbol length of 16 bytes during initialization, preventing account layout and serialization issues.

These additions make V4 suitable as a production-grade **internal oracle primitive** for protocol components such as lending markets, AMMs, and liquidation engines.
