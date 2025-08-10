use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace, PartialEq, Eq)]
pub enum Status {
    Ongoing,
    Completed,
    Failed,
}

#[account]
#[derive(InitSpace)]
pub struct Streak {
    pub categories: u8,
    pub start_timestamp: i64,
    pub total_checkins: u16,
    pub required_checkin: u16,
    pub status: Status,
    pub streak_bump: u8,
}
