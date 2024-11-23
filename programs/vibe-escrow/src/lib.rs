use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("FWsgnQwgAJBmupaxcGwDmnBBGu2epZfkKhCDyQAC4wN9");

#[program]
mod vibe_escrow {
    use anchor_lang::solana_program::{program::invoke, system_instruction};
    
    use super::*;

    // Initialize the escrow
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        booking_id: String,
        verify_code: String,
        amount: u64
    ) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;

        escrow_account.client = *ctx.accounts.client.key;
        escrow_account.escort = *ctx.accounts.escort.key;
        escrow_account.verify_code = verify_code;
        escrow_account.booking_id = booking_id;

        let ix = system_instruction::transfer(
            &ctx.accounts.client.key(),
            escrow_account.to_account_info().key,
            amount,
        );
 
        invoke(
            &ix,
            &[
                ctx.accounts.client.clone(),
                escrow_account.to_account_info(),
            ],
        )?;

        Ok(())
    }

    // Release funds from escrow upon verification of the code
    pub fn release_escrow(ctx: Context<ReleaseEscrow>, verify_code: String) -> Result<()> {
        let escrow_account = &ctx.accounts.escrow_account;

        require!(escrow_account.escort == ctx.accounts.signer.key(), ErrorCode::InvalidSigner);
 
        // Verify the code matches
        require!(
            escrow_account.verify_code == verify_code,
            ErrorCode::InvalidVerificationCode
        );

        // Send the funds to the escort
        **ctx.accounts.signer.try_borrow_mut_lamports()? += **escrow_account.to_account_info().lamports.borrow();
        **escrow_account.to_account_info().lamports.borrow_mut() = 0;

        // Close the PDA
        Ok(())
    }
}

// Escrow Account
#[account]
#[derive(InitSpace)]
pub struct EscrowAccount {
    pub client: Pubkey,      // Client who created the escrow
    pub escort: Pubkey,      // Escort receiving funds
    #[max_len(32)]
    pub booking_id: String,  // Unique booking identifier
    #[max_len(6)]
    pub verify_code: String, // Verification code for releasing funds
}

// Contexts
#[derive(Accounts)]
#[instruction(booking_id: String)]
pub struct InitializeEscrow<'info> {
    #[account(
        init,
        payer = client,
        seeds = [b"escrow", booking_id.as_bytes(), escort.key.as_ref()],
        bump,
        space = 8 + EscrowAccount::INIT_SPACE
    )]
    pub escrow_account: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub client: AccountInfo<'info>,

    /// CHECK: This is just for public key verification
    pub escort: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(
        mut,
        close = signer,
        seeds = [b"escrow", escrow_account.booking_id.as_bytes(), signer.key.as_ref()],
        bump
    )]
    pub escrow_account: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided amount must be greater than zero.")]
    InvalidAmount,

    #[msg("Insufficient funds to perform the transfer.")]
    InsufficientFunds,

    #[msg("Signer does not have access to call this instruction.")]
    InvalidSigner,

    #[msg("The provided amount must be greater than zero.")]
    InvalidVerificationCode,
}