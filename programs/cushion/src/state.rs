use anchor_lang::prelude::*;

/// Lending-Liquidating Automated Market Maker Algorithm (LLAMMA)
#[account]
#[derive(Default)]
pub struct Llamma {
    /// The admin that can create markets and receives fees
    pub admin: Pubkey,

    /// The mint of the token used for loans
    /// It will be used as the quote for all markets
    pub borrowed_mint: Pubkey,

    pub fee: u64,

    pub admin_fee: u64,
}

impl Llamma {
    pub const LEN: usize = 8 // Discriminator
        + 32 // Admin
        + 32; // Borrowed
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
