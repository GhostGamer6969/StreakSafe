use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Stake amount is lower than min stake")]
    LowStake,
    
}
