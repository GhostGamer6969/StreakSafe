use anchor_lang::prelude::{
    borsh::{BorshDeserialize, BorshSerialize},
    *,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, InitSpace)]
pub enum Status {
    Ongoing,
    Completed,
    Slashed,
}

#[account]
#[derive(InitSpace)]
pub struct Streak {
    pub categories: u8,
    pub total_checkins: u16,
    pub required_checkin: u16,
    pub start_timestamp: i64,
    pub status: Status,
    pub streak_bump: u8,
}
