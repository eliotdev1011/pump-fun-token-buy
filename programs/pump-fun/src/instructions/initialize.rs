use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + StateAccount::SPACE)]
    pub state: Account<'info, StateAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: This is safe as it's just used to store the fee address
    pub fee_address: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct AdminInitialized {
    pub admin: Pubkey,
    pub fee_address: Pubkey,
    pub fee_amount: u64,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.admin = ctx.accounts.admin.key();
    state.fee_address = ctx.accounts.fee_address.key();
    state.fee_amount = 100;
    state.max_fee_cap = 1000;
    state.paused = false;
    
    emit!(AdminInitialized {
        admin: state.admin,
        fee_address: state.fee_address,
        fee_amount: state.fee_amount,
    });
    
    Ok(())
}