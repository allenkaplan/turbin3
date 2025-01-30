use anchor_lang::prelude::*;

mod instructions;
mod state;
mod errors;

use instructions::*;

declare_id!("EpHvvodohEw417bp5VDKrmLSDL7HAMPfT42q2FtzdSN2");

#[program]
pub mod amm_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.initialize(seed, fee, authority, &ctx.bumps);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y);
        Ok(())
    }
}