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
