use anchor_lang::prelude::*;
use instructions::{
    admin::enable_token::*,
    user::{lock_token::*, unlock_token::*},
};

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU");

#[program]
pub mod berrie_bridge {
    use super::*;

    // Admin instructions

    pub fn enable_token(ctx: Context<EnableToken>, args: EnableTokenArgs) -> Result<()> {
        enable_token_handler(ctx, args)
    }

    // User instructions

    pub fn lock_token(ctx: Context<LockToken>, amount: u64) -> Result<()> {
        lock_token_handler(ctx, amount)
    }

    pub fn unlock_token(ctx: Context<UnlockToken>, amount: u64) -> Result<()> {
        unlock_token_handler(ctx, amount)
    }
}
