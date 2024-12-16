use anchor_lang::prelude::*;
pub mod state;
pub mod error;
pub mod instructions;

use crate::instructions::*;

declare_id!("9Be39dF3DELzSqEYtsPqrw7GAyz5AggzQrM2hoBLs1H1");

#[program]
pub mod pump_fun {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn change_fee(ctx: Context<ChangeFee>, new_fee: u64) -> Result<()> {
        instructions::change_fee(ctx, new_fee)
    }

    pub fn change_fee_address(ctx: Context<ChangeFeeAddress>, new_address: Pubkey) -> Result<()> {
        instructions::change_fee_address(ctx, new_address)
    }

    pub fn pause_contract(ctx: Context<PauseContract>, state: bool) -> Result<()> {
        instructions::pause_contract(ctx, state)
    }

    pub fn buy_token(ctx: Context<BuyToken>, identifier: String, amount: u64) -> Result<()> {
        instructions::buy_token(ctx, identifier, amount)
    }
}