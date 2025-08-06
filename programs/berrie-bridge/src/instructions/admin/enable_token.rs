use crate::{
    constants::{seeds, ADMIN_PUBKEY},
    state::Lock,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::Token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EnableTokenArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Accounts)]
pub struct EnableToken<'info> {
    #[account(
        mut,
        address = ADMIN_PUBKEY,
    )]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [
            seeds::LOCK_SEED,
            pre_token_mint.key().as_ref(),
        ],
        payer = admin,
        space = 8 + Lock::INIT_SPACE,
        bump,
    )]
    pub lock: Box<Account<'info, Lock>>,

    #[account(
        mut,
        seeds = [
            seeds::METADATA_SEED,
            token_metadata_program.key().as_ref(),
            post_token_mint.key().as_ref(),
        ],
        seeds::program = token_metadata_program.key(),
        bump,
    )]
    pub metadata: SystemAccount<'info>,
    #[account(
        init,
        seeds = [
            seeds::LOCK_TOKEN_SEED,
            pre_token_mint.key().as_ref(),
        ],
        mint::authority = lock,
        mint::decimals = pre_token_mint.decimals,
        mint::token_program = post_token_program,
        payer = admin,
        bump,
    )]
    pub post_token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub pre_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = pre_token_mint,
        associated_token::authority = lock,
        associated_token::token_program = pre_token_program,
    )]
    pub pre_lock_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,

    pub pre_token_program: Interface<'info, TokenInterface>,
    pub post_token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn enable_token_handler(ctx: Context<EnableToken>, args: EnableTokenArgs) -> Result<()> {
    let lock = &mut ctx.accounts.lock;

    lock.pre_token_mint = ctx.accounts.pre_token_mint.key();
    lock.post_token_mint = ctx.accounts.post_token_mint.key();

    let token_data = DataV2 {
        name: args.name.clone(),
        symbol: args.symbol.clone(),
        uri: args.uri.clone(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let pre_token_mint = ctx.accounts.pre_token_mint.key();
    let signer_seeds: &[&[&[u8]]] =
        &[&[seeds::LOCK_SEED, pre_token_mint.as_ref(), &[ctx.bumps.lock]]];

    let metadata_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            payer: ctx.accounts.admin.to_account_info(),
            update_authority: lock.to_account_info(),
            mint: ctx.accounts.post_token_mint.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            mint_authority: lock.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer_seeds,
    );

    create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;

    Ok(())
}
