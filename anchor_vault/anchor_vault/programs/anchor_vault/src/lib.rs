use anchor_lang::prelude::*;

declare_id!("EmuD41BSjs92FrJQ86xgtAbgK9tWiyCpE4NAvEksETGp");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
        Ok(())
    }

    pub fn payment(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.payment(amount)
        Ok(())
    }   
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, 
        payer = user, 
        space = 8 + VaultState::INIT_SPACE, 
        seeds = [b"state", user.key().as_ref()], 
        bump
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, buumps: &InitializeBumps) -> Result<()> {

        self.state.vault_bump = buumps.vault_bump;
        self.state.state_bump = buumps.state_bump;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump
    )]
    pub state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payment<'info> {
    pub user: Signer<'info>
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"vault", user.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault_state: Account<'info, VaultState>
    pub system_program: Program<'info, System>
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx: CpiContext< = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ct, lamports: amount)?;

        self.state.balance += amount;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let vault_state_key = self.vault_state.key();

        let seeds = &[b"state", vault_state_key.as_ref()], &[self.vault_state.vault_bump];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx: CpiContext.new_with_signer(cpi_program, cpi_accounts, signer_seeds)

        transfer(cpi_ctx, lamports: amount);

        self.state.balance -= amount;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
    pub balance: u64,
}
