use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MarketSizeParams {
    pub bids_size: u64,
    pub asks_size: u64,
    pub num_seats: u64,
}
