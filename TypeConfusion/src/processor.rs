//! Program instruction processor
use crate::{state::{User, Metadata}, instruction::TypeInstruction};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
    borsh::try_from_slice_unchecked
};


/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TypeInstruction::unpack(instruction_data)?;

    match instruction {
        TypeInstruction::InitializeUser => {
            msg!("Instruction: InitializeUser");
            InitializeUser(accounts)
        }
        TypeInstruction::InitializeMeta => {
            msg!("Instruction: InitializeConfig");
            InitializeMeta(accounts)
        }
        TypeInstruction::Test => {
            msg!("Instruction: Test");
            Test(program_id, accounts)
        }
     }
}


/// Initialize User
pub fn InitializeUser(
    accounts: &[AccountInfo]
) -> ProgramResult {
     let account_info_iter = &mut accounts.iter();
     let user_info = next_account_info(account_info_iter)?;
     let authority_info = next_account_info(account_info_iter)?;

     /// deserializing 
     let mut user = User::try_from_slice_unchecked(&user_info.data.borrow())?;
     
     user.authority = authority_info.key;
     
     /// serializing
     user.serialize(&mut &mut user_info.data.borrow_mut()[..])?;

     Ok(())
}

/// Initialize Meta
pub fn InitializeMeta(
    accounts: &[AccountInfo],
) -> ProgramResult {
     let account_info_iter = &mut accounts.iter();
     let meta_info = next_account_info(account_info_iter)?;
     let account_info = next_account_info(account_info_iter)?;

     /// deserializing 
     let mut meta = Metadata::try_from_slice_unchecked(&meta_info.data.borrow())?;
     
     meta.account = account_info.key;
     
     /// serializing
     meta.serialize(&mut &mut meta_info.data.borrow_mut()[..])?;

     Ok(())
}

/// Test: Aims to pass in User Account 
pub fn Test(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
     
    if user_info.owner != program_id {
        msg!("User account should be owned by the program!");
        return Err(ProgramError::IncorrectProgramId);
    }

    /// deserializing 
    let mut user = User::try_from_slice_unchecked(&user_info.data.borrow())?;
    if authority_info.key != user.authority {
        msg!("The authority of the user account should match the authority account passed in");
        return Err(ProgramError::InvalidArgument);
    }
    
    if !authority_info.is_signer {
        msg!("MissingRequiredSignature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!(Test Passed!!!);

    Ok(())

}









