//! Contract-oriented protocol modules.

pub mod circuit_breaker;
pub mod lending;
pub mod oracle;

pub use circuit_breaker::CircuitBreakerContract;
pub use lending::LendingProtocol;
pub use oracle::PriceOracle;
