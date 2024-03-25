use anchor_lang::error_code;

#[error_code]
pub enum FoodGatheringError {
    #[msg("FoodGathering: Not allowed owner")]
    NotAllowedOwner,

    #[msg("FoodGathering: Not allowed owner or minter")]
    NotAllowedOwnerOrMinter,

    #[msg("FoodGathering: insufficient SOL")]
    InsufficientBalance,

    #[msg("FoodGathering: you don't have enough ant coin balance for staking")]
    InsufficientTokenBalance,

    #[msg("FoodGathering: mint amount exceeds the maximum supply for this batch")]
    MaxStakingAmountAttained,

    #[msg("FoodGathering: You didn't stake any amount of ant coins")]
    NotStaked,
}
