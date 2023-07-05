use anchor_lang::prelude::*;

#[constant]
pub const AUTHORITY_SEED: &str = "authority";

#[constant]
pub const STALENESS_THRESHOLD: u64 = 86400; // staleness threshold in seconds for the oracle

#[constant]
pub const MINIMUM_LIQUIDITY: u64 = 100; // The minimum amount of liquidity in a band
