use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constants::*, error::*, states::*};

#[derive(Accounts)]
pub struct RemoveFromBlacklist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(has_one = admin)]
    pub settings: Box<Account<'info, Settings>>,

    #[account(
        mut,
        seeds = [BLACKLIST_SEED],
        bump
    )]
    pub blacklist: Box<Account<'info, Blacklist>>
}

pub fn handler(ctx: Context<RemoveFromBlacklist>, 
    address_to_remove: Pubkey,
) -> Result<()> {
    let accts = ctx.accounts;
    if accts.blacklist.addresses.contains(&address_to_remove) {
        let idx = accts.blacklist.addresses.iter().position(|x| *x == address_to_remove).unwrap();
        accts.blacklist.addresses.remove(idx);
    }
    Ok(())
}
