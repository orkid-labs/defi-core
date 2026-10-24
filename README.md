# DeFi Core

Generic DeFi core types and error handling for Rust - chain, token, pool, trade types with comprehensive error handling.

## Features

- Chain identifiers (Ethereum, Base, Arbitrum, Polygon, Optimism, Avalanche, Binance, Solana, custom)
- Token information with address, symbol, decimals, and chain
- Pool identifiers and data structures
- Trade parameters and routing types
- Comprehensive error handling for DeFi operations
- Serialization support (serde)

## Usage

```rust
use defi_core::{Chain, Token, TradeParams, TradeDirection, DeFiError};

// Create a token
let token = Token {
    address: Address::ZERO,
    symbol: "ETH".to_string(),
    decimals: 18,
    chain: Chain::Ethereum,
};

// Define trade parameters
let params = TradeParams {
    token_in: Address::ZERO,
    token_out: Address::ZERO,
    amount_in: U256::from(1000u64),
    min_amount_out: U256::from(990u64),
    direction: TradeDirection::Buy,
};

// Handle errors
fn execute_trade() -> Result<()> {
    // Your trading logic here
    Ok(())
}
```

## License

MIT

---

## Part of Orkid Labs

**defi-core** — Core DeFi domain types for Rust — chains, tokens, pools, trades, and a comprehensive error model.

- Install: `cargo add defi-core`
- Package: https://crates.io/crates/defi-core
- API docs: https://docs.rs/defi-core
- Docs: https://docs.orkidlabs.xyz/packages/defi-core

Related Orkid projects: [cctp-utils](https://github.com/orkid-labs/cctp-utils), [splits-distributor](https://github.com/orkid-labs/splits-distributor), [settlement-types](https://github.com/orkid-labs/settlement-types)

Orkid Labs builds gasless, MEV-protected swap routing and agent-native DeFi infrastructure — [orkidlabs.xyz](https://orkidlabs.xyz) · [@orkid-labs on npm](https://www.npmjs.com/org/orkid-labs) · [github.com/orkid-labs](https://github.com/orkid-labs)
