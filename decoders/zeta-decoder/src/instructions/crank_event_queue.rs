use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x438561dfb2bcebb5")]
pub struct CrankEventQueue {
    pub asset: Asset,
}

pub struct CrankEventQueueInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub pricing: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub event_queue: solana_sdk::pubkey::Pubkey,
    pub dex_program: solana_sdk::pubkey::Pubkey,
    pub serum_authority: solana_sdk::pubkey::Pubkey,
    pub perp_sync_queue: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CrankEventQueue {
    type ArrangedAccounts = CrankEventQueueInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, market, event_queue, dex_program, serum_authority, perp_sync_queue, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CrankEventQueueInstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            market: market.pubkey,
            event_queue: event_queue.pubkey,
            dex_program: dex_program.pubkey,
            serum_authority: serum_authority.pubkey,
            perp_sync_queue: perp_sync_queue.pubkey,
        })
    }
}
