use anchor_lang::prelude::{Pubkey, *};

#[event]
pub struct FoodGatheringStaked {
    pub staker: Pubkey,
    pub antc_stake_amount: u64,
}

#[event]
pub struct FoodGatheringUnStaked {
    pub staker: Pubkey,
    pub antc_stake_amount: u64,
    pub reward_ant_food_amount: u64,
}
