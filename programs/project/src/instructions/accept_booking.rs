
use anchor_lang::prelude::*;

use crate::errors::ServiceHubError;
use crate::state::provider::Provider;
use crate::state::booking::{
    Booking,
    BOOKING_CREATED,
    BOOKING_ACCEPTED,
};

#[derive(Accounts)]
#[instruction(booking_id: u64)]
pub struct AcceptBooking<'info> {
    pub provider_authority: Signer<'info>,

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
        mut,
        seeds = [
            b"booking",
            booking.customer.as_ref(),
            booking_id.to_le_bytes().as_ref()
        ],
        bump = booking.bump,
        constraint = booking.booking_id == booking_id,
        constraint = booking.provider == provider_authority.key()
    )]
    pub booking: Account<'info, Booking>,
}

pub fn accept_booking_handler(
    ctx: Context<AcceptBooking>,
    _booking_id: u64,
) -> Result<()> {
    require!(
        ctx.accounts.provider.registered,
        ServiceHubError::ProviderNotRegistered
    );

    require!(
        ctx.accounts.booking.status == BOOKING_CREATED,
        ServiceHubError::InvalidBookingStatus
    );

    ctx.accounts.booking.status = BOOKING_ACCEPTED;

    Ok(())
}
