use crate::*;
use anchor_spl::token;
use anchor_spl::token::{CloseAccount, Token, TokenAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Account<'info, TokenAccount>,
    #[account(mut, close = authority, address = stake.vault @ NosanaError::InvalidTokenAccount)]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        close = authority,
        has_one = authority @ NosanaError::Unauthorized,
        constraint = stake.time_unstake != 0 @ NosanaError::StakeNotUnstaked,
    )]
    pub stake: Account<'info, StakeAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Claim>) -> Result<()> {
    // get and check the stake
    let stake: &mut Account<StakeAccount> = &mut ctx.accounts.stake;
    require!(
        Clock::get()?.unix_timestamp > stake.time_unstake + i64::try_from(stake.duration).unwrap(),
        NosanaError::StakeLocked
    );

    // return tokens, the stake account is closed so no need to update it.
    utils::transfer_tokens_with_seeds(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.vault.amount,
        &[
            b"vault",
            id::NOS_TOKEN.as_ref(),
            stake.authority.key().as_ref(),
            &[stake.vault_bump],
        ],
    )?;

    // close the token vault
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.vault.to_account_info(),
            destination: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        &[&[
            b"vault",
            id::NOS_TOKEN.as_ref(),
            stake.authority.as_ref(),
            &[stake.vault_bump],
        ]],
    ))
}
