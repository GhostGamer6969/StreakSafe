use anchor_lang::prelude::*;

use crate::{LatestCheckIn, Streak, Vault};

#[derive(Accounts)]
#[instruction(uuid:u64)]
pub struct CompleteStreak<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [b"streak",user.key().as_ref(),uuid.to_le_bytes().as_ref()],
        bump=streak.streak_bump,
    )]
    pub streak: Account<'info, Streak>,

    #[account(
        mut,
        close = user,
        seeds = [b"vault",streak.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        close = user,
        seeds = [b"check_in",streak.key().as_ref()],
        bump = latest_check_in.bump,
    )]
    pub latest_check_in: Account<'info, LatestCheckIn>,

    pub system_program: Program<'info, System>,
}
