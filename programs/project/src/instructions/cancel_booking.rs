
use anchor_lang::prelude::*;

use crate::errors::ServiceHubError;
use crate::state::customer::Customer;
use crate::state::booking::{
    Booking,
    BOOKING_CREATED,
    BOOKING_CANCELLED,
};
use crate::state::escrow::Escrow;

#[derive(Accounts)]
#[instruction(booking_id: u64)]
pub struct CancelBooking<'info> {
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

    #[account(
        mut,
        seeds = [
            b"booking",
            customer_authority.key().as_ref(),
            booking_id.to_le_bytes().as_ref()
        ],
        bump = booking.bump,
        constraint = booking.customer == customer_authority.key(),
        constraint = booking.booking_id == booking_id
    )]
    pub booking: Account<'info, Booking>,

    #[account(
        mut,
        seeds = [
            b"escrow",
            booking.key().as_ref()
        ],
        bump = escrow.bump,
        constraint = escrow.booking == booking.key(),
        constraint = escrow.customer == customer_authority.key(),
        constraint = escrow.provider == booking.provider,
        constraint = escrow.amount_lamports == booking.amount_lamports,
        close = customer_authority
    )]
    pub escrow: Account<'info, Escrow>,
}

pub fn cancel_booking_handler(
    ctx: Context<CancelBooking>,
    _booking_id: u64,
) -> Result<()> {
    require!(
        ctx.accounts.customer.registered,
        ServiceHubError::CustomerNotRegistered
    );

    require!(
        ctx.accounts.booking.status == BOOKING_CREATED,
        ServiceHubError::InvalidCancellationStatus
    );

    let amount = ctx.accounts.escrow.amount_lamports;

    ctx.accounts.escrow.sub_lamports(amount)?;
    ctx.accounts.customer_authority.add_lamports(amount)?;

    ctx.accounts.booking.status = BOOKING_CANCELLED;

    Ok(())
}
