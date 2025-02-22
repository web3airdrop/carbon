use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateVolatilityArgs {
    pub expiry_index: u8,
    pub volatility: [u64; 5],
}
