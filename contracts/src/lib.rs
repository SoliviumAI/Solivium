use anchor_lang::prelude::*;

declare_id!("YourProgramIdHere");

#[program]
pub mod solivium {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, wallet_address: Pubkey) -> Result<()> {
        let account = &mut ctx.accounts.data_account;
        account.wallet_address = wallet_address;
        account.balance = 0;
        Ok(())
    }

    pub fn update_balance(ctx: Context<UpdateBalance>, new_balance: u64) -> Result<()> {
        let account = &mut ctx.accounts.data_account;
        account.balance = new_balance;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub data_account: Account<'info, WalletData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBalance<'info> {
    #[account(mut)]
    pub data_account: Account<'info, WalletData>,
}

#[account]
pub struct WalletData {
    pub wallet_address: Pubkey,
    pub balance: u64,
}
