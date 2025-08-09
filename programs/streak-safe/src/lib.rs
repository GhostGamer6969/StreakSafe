#![allow(deprecated)]
#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FYVByK3ZxtZCxcFyCJTsybSsMRKcd59mVdiyFpnBN3gE");

#[program]
pub mod streak_safe {
    use super::*;
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        min_stake: u64,
        min_checkins: u64,
        expiry_sec: i64,
        max_checkin_gap_sec: i64,
        min_checkin_gap_sec: i64,
        min_votes: u8,
    ) -> Result<()> {
        ctx.accounts.initialize_config(
            min_stake,
            min_checkins,
            expiry_sec,
            max_checkin_gap_sec,
            min_checkin_gap_sec,
            min_votes,
            &ctx.bumps,
        )
    }

    pub fn initialize_streak(
        ctx: Context<InitializeStreak>,
        categories: u8,
        total_checkins: u16,
        required_checkin: u16,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.initialize_streak(
            categories,
            total_checkins,
            required_checkin,
            amount,
            &ctx.bumps,
        )
    }
}
pub fn fail_streak(ctx: Context<FailStreak>) -> Result<()> {
    ctx.accounts.slash()
}
pub fn complete_streak(ctx: Context<CompleteStreak>) -> Result<()> {
    ctx.accounts.transfer_to_user()
}

pub fn check_in(ctx: Context<CheckIn>) -> Result<()> {

    todo!()
}
