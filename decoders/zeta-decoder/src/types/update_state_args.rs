use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateStateArgs {
    pub strike_initialization_threshold_seconds: u32,
    pub pricing_frequency_seconds: u32,
    pub liquidator_liquidation_percentage: u32,
    pub insurance_vault_liquidation_percentage: u32,
    pub native_deposit_limit: u64,
    pub expiration_threshold_seconds: u32,
    pub position_movement_fee_bps: u8,
    pub margin_concession_percentage: u8,
    pub max_perp_delta_age_seconds: u16,
    pub native_withdraw_limit: u64,
    pub withdraw_limit_epoch_seconds: u32,
    pub native_open_interest_limit: u64,
}
