use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey, // 32 bytes
    pub name: String, // 2 bytes
    pub fee: u16,
    pub bump: u8,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 32 + 8 + 2 + 2 + 1 + 1 + 1;
}
