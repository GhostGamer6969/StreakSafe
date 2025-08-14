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
        min_votes: u8,
    ) -> Result<()> {
        ctx.accounts
            .initialize_config(min_stake, min_checkins, expiry_sec, min_votes, &ctx.bumps)
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        min_stake: Option<u64>,
        min_checkins: Option<u64>,
        expiry_sec: Option<i64>,
        min_votes: Option<u8>,
    ) -> Result<()> {
        ctx.accounts.update_config(
            min_stake,
            min_checkins,
            expiry_sec,
            min_votes,
            // &ctx.bumps,
        )
    }

    pub fn initialize_streak(
        ctx: Context<InitializeStreak>,
        uuid: u64,
        categories: u8,
        required_checkin: u16,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .initialize_streak(uuid, categories, required_checkin, amount, &ctx.bumps)
    }

    pub fn check_in_test(ctx: Context<CheckInT>, uuid_b: u64, image: String) -> Result<()> {
        ctx.accounts.check_in_t(uuid_b, image, &ctx.bumps)
    }

    pub fn check_in(
        ctx: Context<CheckIn>,
        uuid: u64,
        uuid_b: u64,
        image: String,
        is_accept: bool,
    ) -> Result<()> {
        ctx.accounts
            .check_in(uuid, uuid_b, image, is_accept, &ctx.bumps)
    }
}
