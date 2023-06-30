use anchor_lang::prelude::*;
use pyth_sdk_solana::{load_price_feed_from_account_info, Price, PriceFeed};

declare_id!("HPHzFCqjZoCrn8CrnBAu8n2tcTgYCW71fWSXQGwe4XtY");

#[program]
pub mod cushion {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize()
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: verified by Pyth
    pub price_feed: AccountInfo<'info>,
}

impl Initialize<'_> {
    pub fn initialize(&self) -> Result<()> {
        const STALENESS_THRESHOLD: u64 = 60; // staleness threshold in seconds
        let price_account_info: AccountInfo = self.price_feed.to_account_info();
        let price_feed: PriceFeed = load_price_feed_from_account_info(&price_account_info).unwrap();
        let current_timestamp = Clock::get()?.unix_timestamp;
        let current_price: Price = price_feed
            .get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD)
            .unwrap();
        msg!(
            "price: ({} +- {}) x 10^{}",
            current_price.price,
            current_price.conf,
            current_price.expo
        );
        Ok(())
    }
}
