//! Contract-oriented protocol modules.

pub mod lending;
pub mod oracle;
pub mod price_history;

pub use lending::LendingProtocol;
pub use oracle::{PriceOracle, PriceOracleSim};
pub use price_history::{
    PriceHistoryManager, TimeBucket, PriceHistoryEntry, TwapResult, AnalyticsData,
    PriceTrend, AssetMetadata, PriceBucket, PriceHistoryError,
};
