use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{Status, Streak, Vault};

#[derive(Accounts)]
#[instruction(uuid:u64)]
pub struct InitializeStreak<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        space = 8 + Streak::INIT_SPACE,
        payer = user,
        seeds = [b"streak",user.key().as_ref(),uuid.to_le_bytes().as_ref()],
        bump,
    )]
    pub streak: Account<'info, Streak>,

    #[account(
        init,
        space = 8 + Vault::INIT_SPACE,
        payer = user,
        seeds = [b"vault",streak.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeStreak<'info> {
    pub fn initialize_streak(
        &mut self,
        categories: u8,
        total_checkins: u16,
        required_checkin: u16,
        amount: u64,
        bumps: InitializeStreakBumps,
    ) -> Result<()> {
        self.streak.set_inner(Streak {
            categories,
            total_checkins,
            required_checkin,
            start_timestamp: Clock::get()?.unix_timestamp,
            status: Status::Ongoing,
            streak_bump: bumps.streak,
        });
        self.initialize_vault()?;
        self.transfer_to_vault(amount)
    }

    pub fn initialize_vault(&mut self) -> Result<()> {
        self.vault.set_inner(Vault {
            streak_owner: self.user.key(),
        });
        Ok(())
    }

    pub fn transfer_to_vault(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        transfer(cpi_context, amount)
    }
}
