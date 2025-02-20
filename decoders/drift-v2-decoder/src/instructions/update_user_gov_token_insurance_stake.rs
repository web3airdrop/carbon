use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8f63ebbb149fb854")]
pub struct UpdateUserGovTokenInsuranceStake {}

pub struct UpdateUserGovTokenInsuranceStakeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub spot_market: solana_sdk::pubkey::Pubkey,
    pub insurance_fund_stake: solana_sdk::pubkey::Pubkey,
    pub user_stats: solana_sdk::pubkey::Pubkey,
    pub signer: solana_sdk::pubkey::Pubkey,
    pub insurance_fund_vault: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateUserGovTokenInsuranceStake {
    type ArrangedAccounts = UpdateUserGovTokenInsuranceStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, spot_market, insurance_fund_stake, user_stats, signer, insurance_fund_vault, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UpdateUserGovTokenInsuranceStakeInstructionAccounts {
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            insurance_fund_stake: insurance_fund_stake.pubkey,
            user_stats: user_stats.pubkey,
            signer: signer.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
        })
    }
}
