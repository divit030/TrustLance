
use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::register_provider::*;
pub use instructions::register_customer::*;
pub use instructions::create_booking::*;
pub use instructions::accept_booking::*;
pub use instructions::complete_job::*;
pub use instructions::approve_job::*;
pub use instructions::cancel_booking::*;

pub use state::Counter;

declare_id!("JAuKfWhmk78tNTNwiZLuKgNCHUZ9hYNQm5BiFouES3re");

#[program]
pub mod project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        counter.count = 0;
        counter.authority = ctx.accounts.user.key();

        Ok(())
    }

    pub fn register_provider(
        ctx: Context<RegisterProvider>,
        name: String,
        service: String,
        price_lamports: u64,
    ) -> Result<()> {
        instructions::register_provider::register_provider_handler(
            ctx,
            name,
            service,
            price_lamports,
        )
    }

    pub fn register_customer(
        ctx: Context<RegisterCustomer>,
        name: String,
    ) -> Result<()> {
        instructions::register_customer::register_customer_handler(
            ctx,
            name,
        )
    }

    pub fn create_booking(
        ctx: Context<CreateBooking>,
        booking_id: u64,
    ) -> Result<()> {
        instructions::create_booking::create_booking_handler(
            ctx,
            booking_id,
        )
    }

    pub fn accept_booking(
        ctx: Context<AcceptBooking>,
        booking_id: u64,
    ) -> Result<()> {
        instructions::accept_booking::accept_booking_handler(
            ctx,
            booking_id,
        )
    }

    pub fn complete_job(
        ctx: Context<CompleteJob>,
        booking_id: u64,
    ) -> Result<()> {
        instructions::complete_job::complete_job_handler(
            ctx,
            booking_id,
        )
    }

    pub fn approve_job(
        ctx: Context<ApproveJob>,
        booking_id: u64,
    ) -> Result<()> {
        instructions::approve_job::approve_job_handler(
            ctx,
            booking_id,
        )
    }

    pub fn cancel_booking(
    ctx: Context<CancelBooking>,
    booking_id: u64,
) -> Result<()> {
    instructions::cancel_booking::cancel_booking_handler(
        ctx,
        booking_id,
    )
}

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
