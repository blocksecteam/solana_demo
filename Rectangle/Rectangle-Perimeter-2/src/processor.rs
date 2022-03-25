//! Program instruction processor
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    borsh::try_from_slice_unchecked,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
    program_error::ProgramError
};
use std::{convert::TryInto, mem, io::BufWriter, ops::DerefMut};

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct CurrentRectangle {
    width: u32,
    height: u32,
    perimeter: u32,
    area: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct OldRectangle {
    width: u32,
    height: u32,
    area: u32,
}


impl CurrentRectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        (self.width + self.height)*2
    }
}




/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data.
    if account.owner != _program_id {
        msg!("Rectangle account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

   
   let (i, rest_ab) = unpack_u32(_instruction_data)?;
   let (a, rest_a) = unpack_u32(rest_ab)?;
   let (b, rest) = unpack_u32(rest_a)?;

   match i {
      0 => initialize(accounts, a, b),
      1 => upgrade(accounts),
   }

}


pub fn initialize(
    accounts: &[AccountInfo],
    a: u32,
    b: u32,
) -> ProgramResult {
    
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    let mut rectangle1 = try_from_slice_unchecked::<CurrentRectangle>(&account.data.borrow())?;
    

    rectangle1.width = a;
    rectangle1.height = b;
    rectangle1.area = rectangle1.area();
    rectangle1.perimeter = rectangle1.perimeter();

    rectangle1.serialize(&mut &mut account.data.borrow_mut()[..])?;
    

    Ok(())
}


pub fn upgrade(
    accounts: &[AccountInfo],
) -> ProgramResult {
    
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    let mut update_account = Rectangle::result(accounts); 
    
    update_account.perimeter = update_account.perimeter();
    
    let mut account_data = account.data.borrow_mut();

    let mut deref = account_data.deref_mut();
    
    let mut bw = BufWriter::new(deref);

    update_account.serialize(&mut bw)?;
    
    
    Ok(())
}


fn conversion_logic(
    accounts: &[AccountInfo]
    ) -> Result<CurrentRectangle, ProgramError> {

    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    let old = try_from_slice_unchecked::<OldRectangle>(&account.data.borrow())?;    
    
    Ok(CurrentRectangle{
        width: old.width,
        height: old.height,
        perimeter: 0,
        area: old.area,
    })

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
