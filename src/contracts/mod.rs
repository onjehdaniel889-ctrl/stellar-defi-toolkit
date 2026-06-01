//! Contract-oriented protocol modules.

pub mod lending;
pub mod oracle;
pub mod decentralized_oracle;

pub use lending::LendingProtocol;
pub use oracle::{PriceOracle, PriceOracleSim};
pub use decentralized_oracle::{DecentralizedOracle, DecentralizedOracleError};
