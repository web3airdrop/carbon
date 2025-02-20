use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x80a68e09febb8fae")]
pub struct RemoveInsuranceFundStake {
    pub market_index: u16,
}

pub struct RemoveInsuranceFundStakeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub spot_market: solana_sdk::pubkey::Pubkey,
    pub insurance_fund_stake: solana_sdk::pubkey::Pubkey,
    pub user_stats: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub insurance_fund_vault: solana_sdk::pubkey::Pubkey,
    pub drift_signer: solana_sdk::pubkey::Pubkey,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveInsuranceFundStake {
    type ArrangedAccounts = RemoveInsuranceFundStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, spot_market, insurance_fund_stake, user_stats, authority, insurance_fund_vault, drift_signer, user_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RemoveInsuranceFundStakeInstructionAccounts {
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            insurance_fund_stake: insurance_fund_stake.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
            drift_signer: drift_signer.pubkey,
            user_token_account: user_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
