use anchor_lang::prelude::*;

/// Lending-Liquidating Automated Market Maker Algorithm (LLAMMA)
#[account]
#[derive(Default)]
pub struct Llamma {
    /// The admin that can create markets and receives fees
    pub admin: Pubkey,

    /// The mint of the token used for loans
    /// It will be used as the quote for all markets
    pub debt_mint: Pubkey,

    pub fee: u64,

    pub admin_fee: u64,
}

impl Llamma {
    pub const LEN: usize = 8 // Discriminator
        + 32 // Admin
        + 32 // Borrowed
        + 8 // Fee
        + 8; // Admin fee
}

#[account]
#[derive(Default)]
pub struct Market {
    pub llamma: Pubkey,
    pub collateral_mint: Pubkey,
    pub price_feed: Pubkey,
    pub amplification: u16,
    pub base_price: u64,
}

impl Market {
    pub const LEN: usize = 8 // Discriminator
        + 32 // LLAMMA
        + 32 // Collateral
        + 32 // Feed
        + 2 // Amplification
        + 8; // Base price
}

#[account]
#[derive(Default)]
pub struct Band {
    pub market: Pubkey,
    pub index: i16,
}

impl Band {
    pub const LEN: usize = 8 // Discriminator
        + 32 // Market
        + 2; // Index
}

#[account]
#[derive(Default)]
pub struct BandDeposit {
    pub band: Pubkey,
    pub depositor: Pubkey,
    pub deposited_amount: u64,
    pub borrowed_amount: u64,
}

impl BandDeposit {
    pub const LEN: usize = 8 // Discriminator
        + 32 // Market
        + 32 // Depositor
        + 8 // Deposit
        + 8; // Debt
}
