# Disaster Relief DAO

**Problem:** During frequent typhoons in the Philippines, international aid takes weeks to trickle down to affected municipalities due to bureaucratic bottlenecks and high cross-border NGO fees.

**Solution:** A transparent, smart-contract-managed relief fund where global donors pool USDC. When a local mayor or verified NGO signals an emergency, the Soroban contract instantly disburses funds to pre-registered local merchants (for water, rice, medicine) via a 2-of-3 multi-sig approval.

**Timeline:** 3-Day Hackathon Project

**Stellar Features Used:** USDC transfers, Soroban smart contracts, Multi-sig / Auth

**Vision and Purpose:** Solves a high-stakes, life-or-death problem in a climate-vulnerable region. Perfectly showcases blockchain transparency and instant settlement.

## Prerequisites
* Rust toolchain (stable)
* Soroban CLI (`soroban-cli v20.x.x`)

## Commands

**How to build:**
```bash
soroban contract build
```

**How to test:**
```bash
cargo test
```

**How to deploy to testnet:**
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/disaster_relief_dao.wasm \
  --source default \
  --network testnet
```

**Sample CLI Invocation (Donate MVP feature):**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source donor \
  --network testnet \
  -- \
  donate \
  --donor <DONOR_ADDR> \
  --token <USDC_TOKEN_ADDR> \
  --amount 1000
```

## License
MIT
