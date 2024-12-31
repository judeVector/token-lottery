use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{Metadata, MetadataAccount},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{CustomError, TokenLottery, NAME};

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"token_lottery".as_ref()],
        bump = token_lottery.bump,
    )]
    pub token_lottery: Account<'info, TokenLottery>,

    #[account(
        mut,
        seeds = [b"collection_mint".as_ref()],
        bump,
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [token_lottery.winner.to_le_bytes().as_ref()],
        bump,
    )]
    pub ticket_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [b"metadata", token_metadata_program.key().as_ref(), ticket_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        associated_token::mint = ticket_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), collection_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub collection_metadata: Account<'info, MetadataAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
}

pub fn handler_claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
    msg!(
        "Winner chosen: {}",
        ctx.accounts.token_lottery.winner_chosen
    );

    require!(
        ctx.accounts.token_lottery.winner_chosen,
        CustomError::WinnerNotChosen
    );

    require!(
        ctx.accounts.metadata.collection.as_ref().unwrap().verified,
        CustomError::NotVerifiedTicket
    );

    require!(
        ctx.accounts.metadata.collection.as_ref().unwrap().key
            == ctx.accounts.collection_mint.key(),
        CustomError::IncorrectTicket
    );

    let ticket_name = NAME.to_owned() + &ctx.accounts.token_lottery.winner.to_string();
    let metadata_name = ctx.accounts.metadata.name.replace("\u{0}", "");

    require!(metadata_name == ticket_name, CustomError::IncorrectTicket);
    require!(ctx.accounts.destination.amount > 0, CustomError::NoTicket);

    **ctx
        .accounts
        .token_lottery
        .to_account_info()
        .lamports
        .borrow_mut() -= ctx.accounts.token_lottery.lottery_pot_amount;
    **ctx.accounts.signer.to_account_info().lamports.borrow_mut() +=
        ctx.accounts.token_lottery.lottery_pot_amount;

    ctx.accounts.token_lottery.lottery_pot_amount = 0;

    Ok(())
}
