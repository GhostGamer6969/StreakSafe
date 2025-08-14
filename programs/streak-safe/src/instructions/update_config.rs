use anchor_lang::prelude::*;

use crate::{error::ErrorC, Config};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        constraint = admin.key() == Pubkey::from_str_const("DWyWmTCLqfLAzfeiaDZmxVa2Y8qWaehYyHsiFtpPNfND")
    )]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config"],
        bump=config.bump,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}
impl<'info> UpdateConfig<'info> {
    pub fn update_config(
        &mut self,
        min_stake: Option<u64>,
        min_checkins: Option<u64>,
        expiry_sec: Option<i64>,
        min_votes: Option<u8>,
        // bumps: &UpdateConfigBumps,
    ) -> Result<()> {
        require!(
            self.admin.key()
                == Pubkey::from_str_const("DWyWmTCLqfLAzfeiaDZmxVa2Y8qWaehYyHsiFtpPNfND"),
            ErrorC::NotAdmin
        );

        self.config.set_inner(Config {
            min_stake: min_stake.unwrap_or(self.config.min_stake),
            min_checkins: min_checkins.unwrap_or(self.config.min_checkins),
            slash_receiver: Pubkey::from_str_const("DWyWmTCLqfLAzfeiaDZmxVa2Y8qWaehYyHsiFtpPNfND"),
            expiry_sec: expiry_sec.unwrap_or(self.config.expiry_sec),
            min_votes: min_votes.unwrap_or(self.config.min_votes),
            bump: self.config.bump,
        });
        Ok(())
    }
}
