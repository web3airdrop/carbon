use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdf3ab45666fbed52")]
pub struct UpdatePricingV3 {
    pub asset: Asset,
    pub price: u64,
    pub timestamp: u64,
}

pub struct UpdatePricingV3InstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub pricing: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub perp_market: solana_sdk::pubkey::Pubkey,
    pub perp_bids: solana_sdk::pubkey::Pubkey,
    pub perp_asks: solana_sdk::pubkey::Pubkey,
    pub pricing_admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePricingV3 {
    type ArrangedAccounts = UpdatePricingV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, oracle, perp_market, perp_bids, perp_asks, pricing_admin, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UpdatePricingV3InstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            oracle: oracle.pubkey,
            perp_market: perp_market.pubkey,
            perp_bids: perp_bids.pubkey,
            perp_asks: perp_asks.pubkey,
            pricing_admin: pricing_admin.pubkey,
        })
    }
}
