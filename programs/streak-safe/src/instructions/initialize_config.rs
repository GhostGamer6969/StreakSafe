use anchor_lang::prelude::*;

use crate::Config;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        space = 8 + Config::INIT_SPACE,
        payer = admin,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(
        &mut self,
        min_stake: u64,
        expiry_sec: i64,
        max_checkin_gap_sec: i64,
        min_checkin_gap_sec: i64,
        min_votes: u8,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            min_stake,
            slash_receiver: Pubkey::from_str_const("DWyWmTCLqfLAzfeiaDZmxVa2Y8qWaehYyHsiFtpPNfND"),
            expiry_sec,
            max_checkin_gap_sec,
            min_checkin_gap_sec,
            min_votes,
            bump: bumps.config,
        });
        Ok(())
    }
}
