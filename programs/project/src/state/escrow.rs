
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub booking: Pubkey,
    pub customer: Pubkey,
    pub provider: Pubkey,
    pub amount_lamports: u64,
    pub bump: u8,
}
