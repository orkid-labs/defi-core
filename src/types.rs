//! Core types used throughout DeFi systems

use alloy_primitives::{Address, U256};
use serde::{Deserialize, Serialize};

/// Chain identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Chain {
    Ethereum,
    Base,
    Arbitrum,
    Polygon,
    Optimism,
    Avalanche,
    Binance,
    Solana,
    Other(String),
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chain::Ethereum => write!(f, "ethereum"),
            Chain::Base => write!(f, "base"),
            Chain::Arbitrum => write!(f, "arbitrum"),
            Chain::Polygon => write!(f, "polygon"),
            Chain::Optimism => write!(f, "optimism"),
            Chain::Avalanche => write!(f, "avalanche"),
            Chain::Binance => write!(f, "binance"),
            Chain::Solana => write!(f, "solana"),
            Chain::Other(name) => write!(f, "{}", name),
        }
    }
}

/// Token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
    pub chain: Chain,
}

/// Pool identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PoolId(pub String);

/// Trade direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeDirection {
    Buy,
    Sell,
}

/// Trade parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeParams {
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: U256,
    pub min_amount_out: U256,
    pub direction: TradeDirection,
}

/// Route step in a multi-hop trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStep {
    pub pool_id: PoolId,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: U256,
    pub expected_out: U256,
}

/// Complete trade route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub steps: Vec<RouteStep>,
    pub total_in: U256,
    pub total_out: U256,
    pub gas_estimate: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_display() {
        assert_eq!(Chain::Ethereum.to_string(), "ethereum");
        assert_eq!(Chain::Base.to_string(), "base");
        assert_eq!(Chain::Arbitrum.to_string(), "arbitrum");
        assert_eq!(Chain::Polygon.to_string(), "polygon");
        assert_eq!(Chain::Optimism.to_string(), "optimism");
        assert_eq!(Chain::Avalanche.to_string(), "avalanche");
        assert_eq!(Chain::Binance.to_string(), "binance");
        assert_eq!(Chain::Solana.to_string(), "solana");
        assert_eq!(Chain::Other("custom".to_string()).to_string(), "custom");
    }

    #[test]
    fn chain_equality_and_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Chain::Ethereum);
        set.insert(Chain::Base);
        set.insert(Chain::Ethereum); // duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn chain_serde_roundtrip() {
        let chain = Chain::Base;
        let json = serde_json::to_string(&chain).unwrap();
        let deserialized: Chain = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, chain);
    }

    #[test]
    fn token_creation() {
        let token = Token {
            address: Address::ZERO,
            symbol: "ETH".to_string(),
            decimals: 18,
            chain: Chain::Ethereum,
        };
        assert_eq!(token.symbol, "ETH");
        assert_eq!(token.decimals, 18);
    }

    #[test]
    fn pool_id_equality() {
        let id1 = PoolId("pool-1".to_string());
        let id2 = PoolId("pool-1".to_string());
        let id3 = PoolId("pool-2".to_string());
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn trade_params_serde_roundtrip() {
        let params = TradeParams {
            token_in: Address::ZERO,
            token_out: Address::ZERO,
            amount_in: U256::from(1000u64),
            min_amount_out: U256::from(990u64),
            direction: TradeDirection::Buy,
        };
        let json = serde_json::to_string(&params).unwrap();
        let deserialized: TradeParams = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.amount_in, U256::from(1000u64));
        assert_eq!(deserialized.min_amount_out, U256::from(990u64));
    }

    #[test]
    fn route_step_serde_roundtrip() {
        let step = RouteStep {
            pool_id: PoolId("uniswap-v3-eth-usdc".to_string()),
            token_in: "WETH".to_string(),
            token_out: "USDC".to_string(),
            amount_in: U256::from(1_000_000_000_000_000_000u64),
            expected_out: U256::from(2_000_000_000u64),
        };
        let json = serde_json::to_string(&step).unwrap();
        let deserialized: RouteStep = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.pool_id, PoolId("uniswap-v3-eth-usdc".to_string()));
    }

    #[test]
    fn route_with_multiple_steps() {
        let route = Route {
            steps: vec![
                RouteStep {
                    pool_id: PoolId("pool-1".to_string()),
                    token_in: "WETH".to_string(),
                    token_out: "USDC".to_string(),
                    amount_in: U256::from(1000u64),
                    expected_out: U256::from(2000u64),
                },
                RouteStep {
                    pool_id: PoolId("pool-2".to_string()),
                    token_in: "USDC".to_string(),
                    token_out: "DAI".to_string(),
                    amount_in: U256::from(2000u64),
                    expected_out: U256::from(1990u64),
                },
            ],
            total_in: U256::from(1000u64),
            total_out: U256::from(1990u64),
            gas_estimate: 250_000,
        };
        assert_eq!(route.steps.len(), 2);
        assert_eq!(route.gas_estimate, 250_000);
    }

    #[test]
    fn route_empty_steps() {
        let route = Route {
            steps: vec![],
            total_in: U256::ZERO,
            total_out: U256::ZERO,
            gas_estimate: 0,
        };
        assert!(route.steps.is_empty());
    }
}
