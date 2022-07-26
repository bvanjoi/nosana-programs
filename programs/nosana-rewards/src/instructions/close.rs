use crate::*;

use nosana_staking::program::NosanaStaking;
use nosana_common::{NosanaError};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, seeds = [ b"stats" ], bump = stats.bump)]
    pub stats: Account<'info, StatsAccount>,
    #[account(mut, close = authority, seeds = [ b"reward", staker.key().as_ref()], bump = reward.bump)]
    pub reward: Box<Account<'info, RewardAccount>>,
    #[account(owner = staking_program.key(), constraint = staker.key() == stake.authority)]
    pub stake: Account<'info, StakeAccount>,
    pub staker: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub staking_program: Program<'info, NosanaStaking>,
}

pub fn handler(ctx: Context<Close>) -> Result<()> {
    let stats = &mut ctx.accounts.stats;
    let stake = &ctx.accounts.stake;
    let reward = &mut ctx.accounts.reward;

    // if the stake is not unstaked yet, only the owner can close the reward
    if stake.time_unstake == 0_i64 {
        require!(reward.authority == *ctx.accounts.authority.key, NosanaError::Unauthorized);
    }

    stats.r_total = stats.r_total.checked_sub(reward.r_owned).unwrap();
    stats.t_total = stats.t_total.checked_sub(reward.t_owned).unwrap();
    stats.update_rate();

    Ok(())
}
