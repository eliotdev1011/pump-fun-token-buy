use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::*;
use crate::error::*;

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub state: Account<'info, StateAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: Safe because we're just transferring SOL to this account
    #[account(mut)]
    pub fee_recipient: AccountInfo<'info>,
    /// CHECK: Safe because we're just transferring SOL to this account
    #[account(mut)]
    pub token_recipient: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct TokenPurchased {
    pub buyer: Pubkey,
    pub identifier: String,
    pub amount: u64,
    pub fee_amount: u64,
    pub timestamp: i64,
}

pub fn buy_token(ctx: Context<BuyToken>, identifier: String, amount: u64) -> Result<()> {
    let state = &ctx.accounts.state;
    require!(!state.paused, PumpError::ContractPaused);
    require!(amount > 0, PumpError::InvalidAmount);

    // Calculate fee
    let fee = (amount * state.fee_amount) / 10_000;
    let purchase_amount = amount.checked_sub(fee).unwrap();

    // Ensure buyer has enough funds
    require!(
        ctx.accounts.buyer.lamports() >= amount,
        PumpError::InsufficientFunds
    );

    // Transfer fee to fee recipient
    if fee > 0 {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.fee_recipient.to_account_info(),
                },
            ),
            fee,
        )?;
    }

    // Transfer remaining amount to token recipient
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.token_recipient.to_account_info(),
            },
        ),
        purchase_amount,
    )?;

    emit!(TokenPurchased {
        buyer: ctx.accounts.buyer.key(),
        identifier,
        amount: purchase_amount,
        fee_amount: fee,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}