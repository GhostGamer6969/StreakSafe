use anchor_lang::prelude::*;

#[account]
pub struct Config {
    min_stake: u64,
    slash_receiver: Pubkey,
    expiry_sec: i64,
    max_checkin_gap_sec: i64,
    min_checkin_gap_sec: i64,
    min_votes: u8,
    bump: u8,
}
