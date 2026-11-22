
use anchor_lang::prelude::*;

use crate::errors::ServiceHubError;
use crate::state::customer::Customer;

#[derive(Accounts)]
pub struct RegisterCustomer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Customer::INIT_SPACE,
        seeds = [
            b"customer",
            authority.key().as_ref()
        ],
        bump
    )]
    pub customer: Account<'info, Customer>,

    pub system_program: Program<'info, System>,
}

pub fn register_customer_handler(
    ctx: Context<RegisterCustomer>,
    name: String,
) -> Result<()> {
    require!(
        !name.trim().is_empty(),
        ServiceHubError::CustomerNameEmpty
    );

    require!(
        name.len() <= 50,
        ServiceHubError::CustomerNameTooLong
    );

    let customer = &mut ctx.accounts.customer;

    customer.authority = ctx.accounts.authority.key();
    customer.name = name;
    customer.registered = true;
    customer.bump = ctx.bumps.customer;

    Ok(())
}
