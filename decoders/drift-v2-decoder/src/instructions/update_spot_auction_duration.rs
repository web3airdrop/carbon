use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb6b2cb48bb8f9d6b")]
pub struct UpdateSpotAuctionDuration {
    pub default_spot_auction_duration: u8,
}

pub struct UpdateSpotAuctionDurationInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSpotAuctionDuration {
    type ArrangedAccounts = UpdateSpotAuctionDurationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotAuctionDurationInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
