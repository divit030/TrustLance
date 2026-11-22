use anchor_lang::prelude::*;

use crate::errors::ServiceHubError;
use crate::state::customer::Customer;
use crate::state::booking::{
    Booking,
    BOOKING_COMPLETED,
    BOOKING_APPROVED,
};
use crate::state::escrow::Escrow;

#[derive(Accounts)]
#[instruction(booking_id: u64)]
pub struct ApproveJob<'info> {
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

    #[account(mut)]
    pub provider_authority: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"booking",
            customer_authority.key().as_ref(),
            booking_id.to_le_bytes().as_ref()
        ],
        bump = booking.bump,
        constraint = booking.customer == customer_authority.key(),
        constraint = booking.provider == provider_authority.key(),
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
        constraint = escrow.provider == provider_authority.key(),
        constraint = escrow.amount_lamports == booking.amount_lamports,
        close = customer_authority
    )]
    pub escrow: Account<'info, Escrow>,
}

pub fn approve_job_handler(
    ctx: Context<ApproveJob>,
    _booking_id: u64,
) -> Result<()> {
    require!(
        ctx.accounts.customer.registered,
        ServiceHubError::CustomerNotRegistered
    );

    require!(
        ctx.accounts.booking.status == BOOKING_COMPLETED,
        ServiceHubError::InvalidApprovalStatus
    );

    let amount = ctx.accounts.escrow.amount_lamports;

    let escrow_balance = ctx.accounts.escrow.to_account_info().lamports();

    let rent_reserve = Rent::get()?.minimum_balance(
        8 + Escrow::INIT_SPACE
    );

    let available_balance = escrow_balance
        .checked_sub(rent_reserve)
        .ok_or(ServiceHubError::InsufficientEscrowFunds)?;

    require!(
        available_balance >= amount,
        ServiceHubError::InsufficientEscrowFunds
    );

    ctx.accounts.escrow.sub_lamports(amount)?;
    ctx.accounts.provider_authority.add_lamports(amount)?;

    ctx.accounts.booking.status = BOOKING_APPROVED;

    Ok(())
}