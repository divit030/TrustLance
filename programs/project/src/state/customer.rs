
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Customer {
   
    pub authority: Pubkey,

   
    #[max_len(50)]
    pub name: String,

    
    pub registered: bool,


    pub bump: u8,
}
