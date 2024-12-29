use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{CustomError, TokenLottery};

#[derive(Accounts)]
pub struct CommitRandomness<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"token_lottery".as_ref()],
        bump
    )]
    pub token_lottery: Account<'info, TokenLottery>,

    /// CHECK: This account is checked by the Switchboard smart contract
    pub randomness_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler_commit_randomness(ctx: Context<CommitRandomness>) -> Result<()> {
    let clock = Clock::get()?;
    let token_lottery = &mut ctx.accounts.token_lottery;

    if ctx.accounts.signer.key() != token_lottery.authority {
        return Err(CustomError::NotAuthorized.into());
    }

    let randomness_data =
        RandomnessAccountData::parse(ctx.accounts.randomness_account.data.borrow()).unwrap();

    if randomness_data.seed_slot != clock.slot - 1 {
        return Err(CustomError::RandomnessAlreadyRevealed.into());
    }

    token_lottery.randomness_account = ctx.accounts.randomness_account.key();

    Ok(())
}
