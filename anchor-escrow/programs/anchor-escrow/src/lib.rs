use anchor_lang::prelude::*;

mod state;
mod instructions;

declare_id!("6VjjxCjwgPzr1PhkadfvM6UHLjg4r2TrpdgXuo3S6rfP");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
