use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub seed: u64,
    pub fee: u16,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub locked: bool,
    pub config_bump: u8,
    pub lp_bump: u8,
}

impl Config {
    pub const INIT_SPACE: usize = 8 + 32 + 8 + 2 + 32 + 32 + 1 + 1;
}
