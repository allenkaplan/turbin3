use anchor_lang::prelude::*;

declare_id!("Gg9pSeNvuFiGsw6QqEfhZjn5N7UWAGTSGmRKgaVpzbc1");

mod context;
mod state;
mod errors;

use context::*;
use errors::*;

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(mut ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps);
        Ok(())
    }

    pub fn listing(ctx: Context<CreateListing>) -> Result<()> {
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
