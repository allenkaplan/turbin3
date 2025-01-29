use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Freeze period is not over")]
    FreezePeriodNotOver,
    #[msg("Max stake reached")]
    MaxStakeReached,
}