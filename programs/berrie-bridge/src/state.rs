use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct Lock {
    pub pre_token_mint: Pubkey,
    pub post_token_mint: Pubkey,
    pub locked_amount: u64,
}

impl Lock {}
