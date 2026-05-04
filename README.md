# SariPay Split

Digitizing sari-sari store *utang* with transparent, on-chain tracking.

---

## 🧩 Problem

A sari-sari store owner in the Philippines records customer debts (*utang*) in a notebook, often losing ₱500–₱2,000 per month due to forgotten entries, disputes, or unpaid balances.

---

## 💡 Solution

SariPay Split uses Soroban smart contracts on the Stellar network to record debts and repayments transparently.
Store owners add debt entries, and customers repay using digital payments, with balances updated instantly and verifiable on-chain.

---

## ⚙️ MVP Flow (Demo in <2 minutes)

1. Store owner adds ₱100 debt for a customer
2. Customer repays ₱40
3. Contract updates balance to ₱60
4. Anyone can query and verify the remaining debt

---

## ⏱️ Timeline

* Day 1: Smart contract (debt + repayment logic)
* Day 2: Testing + basic frontend (input + display)
* Day 3: Demo polish + pitch

---

## 🌐 Stellar Features Used

* Soroban Smart Contracts – for debt tracking logic
* USDC (planned integration) – for real payments
* Low fees & fast finality – ideal for microtransactions

---

## 🎯 Vision & Purpose

SariPay Split aims to modernize informal micro-credit systems used by millions of small stores in Southeast Asia by making them transparent, tamper-proof, and digitally verifiable.

---

## 🛠️ Prerequisites

* Rust (latest stable)
* Soroban CLI (v20+)
* Cargo

---

## 🔨 Build

```bash
soroban contract build
```

---

## 🧪 Test

```bash
cargo test
```

---

## 🚀 Deploy (Testnet)

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/sari_pay_split.wasm \
  --source <YOUR_IDENTITY> \
  --network testnet
```

---

## ▶️ Sample Invocation (MVP)

### Add Debt

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn add_debt \
  --arg <STORE_ADDRESS> \
  --arg <CUSTOMER_ADDRESS> \
  --arg 100
```

### Repay Debt

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn repay \
  --arg <CUSTOMER_ADDRESS> \
  --arg <STORE_ADDRESS> \
  --arg 40
```

### Check Balance

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn get_balance \
  --arg <CUSTOMER_ADDRESS> \
  --arg <STORE_ADDRESS>
```

---

## 🔮 Future Improvements

* USDC token integration for real payments
* Mobile-first frontend for sari-sari stores
* QR code payments
* Offline transaction support (queued sync)

---

## 📄 License

MIT License
