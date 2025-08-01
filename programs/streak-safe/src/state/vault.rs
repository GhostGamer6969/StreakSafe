use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    streak_owner: Pubkey,
}
