use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LatestCheckIn {
    pub check_in_time: i64,
    pub votes: [u64; 2],
    #[max_len(44)]
    pub image: String,
    pub bump: u8,
}
