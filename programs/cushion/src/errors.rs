use anchor_lang::prelude::*;

#[error_code]
pub enum CushionError {
    #[msg("Not the llamma admin")]
    NotAdmin,

    #[msg("The oracle price is too old")]
    OutdatedPrice,

    #[msg("The provided price feed is invalid")]
    InvalidPriceFeed,

    #[msg("Parsing an invalid number")]
    NumberParsingFailed,
}
