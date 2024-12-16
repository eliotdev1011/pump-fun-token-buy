use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;

#[derive(Accounts)]
pub struct ChangeFee<'info> {
    #[account(mut, has_one = admin @ PumpError::Unauthorized)]
    pub state: Account<'info, StateAccount>,
    pub admin: Signer<'info>,
}

#[event]
pub struct FeeUpdated {
    pub new_fee: u64,
    pub timestamp: i64,
}

pub fn change_fee(ctx: Context<ChangeFee>, new_fee: u64) -> Result<()> {
    require!(!ctx.accounts.state.paused, PumpError::ContractPaused);
    require!(new_fee <= ctx.accounts.state.max_fee_cap, PumpError::FeeExceedsMax);
    
    let state = &mut ctx.accounts.state;
    state.fee_amount = new_fee;
    
    emit!(FeeUpdated {
        new_fee,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}