//! Error types for DeFi operations

use thiserror::Error;

/// Core error type for DeFi operations
#[derive(Error, Debug)]
pub enum DeFiError {
    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Network error: {message}")]
    Network { message: String },

    #[error("Execution error: {message}")]
    Execution { message: String },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("DEX error: {message}")]
    Dex { message: String },

    #[error("Protocol error: {message}")]
    Protocol { message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Generic error: {0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, DeFiError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_error_display() {
        let err = DeFiError::Config {
            message: "missing key".to_string(),
        };
        assert_eq!(err.to_string(), "Configuration error: missing key");
    }

    #[test]
    fn network_error_display() {
        let err = DeFiError::Network {
            message: "timeout".to_string(),
        };
        assert_eq!(err.to_string(), "Network error: timeout");
    }

    #[test]
    fn execution_error_display() {
        let err = DeFiError::Execution {
            message: "reverted".to_string(),
        };
        assert_eq!(err.to_string(), "Execution error: reverted");
    }

    #[test]
    fn validation_error_display() {
        let err = DeFiError::Validation {
            message: "bad input".to_string(),
        };
        assert_eq!(err.to_string(), "Validation error: bad input");
    }

    #[test]
    fn dex_error_display() {
        let err = DeFiError::Dex {
            message: "insufficient liquidity".to_string(),
        };
        assert_eq!(err.to_string(), "DEX error: insufficient liquidity");
    }

    #[test]
    fn protocol_error_display() {
        let err = DeFiError::Protocol {
            message: "unsupported chain".to_string(),
        };
        assert_eq!(err.to_string(), "Protocol error: unsupported chain");
    }

    #[test]
    fn generic_error_display() {
        let err = DeFiError::Generic("something broke".to_string());
        assert_eq!(err.to_string(), "Generic error: something broke");
    }

    #[test]
    fn from_serde_json_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("not json").unwrap_err();
        let err: DeFiError = json_err.into();
        assert!(matches!(err, DeFiError::Serialization(_)));
    }

    #[test]
    fn from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let err: DeFiError = io_err.into();
        assert!(matches!(err, DeFiError::Io(_)));
    }

    #[test]
    fn result_type_alias() {
        fn returns_ok() -> Result<u32> {
            Ok(42)
        }
        fn returns_err() -> Result<u32> {
            Err(DeFiError::Generic("fail".to_string()))
        }
        assert_eq!(returns_ok().unwrap(), 42);
        assert!(returns_err().is_err());
    }
}
