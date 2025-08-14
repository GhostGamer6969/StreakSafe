use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{error::ErrorC, Config, LatestCheckIn, Status, Streak, Vault};

#[derive(Accounts)]
#[instruction(uuid:u64,uuid_b:u64)]
pub struct CheckIn<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub user_b: SystemAccount<'info>,

    #[account(
        constraint = slash_reciever.key() == config.slash_receiver,
    )]
    pub slash_reciever: SystemAccount<'info>,

    #[account(
        init_if_needed,
        space = 8 + LatestCheckIn::INIT_SPACE,
        payer = user,
        seeds = [b"check_in",streak.key().as_ref()],
        bump,
    )]
    pub latest_checkin: Account<'info, LatestCheckIn>,

    #[account(
        mut,
        // close = user,
        seeds = [b"streak",user.key().as_ref(),uuid.to_le_bytes().as_ref()],
        bump = streak.streak_bump,
    )]
    pub streak: Account<'info, Streak>,

    #[account(
        mut,
        // close = user,
        seeds = [b"vault",streak.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        seeds = [b"config"],
        bump= config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        // close = slash_reciever,
        seeds = [b"streak",user_b.key().as_ref(),uuid_b.to_le_bytes().as_ref()],
        bump = streak_b.streak_bump,
    )]
    pub streak_b: Account<'info, Streak>,

    #[account(
        mut,
        // close = slash_reciever,
        seeds = [b"vault",streak_b.key().as_ref()],
        bump = vault_b.bump,
    )]
    pub vault_b: Account<'info, Vault>,

    #[account(
        mut,
        // close = user,
        constraint = latest_checkin.key() != latest_checkin_b.key(),
        seeds = [b"check_in",streak_b.key().as_ref()],
        bump = latest_checkin_b.bump,
    )]
    pub latest_checkin_b: Account<'info, LatestCheckIn>,
    pub system_program: Program<'info, System>,
}

impl<'info> CheckIn<'info> {
    pub fn check_in(
        &mut self,
        _uuid: u64,
        _uuid_b: u64,
        image: String,
        is_accept: bool,
        bumps: &CheckInBumps,
    ) -> Result<()> {
        require!(
            self.streak.total_checkins < self.streak.required_checkin,
            ErrorC::NotOngoing
        );

        match self.streak.status {
            Status::Ongoing => {}
            Status::Completed => self.complete_streak()?,
            Status::Failed => self.fail_streak()?,
        };

        require!(self.streak.status == Status::Ongoing, ErrorC::NotOngoing);

        match self.latest_checkin.image.is_empty() {
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
                        > self.config.expiry_sec,
                    ErrorC::NotVerified
                );
                self.streak.status = Status::Failed;
            }
        }
        self.verify(is_accept)
    }

    pub fn verify(&mut self, is_accept: bool) -> Result<()> {
        require!(
            (Clock::get()?.unix_timestamp - self.latest_checkin_b.check_in_time)
                < self.config.expiry_sec,
            ErrorC::ExpiredCheckIn
        );

        require!(
            self.streak.categories == self.streak_b.categories,
            ErrorC::NotSameCategory
        );

        match is_accept {
            true => self.latest_checkin_b.votes[0] += 1,
            false => self.latest_checkin_b.votes[1] += 1,
        }

        let total_votes = self.latest_checkin_b.votes[0] + self.latest_checkin_b.votes[1];

        match total_votes > self.config.min_votes as u64 {
            true => match self.latest_checkin_b.votes[0]
                .checked_mul(100)
                .unwrap()
                .checked_div(total_votes)
                .unwrap()
                > 51
            {
                true => {
                    self.latest_checkin_b.exit(&self.user.key())?;
                    self.streak_b.total_checkins += 1;
                }
                false => {
                    self.streak_b.status = Status::Failed;
                    self.fail_streak_b()?;
                }
            },
            false => {}
        }

        Ok(())
    }

    pub fn complete_streak(&mut self) -> Result<()> {
        require!(
            self.streak.status == Status::Completed,
            ErrorC::NotCompleted
        );

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let binding = self.streak.key().clone();
        let seeds = &[&b"vault"[..], &binding.as_ref(), &[self.vault.bump]];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer(cpi_context, self.vault.get_lamports())?;

        self.latest_checkin.exit(&self.user.key())?;
        Ok(())
    }

    pub fn fail_streak(&mut self) -> Result<()> {
        require!(self.streak.status == Status::Failed, ErrorC::NotFailed);
        require!(
            self.slash_reciever.key() == self.config.slash_receiver,
            ErrorC::NotSlashReciver
        );

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.slash_reciever.to_account_info(),
        };

        let binding = self.streak.key().clone();

        let seeds = &[&b"vault"[..], binding.as_ref(), &[self.vault.bump]];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer(cpi_context, self.vault.get_lamports())?;
        self.latest_checkin.exit(&self.user.key())?;

        Ok(())
    }
    pub fn fail_streak_b(&mut self) -> Result<()> {
        require!(self.streak_b.status == Status::Failed, ErrorC::NotFailed);
        require!(
            self.slash_reciever.key() == self.config.slash_receiver,
            ErrorC::NotSlashReciver
        );

        let cpi_accounts = Transfer {
            from: self.vault_b.to_account_info(),
            to: self.slash_reciever.to_account_info(),
        };

        let binding = self.streak_b.key().clone();

        let seeds = &[&b"vault"[..], binding.as_ref(), &[self.vault_b.bump]];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer(cpi_context, self.vault.get_lamports())?;

        self.latest_checkin_b.exit(&self.user_b.key())?;
        Ok(())
    }
}
