use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8bcd8d8d71245ebb")]
pub struct UpdateUserDelegate {
    pub sub_account_id: u16,
    pub delegate: solana_sdk::pubkey::Pubkey,
}

pub struct UpdateUserDelegateInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateUserDelegate {
    type ArrangedAccounts = UpdateUserDelegateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserDelegateInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
