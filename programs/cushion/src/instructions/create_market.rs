use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use pyth_sdk_solana::load_price_feed_from_account_info;

use crate::{
    constants::AUTHORITY_SEED,
    errors::CushionError,
    state::{Llamma, Market},
};

pub fn create_market(ctx: Context<CreateMarket>) -> Result<()> {
    msg!("Creating a new market");

    let market = &mut ctx.accounts.market;
    market.llamma = ctx.accounts.llamma.key();
    market.collateral = ctx.accounts.collateral.key();
    market.price_feed = ctx.accounts.price_feed.key();

    // Checking the price feed
    const STALENESS_THRESHOLD: u64 = 60; // staleness threshold in seconds
    let price_account_info = ctx.accounts.price_feed.to_account_info();
    let price_feed = load_price_feed_from_account_info(&price_account_info).unwrap();
    let current_price = price_feed
        .get_price_no_older_than(Clock::get()?.unix_timestamp, STALENESS_THRESHOLD)
        .unwrap();
    let mut price_str = format!("{}", current_price.price);
    price_str.insert((price_str.len() as i32 + current_price.expo) as usize, ',');
    msg!("Price: ~{}", price_str,);

    Ok(())
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account()]
    pub admin: Signer<'info>,

    #[account(
        seeds = [
            llamma.stablecoin.as_ref()
        ],
        bump,
        has_one = admin @ CushionError::NotAdmin,
    )]
    pub llamma: Account<'info, Llamma>,

    /// CHECK: A read-only seeded authority
    #[account(
        mut,
        seeds = [
            llamma.stablecoin.as_ref(),
            AUTHORITY_SEED.as_ref()
        ],
        bump
    )]
    pub llamma_authority: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        space = Market::LEN,
        seeds = [
            llamma.key().as_ref(),
            collateral.key().as_ref()
        ],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub collateral: Account<'info, Mint>,

    /// CHECK: verified by Pyth
    pub price_feed: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
