//! Program instruction processor
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    borsh::try_from_slice_unchecked,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
    program_error::ProgramError
};
use std::convert::TryInto;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    area: u32,
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
    let (b, rest) = unpack_u32(rest_b)?;

    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data.
    if account.owner != _program_id {
        msg!("Rectangle account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut rectangle1 = try_from_slice_unchecked::<Rectangle>(&account.data.borrow())?;
    
    rectangle1.width = a;
    rectangle1.height = b;
    rectangle1.area = rectangle1.area();

    rectangle1.serialize(&mut &mut account.data.borrow_mut()[..])?;
    

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
