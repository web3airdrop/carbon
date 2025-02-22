use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xc5185209520e309a")]
pub struct CrossOpenOrdersMap {
    pub user_key: solana_sdk::pubkey::Pubkey,
    pub subaccount_index: u8,
}
