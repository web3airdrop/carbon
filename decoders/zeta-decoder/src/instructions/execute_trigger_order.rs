use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x690a6888d78654ab")]
pub struct ExecuteTriggerOrder {
    pub trigger_order_bit: u8,
}

pub struct ExecuteTriggerOrderInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub trigger_order: solana_sdk::pubkey::Pubkey,
    pub place_order_accounts: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExecuteTriggerOrder {
    type ArrangedAccounts = ExecuteTriggerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, trigger_order, place_order_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ExecuteTriggerOrderInstructionAccounts {
            admin: admin.pubkey,
            trigger_order: trigger_order.pubkey,
            place_order_accounts: place_order_accounts.pubkey,
        })
    }
}
