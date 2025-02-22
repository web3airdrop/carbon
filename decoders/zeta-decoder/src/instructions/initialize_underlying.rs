use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x726cd55caf7c2b13")]
pub struct InitializeUnderlying {
    pub flex_underlying: bool,
}

pub struct InitializeUnderlyingInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub zeta_program: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub underlying: solana_sdk::pubkey::Pubkey,
    pub underlying_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeUnderlying {
    type ArrangedAccounts = InitializeUnderlyingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, zeta_program, state, system_program, underlying, underlying_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeUnderlyingInstructionAccounts {
            admin: admin.pubkey,
            zeta_program: zeta_program.pubkey,
            state: state.pubkey,
            system_program: system_program.pubkey,
            underlying: underlying.pubkey,
            underlying_mint: underlying_mint.pubkey,
        })
    }
}
