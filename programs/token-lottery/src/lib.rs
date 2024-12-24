pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
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
}
