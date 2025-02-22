use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x69d729efa6cf0167")]
pub struct CloseMarginAccount {}

pub struct CloseMarginAccountInstructionAccounts {
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub zeta_group: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseMarginAccount {
    type ArrangedAccounts = CloseMarginAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [margin_account, authority, zeta_group, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseMarginAccountInstructionAccounts {
            margin_account: margin_account.pubkey,
            authority: authority.pubkey,
            zeta_group: zeta_group.pubkey,
        })
    }
}
