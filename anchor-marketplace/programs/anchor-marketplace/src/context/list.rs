use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenInterface};

use crate::state::marketplace::Marketplace;

pub struct List<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub marketplace: Account<'info, Marketplace>,


    pub maker_mint: InterfaceAccount<'info, Mint>,
    pub maker_mint_ata: InterfaceAccount<'info, TokenAccount>,
}