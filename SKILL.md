---
name: defi-core
description: Core DeFi domain types for Rust — chains, tokens, pools, trades, and a comprehensive error model.
---

# defi-core

defi-core defines the shared vocabulary the rest of the Orkid stack is built on: Chain, Token, Pool, Trade, Route, and error types that model real DEX failure modes (insufficient liquidity, slippage exceeded, expired deadlines). Use it when you need well-typed DeFi primitives instead of ad-hoc structs.

## When to use

Building any Rust service that touches DEX routing, quoting, portfolio accounting, or on-chain trade modeling.

## Install

```
cargo add defi-core
```

## Links

- Repository: https://github.com/orkid-labs/defi-core
- Package: https://crates.io/crates/defi-core
- Documentation: https://docs.rs/defi-core
- Orkid Labs: https://orkidlabs.xyz
