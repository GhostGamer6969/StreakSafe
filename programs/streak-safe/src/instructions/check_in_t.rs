use anchor_lang::prelude::*;

use crate::{error::ErrorC, Config, LatestCheckIn, Status, Streak, Vault};

#[derive(Accounts)]
#[instruction(uuid_b:u64)]
pub struct CheckInT<'info> {
    #[account(
        mut,
        constraint = user_b.key() == Pubkey::from_str_const("DWyWmTCLqfLAzfeiaDZmxVa2Y8qWaehYyHsiFtpPNfND")
    )]
    pub user_b: Signer<'info>,

    #[account(
        constraint = slash_reciever.key() == config.slash_receiver,
    )]
    pub slash_reciever: SystemAccount<'info>,

    #[account(
        init,
        space = 8 + LatestCheckIn::INIT_SPACE,
        payer = user_b,
        seeds = [b"check_in",streak_b.key().as_ref()],
        bump,
    )]
    pub latest_checkin_b: Account<'info, LatestCheckIn>,

    #[account(
        mut,
        // close = user_b,
        seeds = [b"streak",user_b.key().as_ref(),uuid_b.to_le_bytes().as_ref()],
        bump = streak_b.streak_bump,
    )]
    pub streak_b: Account<'info, Streak>,

    #[account(
        mut,
        // close = user_b,
        seeds = [b"vault",streak_b.key().as_ref()],
        bump = vault_b.bump,
    )]
    pub vault_b: Account<'info, Vault>,

    #[account(
        seeds = [b"config"],
        bump= config.bump,
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

impl<'info> CheckInT<'info> {
    pub fn check_in_t(&mut self, _uuid_b: u64, image: String, bumps: &CheckInTBumps) -> Result<()> {
        require!(
            self.streak_b.total_checkins < self.streak_b.required_checkin,
            ErrorC::NotOngoing
        );

        require!(self.streak_b.status == Status::Ongoing, ErrorC::NotOngoing);
        self.latest_checkin_b.set_inner(LatestCheckIn {
            check_in_time: Clock::get()?.unix_timestamp,
            votes: [0, 0],
            image,
            bump: bumps.latest_checkin_b,
        });

        Ok(())
        // self.verify(is_accept)
    }
}
