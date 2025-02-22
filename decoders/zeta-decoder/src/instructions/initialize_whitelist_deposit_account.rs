use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3de773db51f39e8a")]
pub struct InitializeWhitelistDepositAccount {
    pub nonce: u8,
}

pub struct InitializeWhitelistDepositAccountInstructionAccounts {
    pub whitelist_deposit_account: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeWhitelistDepositAccount {
    type ArrangedAccounts = InitializeWhitelistDepositAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whitelist_deposit_account, admin, user, system_program, state, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeWhitelistDepositAccountInstructionAccounts {
            whitelist_deposit_account: whitelist_deposit_account.pubkey,
            admin: admin.pubkey,
            user: user.pubkey,
            system_program: system_program.pubkey,
            state: state.pubkey,
        })
    }
}
