use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constants::*, error::*, states::*};

#[derive(Accounts)]
pub struct InitBlackList<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [BLACKLIST_SEED],
        bump,
        space = 8 + size_of::<Pubkey>() * 300
    )]
    pub blacklist: Box<Account<'info, Blacklist>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitBlackList>) -> Result<()> {
    Ok(())
}
