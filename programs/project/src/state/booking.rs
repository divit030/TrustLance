
use anchor_lang::prelude::*;


pub const BOOKING_CREATED: u8 = 0;
pub const BOOKING_ACCEPTED: u8 = 1;
pub const BOOKING_COMPLETED: u8 = 2;
pub const BOOKING_APPROVED: u8 = 3;
pub const BOOKING_CANCELLED: u8 = 4;

#[account]
#[derive(InitSpace)]
pub struct Booking {

  
    pub booking_id: u64,

    pub customer: Pubkey,

    pub provider: Pubkey,

    #[max_len(50)]
    pub service: String,

    pub amount_lamports: u64,

    pub status: u8,

    pub bump: u8,
}
