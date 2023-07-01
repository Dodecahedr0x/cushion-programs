use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::*;

declare_id!("2rYUpCQGEqxz8WesY44kkXm1JvtWairzVJLjhf2UpynL");

#[program]
pub mod cushion {
    use super::*;

    pub fn initialize_llamma(ctx: Context<InitializeLlamma>) -> Result<()> {
        instructions::initialize_llamma(ctx)
    }

    pub fn create_market(ctx: Context<CreateMarket>, base_price: u64) -> Result<()> {
        instructions::create_market(ctx, base_price)
    }

    pub fn create_band(ctx: Context<CreateBand>, index: u16) -> Result<()> {
        instructions::create_band(ctx, index)
    }
}
