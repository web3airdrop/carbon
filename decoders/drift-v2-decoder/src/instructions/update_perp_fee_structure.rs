use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17b26fcb49168c4b")]
pub struct UpdatePerpFeeStructure {
    pub fee_structure: FeeStructure,
}

pub struct UpdatePerpFeeStructureInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePerpFeeStructure {
    type ArrangedAccounts = UpdatePerpFeeStructureInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpFeeStructureInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
