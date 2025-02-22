use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x354da79daf8390ab")]
pub struct CancelOrderByClientOrderIdNoError {
    pub client_order_id: u64,
    pub asset: Asset,
}

pub struct CancelOrderByClientOrderIdNoErrorInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub cancel_accounts: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelOrderByClientOrderIdNoError {
    type ArrangedAccounts = CancelOrderByClientOrderIdNoErrorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, cancel_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelOrderByClientOrderIdNoErrorInstructionAccounts {
            authority: authority.pubkey,
            cancel_accounts: cancel_accounts.pubkey,
        })
    }
}
