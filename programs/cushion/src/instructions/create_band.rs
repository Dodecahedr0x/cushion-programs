use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::{
    constants::AUTHORITY_SEED,
    state::{Band, Llamma, Market},
};

pub fn create_band(ctx: Context<CreateBand>, index: i16) -> Result<()> {
    msg!("Creating a new band");

    let band = &mut ctx.accounts.band;
    band.market = ctx.accounts.market.key();
    band.index = index;

    Ok(())
}

#[derive(Accounts)]
#[instruction(index: i16)]
pub struct CreateBand<'info> {
    #[account(
        seeds = [
            llamma.debt_mint.as_ref()
        ],
        bump,
    )]
    pub llamma: Account<'info, Llamma>,

    /// CHECK: A read-only seeded authority
    #[account(
        mut,
        seeds = [
            llamma.debt_mint.as_ref(),
            AUTHORITY_SEED.as_ref()
        ],
        bump
    )]
    pub llamma_authority: AccountInfo<'info>,

    #[account(
        seeds = [
            market.llamma.key().as_ref(),
            market.collateral_mint.key().as_ref()
        ],
        bump,
        has_one = llamma,
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = payer,
        space = Band::LEN,
        seeds = [
            market.key().as_ref(),
            &index.to_le_bytes(),
        ],
        bump
    )]
    pub band: Account<'info, Band>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
