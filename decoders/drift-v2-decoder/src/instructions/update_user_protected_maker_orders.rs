use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x72277bc6bb195adb")]
pub struct UpdateUserProtectedMakerOrders {
    pub sub_account_id: u16,
    pub protected_maker_orders: bool,
}

pub struct UpdateUserProtectedMakerOrdersInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub protected_maker_mode_config: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateUserProtectedMakerOrders {
    type ArrangedAccounts = UpdateUserProtectedMakerOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, authority, protected_maker_mode_config, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(UpdateUserProtectedMakerOrdersInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            authority: authority.pubkey,
            protected_maker_mode_config: protected_maker_mode_config.pubkey,
        })
    }
}
