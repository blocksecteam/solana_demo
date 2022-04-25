use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey
};



/// User 
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct User {
    /// authority
    pub authority: Pubkey
}



/// Metadata
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Metadata {
    /// account
    pub account: Pubkey,
}

