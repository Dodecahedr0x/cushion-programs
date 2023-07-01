use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use pyth_sdk_solana::load_price_feed_from_account_info;

use crate::{
    constants::AUTHORITY_SEED,
    errors::CushionError,
    state::{Llamma, Market},
};

pub fn create_market(ctx: Context<CreateMarket>, base_price: u64) -> Result<()> {
    msg!("Creating a new market");

    let market = &mut ctx.accounts.market;
    market.llamma = ctx.accounts.llamma.key();
    market.collateral_mint = ctx.accounts.collateral_mint.key();
    market.price_feed = ctx.accounts.price_feed.key();
    market.base_price = base_price;

    // Checking the price feed
    const STALENESS_THRESHOLD: u64 = 86400; // staleness threshold in seconds
    let price_account_info = ctx.accounts.price_feed.to_account_info();
    let price_feed = load_price_feed_from_account_info(&price_account_info).unwrap();
    let current_time = Clock::get()?.unix_timestamp;
    let current_price: pyth_sdk_solana::Price = price_feed
        .get_ema_price_no_older_than(current_time, STALENESS_THRESHOLD)
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
            llamma.debt_mint.as_ref()
        ],
        bump,
        has_one = admin @ CushionError::NotAdmin,
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
        init,
        payer = payer,
        space = Market::LEN,
        seeds = [
            llamma.key().as_ref(),
            collateral_mint.key().as_ref()
        ],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub debt_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub collateral_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = debt_mint,
        associated_token::authority = llamma_authority,
    )]
    pub debt_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = collateral_mint,
        associated_token::authority = llamma_authority,
    )]
    pub collateral_account: Account<'info, TokenAccount>,

    /// CHECK: verified by Pyth
    pub price_feed: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
