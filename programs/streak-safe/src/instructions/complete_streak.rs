use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

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

impl<'info> CompleteStreak<'info> {
    pub fn transfer_to_user(&mut self) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        // [b"vault", streak.key().as_ref()]
        let binding = self.streak.key().clone();
        let seeds = &[&b"vault"[..], binding.as_ref(), &[self.vault.bump]];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer(cpi_context, self.vault.get_lamports())
    }
}
