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
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid state")]
    InvalidState,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Incorrect randomness account")]
    IncorrectRandomnessAccount,
    #[msg("Invalid ticket price")]
    InvalidTicketPrice,
    #[msg("Lottery not completed")]
    LotteryNotCompleted,
    #[msg("Winner already claimed")]
    WinnerClaimed,
    #[msg("Randomness not resolved")]
    RandomnessNotResolved,
}
