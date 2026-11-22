
use anchor_lang::prelude::*;

pub mod provider;
pub mod customer;
pub mod booking;
pub mod escrow;

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
    pub authority: Pubkey,
}
