use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x22")]
pub struct EnableCpiGuard {
    pub cpi_guard_discriminator: u8,
}

pub struct EnableCpiGuardInstructionAccounts {
    pub token: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EnableCpiGuard {
    type ArrangedAccounts = EnableCpiGuardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, owner] = accounts else {
            return None;
        };

        Some(EnableCpiGuardInstructionAccounts {
            token: token.pubkey,
            owner: owner.pubkey,
        })
    }
}
