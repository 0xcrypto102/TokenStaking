use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub is_initialized: u8,
    pub is_paused: bool,
    pub owner: Pubkey,
    pub vault: Pubkey,  // keep SOL from minters
    pub ant_coin: Pubkey,
    pub precision: u32,
    pub ant_food_token: Pubkey,
    pub stake_fee_amount: u64,
    pub max_amount_for_stake: u64,
    pub cycle_staked_amount: u64,
    pub cycle_timestamp: u32,
    pub antc_price: u64,
    pub antc_expo: u64,
}

#[account]
#[derive(Default)]
pub struct Minter {
    pub minter_key: Pubkey,
    pub is_minter: bool,
}

#[account]
#[derive(Default)]
pub struct StakedInfo {
    pub staked_amount: u64,
    pub staked_timestamp: i64,
    pub reward_debt: u64,
}