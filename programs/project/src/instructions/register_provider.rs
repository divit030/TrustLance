use anchor_lang::prelude::*;
use crate::state::provider::Provider;

#[derive(Accounts)]
pub struct RegisterProvider<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Provider::INIT_SPACE,
        seeds = [b"provider", authority.key().as_ref()],
        bump
    )]
    pub provider: Account<'info, Provider>,

    pub system_program: Program<'info, System>,
}

pub fn register_provider_handler(
    ctx: Context<RegisterProvider>,
    name: String,
    service: String,
    price_lamports: u64,
) -> Result<()> {
    let provider = &mut ctx.accounts.provider;

    provider.authority = ctx.accounts.authority.key();
    provider.name = name;
    provider.service = service;
    provider.price_lamports = price_lamports;
    provider.registered = true;
    provider.bump = ctx.bumps.provider;

    Ok(())
}