use anchor_lang::prelude::*;

use crate::{TokenLottery, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + TokenLottery::INIT_SPACE,
        seeds = [b"token_lottery".as_ref()],
        bump
    )]
    pub token_lottery: Account<'info, TokenLottery>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, start: u64, end: u64, price: u64) -> Result<()> {
    ctx.accounts.token_lottery.set_inner(TokenLottery {
        winner: 0,
        winner_claimed: false,
        start_time: start,
        end_time: end,
        lottery_pot_amount: 0,
        total_ticket: 0,
        ticket_price: price,
        authority: ctx.accounts.signer.key(),
        randomness_account: Pubkey::default(),
        bump: ctx.bumps.token_lottery,
    });

    Ok(())
}
