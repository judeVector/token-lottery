use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
/// The struct representing the state of the token lottery
pub struct TokenLottery {
    pub winner: u64,                // The ID of the winning participant
    pub winner_claimed: bool,       // Indicate whether the prize has been claimed
    pub start_time: u64,            // Timestamp when the lottery begins
    pub end_time: u64,              // Timestamp when the lottery ends
    pub lottery_pot_amount: u64,    // Total amount of tokens collected for the lottery pot
    pub total_tickets: u64,         // Total number of tickets sold
    pub ticket_price: u64,          // Price per ticket in tokens
    pub authority: Pubkey,          // Authority/admin account who can manage the lottery
    pub randomness_account: Pubkey, // Account used for random number generation
    pub bump: u8,                   // PDA bump seed
}
