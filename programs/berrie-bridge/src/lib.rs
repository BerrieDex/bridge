use anchor_lang::prelude::*;
use instructions::{
    admin::enable_token::*,
    user::{lock_token::*, unlock_token::*},
};

#[allow(unused_imports)]
use solana_security_txt::security_txt;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Berrie Staking Program",
    project_url: "https://berr.ie/",
    contacts: "twitter:BerrieOrg",
    policy: "https://berrie.gitbook.io/berrie/privacy-policy",
    preferred_languages: "en",
    source_code: "https://github.com/BerrieDex/bridge/"
}


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
