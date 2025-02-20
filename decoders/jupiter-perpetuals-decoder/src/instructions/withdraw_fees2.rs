use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfc808f91e1dd9fcf")]
pub struct WithdrawFees2 {
    pub params: WithdrawFees2Params,
}

pub struct WithdrawFees2InstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_token_account: solana_sdk::pubkey::Pubkey,
    pub custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub receiving_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFees2 {
    type ArrangedAccounts = WithdrawFees2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, transfer_authority, perpetuals, pool, custody, custody_token_account, custody_doves_price_account, custody_pythnet_price_account, receiving_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawFees2InstructionAccounts {
            keeper: keeper.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_token_account: custody_token_account.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            receiving_token_account: receiving_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
