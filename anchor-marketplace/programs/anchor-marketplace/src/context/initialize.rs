use anchor_lang::prelude::*;

use crate::state::marketplace::Marketplace;
use crate::errors::ErrorCode;
#[derive(Accounts)]
#[instruction(name: String, fee: u16)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = Marketplace::INIT_SPACE,
        seeds = [b"marketplace",name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub treasury: SystemAccount<'info>,

    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(mut self, ctx: Context<Initialize>, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {

        require!(name.len() > 0 && name.len() < 4, ErrorCode::InvalidNameLength);
        require!(fee >= 0 && fee <= 100, ErrorCode::InvalidFee);

        ctx.accounts.marketplace.admin = ctx.accounts.admin.key();
        ctx.accounts.marketplace.name = name;
        ctx.accounts.marketplace.fee = fee;
        ctx.accounts.marketplace.bump = bumps.marketplace;
        ctx.accounts.marketplace.treasury_bump = bumps.treasury;
        ctx.accounts.marketplace.rewards_bump = bumps.rewards;
        Ok(())
    }
}