use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x41a0c570efa867b9")]
pub struct SerumV3FulfillmentConfig {
    pub pubkey: solana_sdk::pubkey::Pubkey,
    pub serum_program_id: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_request_queue: solana_sdk::pubkey::Pubkey,
    pub serum_event_queue: solana_sdk::pubkey::Pubkey,
    pub serum_bids: solana_sdk::pubkey::Pubkey,
    pub serum_asks: solana_sdk::pubkey::Pubkey,
    pub serum_base_vault: solana_sdk::pubkey::Pubkey,
    pub serum_quote_vault: solana_sdk::pubkey::Pubkey,
    pub serum_open_orders: solana_sdk::pubkey::Pubkey,
    pub serum_signer_nonce: u64,
    pub market_index: u16,
    pub fulfillment_type: SpotFulfillmentType,
    pub status: SpotFulfillmentConfigStatus,
    pub padding: [u8; 4],
}
