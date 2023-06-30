use anchor_lang::prelude::*;

/// Lending-Liquidating Automated Market Maker Algorithm (LLAMMA)
#[account]
#[derive(Default)]
pub struct Llamma {
    /// The mint of the stablecoin used for loans
    /// It will be used as the quote for all pairs
    pub stablecoin: Pubkey,
}

impl Llamma {
    pub const LEN: usize = 8 // Discriminator
        + 32; // Stablecoin
}
