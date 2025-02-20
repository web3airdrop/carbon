use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7f4275392832987f")]
pub struct SettleMultiplePnls {
    pub market_indexes: Vec<u16>,
    pub mode: SettlePnlMode,
}

pub struct SettleMultiplePnlsInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub spot_market_vault: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettleMultiplePnls {
    type ArrangedAccounts = SettleMultiplePnlsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, authority, spot_market_vault, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SettleMultiplePnlsInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            authority: authority.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
        })
    }
}
