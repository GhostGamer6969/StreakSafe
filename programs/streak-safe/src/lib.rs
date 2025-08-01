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
    pub fn initialize(ctx: Context<InitializeConfig>) -> Result<()> {
        todo!()
    }
}
