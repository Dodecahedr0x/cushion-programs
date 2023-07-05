use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use fixed_point_decimal_math::BigNumber;
use pyth_sdk_solana::load_price_feed_from_account_info;

use crate::{
    constants::{AUTHORITY_SEED, MINIMUM_LIQUIDITY, STALENESS_THRESHOLD},
    errors::CushionError,
    state::{Band, Llamma, Market},
};

pub fn create_band(ctx: Context<CreateBand>, index: i16) -> Result<()> {
    msg!("Creating a new band");

    let amplification = ctx.accounts.market.amplification as u64;
    let base_price = ctx.accounts.market.base_price;

    let price_feed = load_price_feed_from_account_info(&ctx.accounts.price_feed.to_account_info())
        .map_err(|_| CushionError::InvalidPriceFeed)?;
    let current_price: pyth_sdk_solana::Price = price_feed
        .get_ema_price_no_older_than(Clock::get()?.unix_timestamp, STALENESS_THRESHOLD)
        .ok_or_else(|| CushionError::OutdatedPrice)?;
    let price_oracle = BigNumber::new(current_price.price as u64, current_price.expo.abs() as u8);
    let collateral_price = price_oracle.mul(&BigNumber::new(
        MINIMUM_LIQUIDITY,
        ctx.accounts.collateral_mint.decimals,
    ));

    // p_{u, n} = p_{base} ((A - 1) / A)^n
    // TODO: Fees
    let upper_band_price = BigNumber::unit(ctx.accounts.debt_mint.decimals)
        .mul(&BigNumber::new(amplification - 1, 0))
        .pow(index as i16)
        .div(&BigNumber::new(amplification, 0).pow(index as i16))
        .mul(&BigNumber::new(base_price, current_price.expo.abs() as u8));

    if price_oracle < upper_band_price {
        return err!(CushionError::PriceTooLowForCreation);
    }

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.creator_account.to_account_info(),
                to: ctx.accounts.llamma_account.to_account_info(),
                authority: ctx.accounts.creator.to_account_info(),
            },
        ),
        MINIMUM_LIQUIDITY,
    )?;

    msg!(
        "{} collateral = {} debt ({})",
        BigNumber::new(MINIMUM_LIQUIDITY, ctx.accounts.collateral_mint.decimals),
        collateral_price,
        collateral_price.value
    );

    let band = &mut ctx.accounts.band;
    band.market = ctx.accounts.market.key();
    band.index = index;
    band.collateral_amount = MINIMUM_LIQUIDITY;
    band.debt_amount = collateral_price.value;

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
    pub creator: Signer<'info>,

    #[account(
      init_if_needed,
      payer = payer,
      associated_token::mint = collateral_mint,
      associated_token::authority = creator,
    )]
    pub creator_account: Box<Account<'info, TokenAccount>>,

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
