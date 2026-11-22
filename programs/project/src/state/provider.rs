use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Provider {
    pub authority: Pubkey,

    #[max_len(50)]
    pub name: String,

    #[max_len(50)]
    pub service: String,

    pub price_lamports: u64,
    pub registered: bool,
    pub bump: u8,
}