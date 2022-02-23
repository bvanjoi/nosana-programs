use crate::*;

use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitVault<'info> {

    pub authority: Signer<'info>,

    #[account(address = constants::TOKEN_PUBLIC_KEY.parse::<Pubkey>().unwrap())]
    pub nos: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = authority,
        token::mint = nos,
        token::authority = vault,
        seeds = [ constants::TOKEN_PUBLIC_KEY.parse::<Pubkey>().unwrap().as_ref() ],
        bump = bump,
    )]
    /// the not-yet-Created, derived token vault public key
    pub vault: Box<Account<'info, TokenAccount>>,

    /// required
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler() -> ProgramResult {
    Ok(())
}
