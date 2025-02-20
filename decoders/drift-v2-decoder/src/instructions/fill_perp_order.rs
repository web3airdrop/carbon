use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0dbcf86786d96af0")]
pub struct FillPerpOrder {
    pub order_id: Option<u32>,
    pub maker_order_id: Option<u32>,
}

pub struct FillPerpOrderInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub filler: solana_sdk::pubkey::Pubkey,
    pub filler_stats: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_stats: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FillPerpOrder {
    type ArrangedAccounts = FillPerpOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, filler, filler_stats, user, user_stats, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(FillPerpOrderInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            filler: filler.pubkey,
            filler_stats: filler_stats.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
        })
    }
}
