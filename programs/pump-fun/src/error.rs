use anchor_lang::prelude::*;

#[error_code]
pub enum PumpError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Contract is paused")]
    ContractPaused,
    #[msg("Fee exceeds maximum cap")]
    FeeExceedsMax,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}