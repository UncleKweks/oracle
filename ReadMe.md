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


