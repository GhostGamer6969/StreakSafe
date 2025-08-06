use anchor_lang::prelude::*;

use crate::{error::ErrorC, Config, LatestCheckIn, Status, Streak};

#[derive(Accounts)]
#[instruction(uuid:u64,uuid_b:u64)]
pub struct CheckIn<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub user_b: SystemAccount<'info>,

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

    #[account(
        mut,
        seeds = [b"check_in",user_b.key().as_ref(),uuid_b.to_le_bytes().as_ref()],
        bump = latest_checkin_b.bump,
    )]
    pub latest_checkin_b: Account<'info, LatestCheckIn>,

    pub system_program: Program<'info, System>,
}

impl<'info> CheckIn<'info> {
    pub fn check_in(&mut self, image: String, bumps: &CheckInBumps) -> Result<()> {
        require!(
            self.streak.total_checkins < self.streak.required_checkin,
            ErrorC::NotOngoing
        );
        require!(self.streak.status == Status::Ongoing, ErrorC::NotOngoing);
        match self.latest_checkin.to_account_info().data_is_empty() {
            true => {
                self.latest_checkin.set_inner(LatestCheckIn {
                    check_in_time: Clock::get()?.unix_timestamp,
                    votes: [0, 0],
                    image,
                    bump: bumps.latest_checkin,
                });
            }
            false => {
                require!(
                    (Clock::get()?.unix_timestamp - self.latest_checkin.check_in_time)
                        < self.config.expiry_sec,
                    ErrorC::NotVerified
                );
                self.streak.status = Status::Failed
            }
        }
        Ok(())
    }

    pub fn verify(&mut self, is_accept: bool) -> Result<()> {
        match is_accept {
            true => self.latest_checkin_b.votes[0] += 1,
            false => self.latest_checkin_b.votes[1] += 1,
        }
        todo!()
    }
}
