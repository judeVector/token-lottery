use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Lottery not open")]
    LotteryNotOpen,
    #[msg("Not authorized")]
    NotAuthorized,
    #[msg("Invalid ticket index")]
    InvalidTicketIndex,
    #[msg("Randomness already revealed")]
    RandomnessAlreadyRevealed,
}
