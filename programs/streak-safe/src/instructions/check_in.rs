use anchor_lang::prelude::*;

use crate::{Config, LatestCheckIn, Streak};

#[derive(Accounts)]
#[instruction(uuid:u64)]
pub struct CheckIn<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        space = 8 + LatestCheckIn::INIT_SPACE,
        payer = user,
        seeds = [b"check_in",streak.key().as_ref()],
        bump,
    )]
    pub latest_checkin: Account<'info, LatestCheckIn>,

    #[account(
        seeds = [b"streak",user.key().as_ref(),uuid.to_le_bytes().as_ref()],
        bump = streak.streak_bump,
    )]
    pub streak: Account<'info, Streak>,

    #[account(
        seeds = [b"config"],
        bump= config.bump,
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}
