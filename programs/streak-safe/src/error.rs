use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorC {
    #[msg("You are not the admin so you cant do that")]
    NotAdmin,

    #[msg("Stake amount is lower than min stake")]
    LowStake,

    #[msg("Streak not Ongoing")]
    NotOngoing,

    #[msg("Last check in is not verified")]
    NotVerified,

    #[msg("Last Checkin has expired")]
    ExpiredCheckIn,

    #[msg("Streak not Failed")]
    NotFailed,

    #[msg("Streak not Copleted")]
    NotCompleted,

    #[msg("The reciever doesnt match")]
    NotSlashReciver,

    #[msg("The streak trying to verify is not of same category")]
    NotSameCategory,
}
