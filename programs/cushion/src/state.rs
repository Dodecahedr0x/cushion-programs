use anchor_lang::prelude::*;

/// Lending-Liquidating Automated Market Maker Algorithm (LLAMMA)
#[account]
#[derive(Default)]
pub struct Llamma {
    /// The admin that can create markets and receives fees
    pub admin: Pubkey,

    /// The mint of the stablecoin used for loans
    /// It will be used as the quote for all pairs
    pub stablecoin: Pubkey,
}

impl Llamma {
    pub const LEN: usize = 8 // Discriminator
        + 32 // Admin
        + 32; // Stablecoin
}

#[account]
#[derive(Default)]
pub struct Market {
    pub llamma: Pubkey,
    pub collateral: Pubkey,
    pub price_feed: Pubkey,
}

impl Market {
    pub const LEN: usize = 8 // Discriminator
        + 32 // LLAMMA
        + 32 // Collateral
        + 32; // Feed
}
