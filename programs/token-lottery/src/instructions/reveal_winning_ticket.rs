use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{CustomError, TokenLottery};

#[derive(Accounts)]
pub struct RevealWinningTicket<'info> {
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

pub fn handler_reveal_winning_ticket(ctx: Context<RevealWinningTicket>) -> Result<()> {
    let clock = Clock::get()?;
    let token_lottery = &mut ctx.accounts.token_lottery;

    if ctx.accounts.signer.key() != token_lottery.authority {
        return Err(CustomError::Unauthorized.into());
    }

    if ctx.accounts.randomness_account.key() != token_lottery.randomness_account {
        return Err(CustomError::IncorrectRandomnessAccount.into());
    }

    if clock.slot < token_lottery.end_time {
        return Err(CustomError::LotteryNotCompleted.into());
    }

    require!(!token_lottery.winner_chosen, CustomError::WinnerClaimed);

    let randomness_data =
        RandomnessAccountData::parse(ctx.accounts.randomness_account.data.borrow()).unwrap();

    let reveal_random_value = randomness_data
        .get_value(&clock)
        .map_err(|_| CustomError::RandomnessNotResolved)?;

    let winner = reveal_random_value[0] as u64 % token_lottery.total_tickets;

    msg!("Winner chosen: {}", winner);

    token_lottery.winner = winner;
    token_lottery.winner_chosen = true;

    Ok(())
}
