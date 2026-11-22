
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke,
    system_instruction,
};

use crate::errors::ServiceHubError;
use crate::state::customer::Customer;
use crate::state::provider::Provider;
use crate::state::booking::{
    Booking,
    BOOKING_CREATED,
};
use crate::state::escrow::Escrow;

#[derive(Accounts)]
#[instruction(booking_id: u64)]
pub struct CreateBooking<'info> {
    #[account(mut)]
    pub customer_authority: Signer<'info>,

    #[account(
        seeds = [
            b"customer",
            customer_authority.key().as_ref()
        ],
        bump = customer.bump,
        constraint = customer.authority == customer_authority.key()
    )]
    pub customer: Account<'info, Customer>,

    pub provider_authority: SystemAccount<'info>,

    #[account(
        seeds = [
            b"provider",
            provider_authority.key().as_ref()
        ],
        bump = provider.bump,
        constraint = provider.authority == provider_authority.key()
    )]
    pub provider: Account<'info, Provider>,

    #[account(
        init,
        payer = customer_authority,
        space = 8 + Booking::INIT_SPACE,
        seeds = [
            b"booking",
            customer_authority.key().as_ref(),
            booking_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub booking: Account<'info, Booking>,

    #[account(
        init,
        payer = customer_authority,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [
            b"escrow",
            booking.key().as_ref()
        ],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}

pub fn create_booking_handler(
    ctx: Context<CreateBooking>,
    booking_id: u64,
) -> Result<()> {
    require!(
        ctx.accounts.customer.registered,
        ServiceHubError::CustomerNotRegistered
    );

    require!(
        ctx.accounts.provider.registered,
        ServiceHubError::ProviderNotRegistered
    );

    require!(
        ctx.accounts.provider.price_lamports > 0,
        ServiceHubError::InvalidPrice
    );

    let amount = ctx.accounts.provider.price_lamports;

    let transfer_instruction = system_instruction::transfer(
        &ctx.accounts.customer_authority.key(),
        &ctx.accounts.escrow.key(),
        amount,
    );

    invoke(
        &transfer_instruction,
        &[
            ctx.accounts.customer_authority.to_account_info(),
            ctx.accounts.escrow.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let booking = &mut ctx.accounts.booking;

    booking.customer = ctx.accounts.customer_authority.key();
    booking.provider = ctx.accounts.provider_authority.key();
    booking.booking_id = booking_id;
    booking.service = ctx.accounts.provider.service.clone();
    booking.amount_lamports = amount;
    booking.status = BOOKING_CREATED;
    booking.bump = ctx.bumps.booking;

    let escrow = &mut ctx.accounts.escrow;

    escrow.booking = ctx.accounts.booking.key();
    escrow.customer = ctx.accounts.customer_authority.key();
    escrow.provider = ctx.accounts.provider_authority.key();
    escrow.amount_lamports = amount;
    escrow.bump = ctx.bumps.escrow;

    Ok(())
}
