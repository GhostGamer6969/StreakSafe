use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub min_stake: u64,
    pub slash_receiver: Pubkey,
    pub min_checkins: u64,
    pub expiry_sec: i64,
    pub min_votes: u8,
    pub bump: u8,
}
