//! Program instruction processor
use arrayref::{array_ref, array_refs};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    borsh::try_from_slice_unchecked,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};
use std::{convert::TryInto, mem, io::BufWriter};

// New one 
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RectangleCurrent {
    width: u32,
    height: u32,
    area: u32,
    perimeter: u32,
}

// Old one 
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RectangleOld {
    width: u32,
    height: u32,
    area: u32,
}

impl RectangleCurrent{
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        (self.width + self.height)*2
    }
}


// Previous data size
const PREVIOUS_DATA_SIZE: usize = mem::size_of::<RectangleOld>();


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

    // The account must be owned by the program in order to modify its data
    if account.owner != _program_id {
        msg!("Rectangle account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    

    let mut account_data = account.data.borrow_mut();

    let mut new_account_data = conversion_logic(&account_data)?;

    new_account_data.perimeter = new_account_data.perimeter(); 
    
    new_account_data.serialize(&mut &mut account.data.borrow_mut()[..])?;

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

fn conversion_logic(src: &[u8]) -> Result<RectangleCurrent, ProgramError> {
    let past = array_ref![src, 0, PREVIOUS_DATA_SIZE];
    let (_, space) = array_refs![
        past,
        0,
        PREVIOUS_DATA_SIZE
    ];
    // Logic to upgrade from previous version
    // GOES HERE
    let old = try_from_slice_unchecked::<RectangleOld>(space).unwrap();

    // Copy the vaule and give back
    Ok(RectangleCurrent{
        width: old.width,
        height: old.height,
        area: old.area,
        perimeter: 0,
    })
}
