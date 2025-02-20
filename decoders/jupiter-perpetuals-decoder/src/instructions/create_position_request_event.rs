use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d02ee5e3569d32eba")]
pub struct CreatePositionRequestEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position_key: solana_sdk::pubkey::Pubkey,
    pub position_side: u8,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_custody: solana_sdk::pubkey::Pubkey,
    pub position_collateral_mint: solana_sdk::pubkey::Pubkey,
    pub position_collateral_custody: solana_sdk::pubkey::Pubkey,
    pub position_request_key: solana_sdk::pubkey::Pubkey,
    pub position_request_mint: solana_sdk::pubkey::Pubkey,
    pub size_usd_delta: u64,
    pub collateral_delta: u64,
    pub price_slippage: Option<u64>,
    pub jupiter_minimum_out: Option<u64>,
    pub pre_swap_amount: Option<u64>,
    pub request_change: u8,
    pub open_time: i64,
    pub referral: Option<solana_sdk::pubkey::Pubkey>,
}
