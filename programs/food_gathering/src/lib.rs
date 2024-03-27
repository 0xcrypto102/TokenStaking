use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod constants;
pub mod utils;
use instructions::*;

declare_id!("EudLu51y9mdaTC5tWdGpptLocsfQimz3ZCyvcfA7DbRa");

#[program]
pub mod food_gathering {
    use super::*;

    // user function

    pub fn stake(ctx: Context<Stake>, antc_amount: u64) -> Result<()> {
        instructions::stake(ctx, antc_amount)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::unstake(ctx)
    }

    // owner or minter function

    pub fn initialize(ctx: Context<Initialize>, new_owner: Pubkey, antc_price: u64, antc_expo: u64) -> Result<()> {
        instructions::initialize(ctx, new_owner, antc_price, antc_expo)
    }

    pub fn deposit_ant_food_token(ctx: Context<DepositAntFoodToken>, amount: u64) -> Result<()> {
        instructions::deposit_ant_food_token(ctx, amount)
    }

    pub fn set_stake_fee_amount(ctx: Context<SetStakeFeeAmount>, stake_fee_amount: u64) -> Result<()> {
        instructions::set_stake_fee_amount(ctx, stake_fee_amount)
    }

    pub fn set_max_amount_for_stake(ctx: Context<SetMaxAmountForStake>, max_amount_for_stake: u64) -> Result<()> {
        instructions::set_max_amount_for_stake(ctx, max_amount_for_stake)
    }

    pub fn set_cycle_staked_amount(ctx: Context<SetCycleStakedAmount>, cycle_staked_amount: u64) -> Result<()> {
        instructions::set_cycle_staked_amount(ctx, cycle_staked_amount)
    }

    pub fn set_cycle_timestamp(ctx: Context<SetCycleTimestamp>, cycle_timestamp: u32) -> Result<()> {
        instructions::set_cycle_timestamp(ctx, cycle_timestamp)
    }

    pub fn set_ant_food_token(ctx: Context<SetAntFoodToken>) -> Result<()> {
        instructions::set_ant_food_token(ctx)
    }

    pub fn set_ant_coin(ctx: Context<SetAntCoin>) -> Result<()> {
        instructions::set_ant_coin(ctx)
    }

    // owner function

    pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
        instructions::set_paused(ctx, paused)
    }

    pub fn set_minter_role(ctx: Context<SetMinterRole>, is_minter: bool) -> Result<()> {
        instructions::set_minter_role(ctx, is_minter)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSOL>, amount: u64) -> Result<()> {
        instructions::withdraw_sol(ctx, amount)
    }

    pub fn withdraw_token(ctx: Context<WithdrawToken>, amount: u64) -> Result<()> {
        instructions::withdraw_token(ctx, amount)
    }

    // get function

    pub fn get_pending_reward(ctx: Context<GetPendingReward>) -> Result<u64> {
        instructions::get_pending_reward(ctx)
    }
}