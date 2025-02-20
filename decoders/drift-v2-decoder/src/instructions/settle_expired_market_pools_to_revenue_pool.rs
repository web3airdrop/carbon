use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3713eea9e35ac8b8")]
pub struct SettleExpiredMarketPoolsToRevenuePool {}

pub struct SettleExpiredMarketPoolsToRevenuePoolInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub spot_market: solana_sdk::pubkey::Pubkey,
    pub perp_market: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettleExpiredMarketPoolsToRevenuePool {
    type ArrangedAccounts = SettleExpiredMarketPoolsToRevenuePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, spot_market, perp_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SettleExpiredMarketPoolsToRevenuePoolInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            spot_market: spot_market.pubkey,
            perp_market: perp_market.pubkey,
        })
    }
}
