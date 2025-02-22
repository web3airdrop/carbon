use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf3d504ec1af6b4ae")]
pub struct CollectTreasuryFunds {
    pub amount: u64,
}

pub struct CollectTreasuryFundsInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub treasury_wallet: solana_sdk::pubkey::Pubkey,
    pub collection_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectTreasuryFunds {
    type ArrangedAccounts = CollectTreasuryFundsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, treasury_wallet, collection_token_account, token_program, admin, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectTreasuryFundsInstructionAccounts {
            state: state.pubkey,
            treasury_wallet: treasury_wallet.pubkey,
            collection_token_account: collection_token_account.pubkey,
            token_program: token_program.pubkey,
            admin: admin.pubkey,
        })
    }
}
