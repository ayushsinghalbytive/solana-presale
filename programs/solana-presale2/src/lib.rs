use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[program]
pub mod solana_presale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Program initialized!");
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_data: u64) -> Result<()> {
        let account = &mut ctx.accounts.my_account;
        account.data = new_data;
        msg!("Account data updated!");
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.presale_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;

        msg!("Tokens bought successfully!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub presale_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
