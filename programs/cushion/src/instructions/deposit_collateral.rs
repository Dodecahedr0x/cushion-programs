use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use pyth_sdk_solana::load_price_feed_from_account_info;

use crate::{
    constants::{AUTHORITY_SEED, STALENESS_THRESHOLD},
    errors::CushionError,
    math::BigNumber,
    state::{Band, BandDeposit, Llamma, Market},
};

pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
    msg!("Creating a new band");

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.depositor_account.to_account_info(),
                to: ctx.accounts.llamma_account.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        ),
        amount,
    )?;

    let market = &mut ctx.accounts.market;
    let band = &mut ctx.accounts.band;
    let decimals = ctx.accounts.debt_mint.decimals;

    let price_feed = load_price_feed_from_account_info(&ctx.accounts.price_feed.to_account_info())
        .map_err(|_| CushionError::InvalidPriceFeed)?;
    let current_price: pyth_sdk_solana::Price = price_feed
        .get_ema_price_no_older_than(Clock::get()?.unix_timestamp, STALENESS_THRESHOLD)
        .ok_or_else(|| CushionError::OutdatedPrice)?;
    let price_oracle = BigNumber::new(current_price.price as u64, current_price.expo.abs() as u8);

    // p_u = p_{base} ((A - 1) / A)^n
    let upper_band_price = BigNumber::unit(decimals)
        .mul(&BigNumber::new(market.amplification as u64 - 1, 0))
        .pow(band.index as i16)
        .div(&BigNumber::new(market.amplification as u64, 0).pow(band.index as i16))
        .mul(&BigNumber::new(
            market.base_price,
            current_price.expo.abs() as u8,
        ));
    // p_d = p_{base} ((A - 1) / A)^(n+1)
    let lower_band_price = BigNumber::unit(decimals)
        .mul(&BigNumber::new(market.amplification as u64 - 1, 0))
        .pow((band.index + 1) as i16)
        .div(&BigNumber::new(market.amplification as u64, 0).pow((band.index + 1) as i16))
        .mul(&BigNumber::new(
            market.base_price,
            current_price.expo.abs() as u8,
        ));
    // x_d = x + y * sqrt(p_d * p)

    msg!(
        "Band@{}: [{}; {}] (Oracle price = {})",
        band.index,
        lower_band_price,
        upper_band_price,
        price_oracle
    );

    if price_oracle < lower_band_price {
        return err!(CushionError::LiquidatingBand);
    }

    let band_deposit = &mut ctx.accounts.band_deposit;
    band_deposit.depositor = ctx.accounts.depositor.key();
    band_deposit.band = ctx.accounts.band.key();
    band_deposit.deposited_amount += amount;

    Ok(())
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(
        seeds = [
            llamma.debt_mint.as_ref()
        ],
        bump,
        has_one = debt_mint,
    )]
    pub llamma: Account<'info, Llamma>,

    /// CHECK: A read-only seeded authority
    #[account(
        mut,
        seeds = [
            llamma.debt_mint.as_ref(),
            AUTHORITY_SEED.as_ref()
        ],
        bump,
    )]
    pub llamma_authority: AccountInfo<'info>,

    #[account(
        seeds = [
            market.llamma.key().as_ref(),
            market.collateral_mint.key().as_ref()
        ],
        bump,
        has_one = llamma,
        has_one = collateral_mint,
        has_one = price_feed,
    )]
    pub market: Account<'info, Market>,

    /// CHECK: verified by Pyth
    pub price_feed: AccountInfo<'info>,

    #[account(mut)]
    pub debt_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub collateral_mint: Box<Account<'info, Mint>>,

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
        init_if_needed,
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

    #[account(
      init_if_needed,
      payer = payer,
      associated_token::mint = collateral_mint,
      associated_token::authority = depositor,
    )]
    pub depositor_account: Box<Account<'info, TokenAccount>>,

    #[account(
      init_if_needed,
      payer = payer,
      associated_token::mint = collateral_mint,
      associated_token::authority = llamma_authority,
    )]
    pub llamma_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
