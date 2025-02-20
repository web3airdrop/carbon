use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x30fc7749ffcdaef7")]
pub struct DepositIntoSpotMarketVault {
    pub amount: u64,
}

pub struct DepositIntoSpotMarketVaultInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub spot_market: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub source_vault: solana_sdk::pubkey::Pubkey,
    pub spot_market_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositIntoSpotMarketVault {
    type ArrangedAccounts = DepositIntoSpotMarketVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, spot_market, admin, source_vault, spot_market_vault, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositIntoSpotMarketVaultInstructionAccounts {
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            admin: admin.pubkey,
            source_vault: source_vault.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
