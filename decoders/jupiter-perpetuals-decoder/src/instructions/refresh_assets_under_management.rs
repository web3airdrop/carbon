use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa200d737e10fb900")]
pub struct RefreshAssetsUnderManagement {
    pub params: RefreshAssetsUnderManagementParams,
}

pub struct RefreshAssetsUnderManagementInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RefreshAssetsUnderManagement {
    type ArrangedAccounts = RefreshAssetsUnderManagementInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, perpetuals, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RefreshAssetsUnderManagementInstructionAccounts {
            keeper: keeper.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
        })
    }
}
