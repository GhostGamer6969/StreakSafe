use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorC {
    #[msg("Stake amount is lower than min stake")]
    LowStake,
    #[msg("Streak not Ongoing")]
    NotOngoing,
    #[msg("Last check in is not verified")]
    NotVerified,
}
