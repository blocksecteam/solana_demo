//! Program instruction processor
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};
use std::convert::TryInto;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    amount: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let (a, rest_b) = unpack_u32(_instruction_data)?;
    let (b, rest_c) = unpack_u32(rest_b)?;
    let (c, rest) = unpack_u32(rest_c)?;
    msg!("width: {:?}, height: {:?}, amount: {:?}", a, b, c);

    let rect1 = Rectangle {
        width: a,
        height: b,
        amount: c
    };

    msg!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    let amount = rect1.amount;

    msg!(
        "The amount of the rectangle is {}.",
        amount
    );


    Ok(())
}

fn unpack_u32(input: &[u8]) -> Result<(u32, &[u8]), ProgramError> {
    if input.len() < 4 {
        msg!("u64 cannot be unpacked");
        return Err(ProgramError::InvalidInstructionData);
    }
    let (bytes, rest) = input.split_at(4);
    let value = bytes
        .get(..4)
        .and_then(|slice| slice.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;
    Ok((value, rest))
}
