use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;


#[derive(Accounts)]
pub struct ChangeFeeAddress<'info> {
    #[account(mut, has_one = admin @ PumpError::Unauthorized)]
    pub state: Account<'info, StateAccount>,
    pub admin: Signer<'info>,
}

#[event]
pub struct FeeAddressUpdated {
    pub old_address: Pubkey,
    pub new_address: Pubkey,
    pub timestamp: i64,
}

pub fn change_fee_address(ctx: Context<ChangeFeeAddress>, new_address: Pubkey) -> Result<()> {
    require!(!ctx.accounts.state.paused, PumpError::ContractPaused);
    
    let state = &mut ctx.accounts.state;
    let old_address = state.fee_address;
    state.fee_address = new_address;
    
    emit!(FeeAddressUpdated {
        old_address,
        new_address,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}