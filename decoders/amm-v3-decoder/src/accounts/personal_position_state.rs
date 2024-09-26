 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use crate::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x466f967ee60f1975")] 
pub struct PersonalPositionState { 
        pub bump: u8, 
        pub nft_mint: solana_sdk::pubkey::Pubkey, 
        pub pool_id: solana_sdk::pubkey::Pubkey, 
        pub tick_lower_index: i32, 
        pub tick_upper_index: i32, 
        pub liquidity: u128, 
        pub fee_growth_inside0_last_x64: u128, 
        pub fee_growth_inside1_last_x64: u128, 
        pub token_fees_owed0: u64, 
        pub token_fees_owed1: u64, 
        pub reward_infos: [PositionRewardInfo; 3], 
        pub recent_epoch: u64, 
        pub padding: [u64; 7], 
}