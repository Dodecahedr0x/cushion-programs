use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{constants::AUTHORITY_SEED, state::Llamma};

pub fn initialize_llamma(ctx: Context<InitializeLlamma>) -> Result<()> {
    msg!("Initializing LLAMMA");

    let llamma = &mut ctx.accounts.llamma;
    llamma.admin = ctx.accounts.admin.key();
    llamma.stablecoin = ctx.accounts.stablecoin.key();

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeLlamma<'info> {
    /// CHECK: Read-only admin
    pub admin: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        space = Llamma::LEN,
        seeds = [
            &stablecoin.key().as_ref()
        ],
        bump
    )]
    pub llamma: Account<'info, Llamma>,

    /// CHECK: A read-only seeded authority
    #[account(
        mut,
        seeds = [
            &stablecoin.key().as_ref(),
            AUTHORITY_SEED.as_ref()
        ],
        bump
    )]
    pub llamma_authority: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        mint::authority = llamma_authority,
        mint::decimals = 8
    )]
    pub stablecoin: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
