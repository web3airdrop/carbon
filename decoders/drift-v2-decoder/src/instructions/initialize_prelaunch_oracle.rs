use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9b25419af3e1df7")]
pub struct InitializePrelaunchOracle {
    pub params: PrelaunchOracleParams,
}

pub struct InitializePrelaunchOracleInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub prelaunch_oracle: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePrelaunchOracle {
    type ArrangedAccounts = InitializePrelaunchOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, prelaunch_oracle, state, rent, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(InitializePrelaunchOracleInstructionAccounts {
            admin: admin.pubkey,
            prelaunch_oracle: prelaunch_oracle.pubkey,
            state: state.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
