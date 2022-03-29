//! Program instruction processor
use crate::{state::{Door, Account}, instruction::DoorInstruction};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    borsh::try_from_slice_unchecked,
    account_info::{next_account_info, AccountInfo},
    program_memory::{sol_memcmp, sol_memset},
    entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
    program_error::ProgramError
};
use std::convert::TryInto;



/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = DoorInstruction::unpack(instruction_data)?;

    match instruction {
        DoorInstruction::InitializeDoor {
             key: Pubkey
        } => {
            msg!("Instruction: InitializeDoor");
            Self::InitializeDoor(accounts, key)
        }
        DoorInstruction::InitializeAccount => {
            msg!("Instruction: InitializeAccount");
            Self::InitializeAccount(program_id, accounts)
        }
        DoorInstruction::Open => {
            msg!("Instruction: Open");
            Self::Open(program_id, accounts)
        }
        DoorInstruction::Close => {
            msg!("Instruction: Close");
            Self::Close(program_id, accounts)
        }
     }
}


/// Initialize Door
pub fn InitializeDoor(
    accounts: &[AccountInfo],
    key: Pubkey,
) -> ProgramResult {
     let account_info_iter = &mut accounts.iter();
     let door_info = next_account_info(account_info_iter)?;
     
     /// deserializing 
     let mut door = Door::unpack_unchecked(&door_info.data.borrow())?;
     if door.is_initialized {
        return Err(ProgramError::InvalidArgument);
     }
     
     door.key = key;
     door.is_initialized = true;
     door.is_opened = false;
     
     /// serializing
     Door::pack(door, &mut account_info.data.borrow_mut())?;

     Ok(())
}

/// Initialize Account
pub fn InitializeAccount(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let new_account_info = next_account_info(account_info_iter)?;
    let door_info = next_account_info(account_info_iter)?;
    let owner = next_account_info(account_info_iter)?;
    
    Self::check_account_owner(program_id, door_info)?;
    /// deserializing
    let mut account = Account::unpack_unchecked(&new_account_info.data.borrow())?;
    if account.is_initialized() {
        return Err(ProgramError::InvalidArgument);
    }

    account.door = *door_info.key;
    account.owner = *owner.key; 

    Account::pack(account, &mut new_account_info.data.borrow_mut())?;

    Ok(())
}

/// Open
pub fn Open(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let door_info = next_account_info(account_info_iter)?;
    let account_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    
    Self::check_account_owner(program_id, door_info)?;
    /// deserializing
    let mut account = Account::unpack(&account_info.data.borrow())?;
    if !Self::cmp_pubkeys(door_info.key, &account.door) {
            return Err(ProgramError::InvalidArgument);
    }
    if !Self::cmp_pubkeys(owner_info.key, &account.owner) {
        return Err(ProgramError::InvalidArgument);
    }

    let mut door = Door::unpack(&door_info.data.borrow())?;

    let expected_owner = door.key;

    Self::validate_owner(expected_owner, owner_info)?;
    
    if  door.is_opened {
        return Err(ProgramError::InvalidArgument);
    }

    door.is_opened = true;

    Account::pack(
        account,
        &mut account_info.data.borrow_mut(),
    )?;
    Door::pack(
        door,
        &mut door_info.data.borrow_mut()
    )?;
    
    Ok(())
}    

/// Close
pub fn Close(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let door_info = next_account_info(account_info_iter)?;
    let account_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    
    Self::check_account_owner(program_id, door_info)?;
    /// deserializing
    let mut account = Account::unpack(&account_info.data.borrow())?;
    if !Self::cmp_pubkeys(door_info.key, &account.door) {
        return Err(ProgramError::InvalidArgument);
    }
    if !Self::cmp_pubkeys(owner_info.key, &account.owner) {
        return Err(ProgramError::InvalidArgument);
    }
    
    let mut door = Door::unpack(&door_info.data.borrow())?;

    let expected_owner = door.key;

    Self::validate_owner(expected_owner, owner_info);
    
    if !door.is_opened {
        return Err(ProgramError::InvalidArgument);
    }

    door.is_opened = false;

    Account::pack(
        account,
        &mut account_info.data.borrow_mut(),
    )?;
    Door::pack(
        door,
        &mut door_info.data.borrow_mut()
    )?;
    
    Ok(())
} 

/// Validate Owner 

pub fn validate_owner(
    expected_owner: &Pubkey,
    owner_account_info: &AccountInfo,
) -> ProgramResult {
    if !Self::cmp_pubkeys(expected_owner, owner_account_info.key) {
        return Err(ProgramError::InvalidArgument);
    }
    if !owner_account_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    Ok(())
}

// Checks that the account is owned by the expected program
pub fn check_account_owner(program_id: &Pubkey, account_info: &AccountInfo) -> ProgramResult {
    if !Self::cmp_pubkeys(program_id, account_info.owner) {
        Err(ProgramError::IncorrectProgramId)
    } else {
        Ok(())
    }
}

/// Checks two pubkeys for equality in a computationally cheap way using
    /// `sol_memcmp`
pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
}








