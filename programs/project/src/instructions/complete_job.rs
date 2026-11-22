
use anchor_lang::prelude::*;

use crate::errors::ServiceHubError;
use crate::state::provider::Provider;
use crate::state::booking::{
    Booking,
    BOOKING_ACCEPTED,
    BOOKING_COMPLETED,
};

#[derive(Accounts)]
#[instruction(booking_id: u64)]
pub struct CompleteJob<'info> {
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

pub fn complete_job_handler(
    ctx: Context<CompleteJob>,
    _booking_id: u64,
) -> Result<()> {
    require!(
        ctx.accounts.provider.registered,
        ServiceHubError::ProviderNotRegistered
    );

    require!(
        ctx.accounts.booking.status == BOOKING_ACCEPTED,
        ServiceHubError::InvalidCompletionStatus
    );

    ctx.accounts.booking.status = BOOKING_COMPLETED;

    Ok(())
}
