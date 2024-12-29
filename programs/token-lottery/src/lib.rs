pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("HnBGF1BEDWFwkkF2wYc2GvZ7f9EXsYtdeYbh263zD3En");

#[program]
pub mod token_lottery {
    use super::*;

    pub fn initialize_config(
        ctx: Context<Initialize>,
        start: u64,
        end: u64,
        price: u64,
    ) -> Result<()> {
        initialize_config::handler_initialize_config(ctx, start, end, price)
    }

    pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<()> {
        initialize_lottery::handler_initialize_lottery(ctx)
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        buy_ticket::handler_buy_ticket(ctx)
    }

    pub fn commit_randomness(ctx: Context<CommitRandomness>) -> Result<()> {
        commit_randomness::handler_commit_randomness(ctx)
    }

    pub fn reveal_winning_ticket(ctx: Context<RevealWinningTicket>) -> Result<()> {
        reveal_winning_ticket::handler_reveal_winning_ticket(ctx)
    }
}
