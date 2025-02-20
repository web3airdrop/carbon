use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2d4f51a0f85a5bdc")]
pub struct PlaceSpotOrder {
    pub params: OrderParams,
}

pub struct PlaceSpotOrderInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceSpotOrder {
    type ArrangedAccounts = PlaceSpotOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PlaceSpotOrderInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
