use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;

#[derive(Accounts)]
pub struct PauseContract<'info> {
    #[account(mut, has_one = admin @ PumpError::Unauthorized)]
    pub state: Account<'info, StateAccount>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetPauseState<'info> {
    pub state: Account<'info, StateAccount>,
}

#[event]
pub struct ContractPauseUpdated {
    pub paused: bool,
    pub timestamp: i64,
}

pub fn pause_contract(ctx: Context<PauseContract>, paused: bool) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.paused = paused;
    
    emit!(ContractPauseUpdated {
        paused,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}

pub fn get_pause_state(ctx: Context<GetPauseState>) -> Result<bool> {
    Ok(ctx.accounts.state.paused)
}