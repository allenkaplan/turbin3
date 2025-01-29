use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::
    {Metadata, MetadataAccount, MasterEditionAccount, mpl_token_metadata::instructions::{FreezeDelegatedAccount; Cpi, UnfreezeDelegatedAccountCpi}}, 
    token::
    {Mint, TokenAccount, Approve}
};

use crate::state::{StakeAccount, StakeConfig, UserAccount};
use crate::errors::StakeError;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"mint"],
        bump,
    )]
    pub mint: Account<'info, Mint>,

    #[account()]
    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user.key(),
    )]
    pub mint_ata: AccountInfo<'info, TokenAccount>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key().as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.mint.key().as_ref().verified == true
    )]
    pub metadata: AccountInfo<'info, Metadata>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref(), b"edition"],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    pub metadata_program: Program<'info, MetadataAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {

        require!(self.user_account.amount_staked < self.config.max_stake, StakeError::MaxStakeReached);

        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: bumps.stake_account,
        });

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };


        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        Approve::approve(cpi_ctx, 1)?;

        let seeds = [b"stake", self.mint.key().as_ref(), self.config.key().as_ref(), &[self.stake_account.bump]];

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info(); 
        let metadata_program = &self.metadata_program.to_account_info();



        Ok(())
    }
}