use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1decba71db2a3395f9")]
pub struct NewUserRecordEvent {
    pub ts: i64,
    pub user_authority: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub sub_account_id: u16,
    pub name: [u8; 32],
    pub referrer: solana_sdk::pubkey::Pubkey,
}
