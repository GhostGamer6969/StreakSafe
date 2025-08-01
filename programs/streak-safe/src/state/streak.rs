use anchor_lang::prelude::*;

#[account]
pub struct Streak {
    categories: u8,
    total_checkins: u16,
    required_checkin: u16,
    start_timestamp: i64,
    status: u8,
    streak_bump: u8,
}
