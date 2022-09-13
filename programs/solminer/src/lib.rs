use anchor_lang::prelude::*;

declare_id!("G9b8S6XsFDZchBXHFXAGpmssnFPRRiA92XEd3jv4jY12");

/// constant
pub mod constants;
/// error
pub mod error;
/// instructions
pub mod instructions;
/// states
pub mod states;
/// utils
pub mod utils;

use crate::instructions::*;

#[program]
pub mod solminer {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        roi: u64, 
        fee: u64, 
        ref_fee: u64,
        withdraw_tax: u64,
        compound_fee: u64
    ) -> Result<()> {
        initialize::handler(ctx, roi, fee, ref_fee, withdraw_tax, compound_fee)
    }

    pub fn add_blacklist(ctx: Context<AddBlacklist>, addr: Pubkey) -> Result<()> {
        add_blacklist::handler(ctx, addr)
    }

    pub fn init_blacklist(ctx: Context<InitBlackList>) -> Result<()> {
      init_blacklist::handler(ctx)
    }

    pub fn remove_from_blacklist(ctx: Context<RemoveFromBlacklist>, addr: Pubkey) -> Result<()> {
        remove_from_blacklist::handler(ctx, addr)
    }
  
    pub fn set_pool_prize(ctx: Context<SetPoolPrize>, mins: u64, ratio: u64) -> Result<()> {
      set_pool_prize::handler(ctx, mins, ratio)
    }

    pub fn start_miner(ctx: Context<StartMiner>) -> Result<()> {
      start_miner::handler(ctx)
    }

    pub fn compound(ctx: Context<Compound>) -> Result<()> {
        compound::handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, seed_key: Pubkey) -> Result<()> {
        deposit::handler(ctx, amount, seed_key)
    }

    pub fn init_user_state(ctx: Context<InitUserState>, referrer: Pubkey) -> Result<()> {
        init_user_state::handler(ctx, referrer)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        unstake::handler(ctx)
    }
}
