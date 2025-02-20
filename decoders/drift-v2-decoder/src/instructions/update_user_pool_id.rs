use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdb56496a38da806d")]
pub struct UpdateUserPoolId {
    pub sub_account_id: u16,
    pub pool_id: u8,
}

pub struct UpdateUserPoolIdInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateUserPoolId {
    type ArrangedAccounts = UpdateUserPoolIdInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserPoolIdInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
