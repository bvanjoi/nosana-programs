mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;
use nosana_common::*;
pub use state::*; // expose stake for cpi // expose stake for cpi

declare_id!(id::STAKING_PROGRAM);

#[program]
pub mod nosana_staking {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        init::handler(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64, duration: u128) -> Result<()> {
        stake::handler(ctx, amount, duration)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        unstake::handler(ctx)
    }

    pub fn restake(ctx: Context<Restake>) -> Result<()> {
        restake::handler(ctx)
    }

    pub fn topup(ctx: Context<Topup>, amount: u64) -> Result<()> {
        topup::handler(ctx, amount)
    }

    pub fn extend(ctx: Context<Extend>, duration: u64) -> Result<()> {
        extend::handler(ctx, duration)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        claim::handler(ctx)
    }

    pub fn slash(ctx: Context<Slash>, amount: u64) -> Result<()> {
        slash::handler(ctx, amount)
    }

    pub fn update_settings(ctx: Context<UpdateSettings>) -> Result<()> {
        update_settings::handler(ctx)
    }
}
