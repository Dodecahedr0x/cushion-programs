use anchor_lang::prelude::*;

use crate::{
    constants::AUTHORITY_SEED,
    state::{Band, BandDeposit, Llamma, Market},
};

pub fn create_band_deposit(ctx: Context<CreateBandDeposit>) -> Result<()> {
    msg!("Creating a new band deposit");

    let band_deposit = &mut ctx.accounts.band_deposit;
    band_deposit.depositor = ctx.accounts.depositor.key();
    band_deposit.band = ctx.accounts.band.key();

    Ok(())
}

#[derive(Accounts)]
pub struct CreateBandDeposit<'info> {
    #[account(
        seeds = [
            llamma.debt_mint.as_ref()
        ],
        bump,
    )]
    pub llamma: Account<'info, Llamma>,

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
        mut,
        seeds = [
            market.key().as_ref(),
            &band.index.to_le_bytes(),
        ],
        bump
    )]
    pub band: Account<'info, Band>,

    #[account(
        init,
        payer = payer,
        space = BandDeposit::LEN,
        seeds = [
            band.key().as_ref(),
            &depositor.key().as_ref(),
        ],
        bump
    )]
    pub band_deposit: Account<'info, BandDeposit>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
