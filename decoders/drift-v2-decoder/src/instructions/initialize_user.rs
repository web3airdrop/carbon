use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6f11b9fa3c7a26fe")]
pub struct InitializeUser {
    pub sub_account_id: u16,
    pub name: [u8; 32],
}

pub struct InitializeUserInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_stats: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeUser {
    type ArrangedAccounts = InitializeUserInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, user_stats, state, authority, payer, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeUserInstructionAccounts {
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            state: state.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
