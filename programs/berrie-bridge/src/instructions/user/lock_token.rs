use crate::{constants::seeds, error::BerrieBridgeError, state::Lock};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_2022::MintTo,
    token_interface::{
        mint_to, transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

#[derive(Accounts)]
pub struct LockToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            seeds::LOCK_SEED,
            pre_token_mint.key().as_ref(),
        ],
        bump,
        has_one = post_token_mint,
    )]
    pub lock: Box<Account<'info, Lock>>,

    #[account(mut)]
    pub post_token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub pre_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = post_token_mint,
        associated_token::authority = user,
        associated_token::token_program = post_token_program,
    )]
    pub post_user_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = pre_token_mint,
        associated_token::authority = user,
        associated_token::token_program = pre_token_program,
    )]
    pub pre_user_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = pre_token_mint,
        associated_token::authority = lock,
        associated_token::token_program = pre_token_program,
    )]
    pub pre_lock_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub pre_token_program: Interface<'info, TokenInterface>,
    pub post_token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn lock_token_handler(ctx: Context<LockToken>, amount: u64) -> Result<()> {
    let lock = &mut ctx.accounts.lock;

    let user_balance = ctx.accounts.pre_user_ata.amount;
    let lock_amount = amount.min(user_balance);

    require!(lock_amount > 0, BerrieBridgeError::InvalidAmount);

    transfer_checked(
        CpiContext::new(
            ctx.accounts.pre_token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.pre_user_ata.to_account_info(),
                to: ctx.accounts.pre_lock_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
                mint: ctx.accounts.pre_token_mint.to_account_info(),
            },
        ),
        lock_amount,
        ctx.accounts.pre_token_mint.decimals,
    )?;

    let pre_token_mint = ctx.accounts.pre_token_mint.key();
    let signer_seeds: &[&[&[u8]]] =
        &[&[seeds::LOCK_SEED, pre_token_mint.as_ref(), &[ctx.bumps.lock]]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.post_token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.post_token_mint.to_account_info(),
                to: ctx.accounts.post_user_ata.to_account_info(),
                authority: lock.to_account_info(),
            },
            signer_seeds,
        ),
        lock_amount,
    )?;

    lock.locked_amount += lock_amount;

    Ok(())
}
