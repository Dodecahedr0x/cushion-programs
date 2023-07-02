use anchor_lang::prelude::*;

#[constant]
pub const AUTHORITY_SEED: &str = "authority";

#[constant]
pub const STALENESS_THRESHOLD: u64 = 86400; // staleness threshold in seconds for the oracle
