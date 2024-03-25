use anchor_lang::prelude::*;
use crate::{state::*};

pub fn _get_pending_reward(global_state: &GlobalState, staked_info: &StakedInfo) -> Result<u64> {
    let now = Clock::get()?.unix_timestamp;
    let staked_period = now.checked_sub(staked_info.staked_timestamp).unwrap() as u64;
    let pending_reward = staked_period
        .checked_mul(staked_info.staked_amount).unwrap()
        .checked_mul(global_state.precision as u64).unwrap()
        .checked_div(
            (global_state.cycle_timestamp as u64) * global_state.cycle_staked_amount
        ).unwrap()
        .checked_add(staked_info.reward_debt)
        .unwrap();
    
    Ok(pending_reward)
}