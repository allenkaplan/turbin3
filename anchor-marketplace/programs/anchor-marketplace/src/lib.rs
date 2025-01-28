use anchor_lang::prelude::*;

declare_id!("Gg9pSeNvuFiGsw6QqEfhZjn5N7UWAGTSGmRKgaVpzbc1");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
