use anchor_lang::prelude::*; // Import the Anchor prelude

declare_id!("Fg6PaFpoGXkYsidMpWxqSWY1hXkYsi1d1tQ1tQ1tQ1tQ"); // Declare the program ID

#[program]
pub mod safe_trading_zone { // Define the program module
    use super::*; // Import the program module

    // Define the initialize function. It takes a context and three public keys as arguments and returns a Result
    pub fn initialize(ctx: Context<Initialize>, seller: Pubkey, safe_zone: Pubkey, notary: Pubkey) -> Result<()> { 
        let escrow_account = &mut ctx.accounts.escrow_account;
        escrow_account.seller = seller; // Set the seller field of the escrow account
        escrow_account.safe_zone = safe_zone; // Set the safe_zone field of the escrow account
        escrow_account.notary = notary; // Set the notary field of the escrow account
        escrow_account.is_initialized = true; // Set the is_initialized field of the escrow account
        Ok(())
    }

    // Define the deposit function. It takes a context and an amount as arguments and returns a Result
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> { 
        let escrow_account = &mut ctx.accounts.escrow_account;
        let buyer = &ctx.accounts.buyer;

        // Transfer funds from buyer to escrow account
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &buyer.key(), 
            &escrow_account.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                buyer.to_account_info(),
                escrow_account.to_account_info(),
            ],
        )?;

        escrow_account.amount = amount;
        Ok(())
    }

    // Define the confirm function. It takes a context as argument and returns a Result
    pub fn confirm(ctx: Context<Confirm>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account; 
        let _buyer = &ctx.accounts.buyer; 
        let notary = &ctx.accounts.notary;

        // Ensure the notary is the expected one
        require!(notary.key() == escrow_account.notary, ErrorCode::InvalidNotary);

        // Split payment
        let seller_amount = (escrow_account.amount as f64 * 0.95) as u64;
        let safe_zone_amount = escrow_account.amount - seller_amount;

        // Transfer funds to seller
        let ix_seller = anchor_lang::solana_program::system_instruction::transfer(
            &escrow_account.key(),
            &escrow_account.seller,
            seller_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix_seller,
            &[
                escrow_account.to_account_info(),
                ctx.accounts.seller.to_account_info(),
            ],
        )?;

        // Transfer funds to safe zone. 
        let ix_safe_zone = anchor_lang::solana_program::system_instruction::transfer(
            &escrow_account.key(),
            &escrow_account.safe_zone,
            safe_zone_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix_safe_zone,
            &[
                escrow_account.to_account_info(),
                ctx.accounts.safe_zone.to_account_info(),
            ],
        )?;

        // Mark escrow as completed
        escrow_account.is_completed = true;
        Ok(())
    }
}

#[derive(Accounts)] // Define the Initialize struct

// Define the Initialize struct. It has an escrow_account, a user, and a system_program as fields and implements the Accounts trait. 
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 32 + 1)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Define the Deposit struct. It has an escrow_account, a buyer, and a system_program as fields and implements the Accounts trait. 
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Define the Confirm struct. It has an escrow_account, a buyer, a notary, a seller, a safe_zone, and a system_program as fields and implements the Accounts trait.
#[derive(Accounts)]
pub struct Confirm<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub notary: Signer<'info>,
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(mut)]
    pub safe_zone: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

// Define the EscrowAccount struct. It has a seller, a safe_zone, a notary, an amount, an is_initialized, and an is_completed as fields and implements the Account trait.
#[account]
pub struct EscrowAccount {
    pub seller: Pubkey,
    pub safe_zone: Pubkey,
    pub notary: Pubkey,
    pub amount: u64,
    pub is_initialized: bool,
    pub is_completed: bool,
}

// Define the ErrorCode enum. It has an InvalidNotary variant and implements the ErrorCode trait.
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid notary.")]
    InvalidNotary,
}
