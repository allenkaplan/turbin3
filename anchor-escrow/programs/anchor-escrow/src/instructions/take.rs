use anchor_lang::{accounts::signer, prelude::*};
use anchor_spl::{
    self,
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub maker: SystemAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,


    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn completeEscrow(&mut self, deposit: u64)-> Result<()> {

        // First transfer: Maker sends tokens to escrow
        let cpi_accounts_1: TransferChecked<'_> = TransferChecked {
            from: self.taker_ata_b.to_account_info(), 
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_b.to_account_info(),
        };

        let cpi_ctx_1: CpiContext<'_, '_, '_, '_, TransferChecked<'_>> = CpiContext::new(self.token_program.to_account_info(), cpi_accounts_1);
        transfer_checked(cpi_ctx_1, deposit, self.mint_b.decimals)?;

        // Second transfer: Vault sends tokens to taker
        let cpi_accounts_2: TransferChecked<'_> = TransferChecked {
            from: self.vault.to_account_info(), 
            to: self.taker_ata_a.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };
        let seeds = &[
            b"escrow",
            self.escrow.to_account_info().key.as_ref(),
            &[self.escrow.bump]
        ];
        let signer = &[&seeds[..]];
        
        let cpi_ctx_2: CpiContext<'_, '_, '_, '_, TransferChecked<'_>> = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts_2, signer);
        transfer_checked(cpi_ctx_2, self.escrow.receive, self.mint_a.decimals)?;
        Ok(())
    }
 }

