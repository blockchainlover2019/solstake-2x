use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};

#[derive(Accounts)]
pub struct Compound<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      seeds = [SETTINGS_SEED, settings.admin.as_ref()],
      bump = settings.bump,
      has_one = pool,
      has_one = marketing_wallet
    )]
    pub settings: Box<Account<'info, Settings>>,

    #[account(
        mut,
        seeds = [DATA_SEED, user.key().as_ref(), invest_data.seed_key.as_ref()],
        bump = invest_data.bump,
    )]
    pub invest_data: Account<'info, InvestData>,

    #[account(mut)]
    /// CHECK: pool account is pda for storing sols
    pub pool: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: checked in settings
    pub marketing_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Compound<'info> {
    fn validate(&self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        let new_roi_time = self
            .invest_data
            .last_roi_time
            //.checked_add(DAY_IN_SEC)
            .checked_add(60 * 1)
            .unwrap();
        require!(new_roi_time <= current_time, CustomError::CantClaimNow);
        require!(self.invest_data.days < WITHDRAW_LIMIT, CustomError::CantCompound);
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handler(ctx: Context<Compound>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp as u64;

    let accts = ctx.accounts;
    let new_active_balance = accts
        .invest_data
        .active_balance
        .checked_mul(accts.settings.roi)
        .unwrap()
        .checked_div(FEE_DIVIDER)
        .unwrap();
    
    let daily_reward = new_active_balance.checked_sub(accts.invest_data.active_balance).unwrap();

    let compound_fee = daily_reward
        .checked_mul(accts.settings.compound_fee)
        .unwrap()
        .checked_div(FEE_DIVIDER)
        .unwrap();

    accts.invest_data.last_roi_time = current_time;

    accts.invest_data.active_balance = new_active_balance
        .checked_sub(compound_fee)
        .unwrap();
    accts.invest_data.days += 1;

    // send compound_fee to marketing wallet
    let signer_seeds: &[&[&[u8]]] = &[&[POOL_SEED.as_ref(), &[accts.settings.pool_bump]]];
    invoke_signed(
        &system_instruction::transfer(
            &accts.pool.key(),
            &accts.marketing_wallet.key(),
            compound_fee,
        ),
        &[
            accts.pool.to_account_info(),
            accts.marketing_wallet.to_account_info(),
            accts.system_program.to_account_info(),
        ],
        signer_seeds,
    )?;

    Ok(())
}
