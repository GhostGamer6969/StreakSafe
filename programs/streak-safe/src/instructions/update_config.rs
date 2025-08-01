use anchor_lang::prelude::*;

use crate::{Config, InitializeConfigBumps};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}
impl<'info> UpdateConfig<'info> {
    pub fn update_config(
        &mut self,
        min_stake: Option<u64>,
        expiry_sec: Option<i64>,
        max_checkin_gap_sec: Option<i64>,
        min_checkin_gap_sec: Option<i64>,
        min_votes: Option<u8>,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            min_stake: min_stake.unwrap(),
            slash_receiver: Pubkey::from_str_const("DWyWmTCLqfLAzfeiaDZmxVa2Y8qWaehYyHsiFtpPNfND"),
            expiry_sec: expiry_sec.unwrap(),
            max_checkin_gap_sec: max_checkin_gap_sec.unwrap(),
            min_checkin_gap_sec: min_checkin_gap_sec.unwrap(),
            min_votes: min_votes.unwrap(),
            bump: bumps.config,
        });
        todo!()
    }
}
