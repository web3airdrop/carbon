use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe60921cae4d1b462")]
pub struct UpdateOracleBackupFeed {}

pub struct UpdateOracleBackupFeedInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub zeta_group: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateOracleBackupFeed {
    type ArrangedAccounts = UpdateOracleBackupFeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, admin, oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateOracleBackupFeedInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            admin: admin.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
