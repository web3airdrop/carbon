use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvictEvent {
    pub index: u16,
    pub maker_id: solana_sdk::pubkey::Pubkey,
    pub order_sequence_number: u64,
    pub price_in_ticks: u64,
    pub base_lots_evicted: u64,
}
