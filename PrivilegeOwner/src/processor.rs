//! Program instruction processor
use crate::{state::{Door, Config}, instruction::DoorInstruction};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    borsh::try_from_slice_unchecked,
    account_info::{next_account_info, AccountInfo},
    program_memory::{sol_memcmp, sol_memset},
    entrypoint::ProgramResult, msg,
    pubkey::{Pubkey,PUBKEY_BYTES},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    system_instruction,
    program::invoke_signed,
};
use std::convert::TryInto;


/// Size of PDA 
pub const SIZE: usize = 1024;

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = DoorInstruction::unpack(instruction_data)?;

    match instruction {
        DoorInstruction::InitializeDoor {
             key
        } => {
            msg!("Instruction: InitializeDoor");
            InitializeDoor(accounts, key)
        }
        DoorInstruction::InitializeConfig {
             key
        } => {
            msg!("Instruction: InitializeConfig");
            InitializeConfig(program_id, accounts, key)
        }
        DoorInstruction::Lock => {
            msg!("Instruction: Lock");
            Lock(program_id, accounts)
        }
        DoorInstruction::Unlock => {
            msg!("Instruction: Unlock");
            Unlock(program_id, accounts)
        }
        DoorInstruction::Open => {
            msg!("Instruction: Open");
            Open(program_id, accounts)
        }
        DoorInstruction::Close => {
            msg!("Instruction: Close");
            Close(program_id, accounts)
        }
        DoorInstruction::AllocatePDA => {
            msg!("Instruction: Allocate");
            AllocatePDA(program_id, accounts)
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
     Door::pack(door, &mut door_info.data.borrow_mut())?;

     Ok(())
}

/// Initialize Config
pub fn InitializeConfig(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    key: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let config_info = next_account_info(account_info_iter)?;
     
    let (expected_allocated_key, bump) =
        Pubkey::find_program_address(&[b"You pass butter"], program_id);
    
    if *config_info.key != expected_allocated_key {
        return Err(ProgramError::InvalidArgument);
    }
         
    /// deserializing 
    let mut config = Config::unpack_unchecked(&config_info.data.borrow())?;
    if config.is_initialized {
       return Err(ProgramError::InvalidArgument);
    }
     
    config.admin = key;
    config.is_locked = true;
    config.is_initialized = true;
     
    /// serializing
    Config::pack(config, &mut config_info.data.borrow_mut())?;

    Ok(())
}

/// Lock the door 
pub fn Lock(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let config_info = next_account_info(account_info_iter)?;
    let admin_info = next_account_info(account_info_iter)?;
     
    check_account_owner(program_id, config_info)?;
    /// deserializing
    let mut config = Config::unpack(&config_info.data.borrow())?;
    if !cmp_pubkeys(admin_info.key, &config.admin) {
        return Err(ProgramError::InvalidArgument);
    }
     
    if !admin_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    if !config.is_initialized {
        return Err(ProgramError::InvalidArgument);
    } 
    
    if  config.is_locked {
        return Err(ProgramError::InvalidArgument);
    } 

    config.is_locked = true;
    
    Config::pack(config, &mut config_info.data.borrow_mut())?;

    Ok(())

}

/// Unlock the door 
pub fn Unlock(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let config_info = next_account_info(account_info_iter)?;
    let admin_info = next_account_info(account_info_iter)?;
     
    check_account_owner(program_id, config_info)?;
    /// deserializing
    let mut config = Config::unpack(&config_info.data.borrow())?;
    if !cmp_pubkeys(admin_info.key, &config.admin) {
        return Err(ProgramError::InvalidArgument);
    }
     
    if !admin_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    if !config.is_initialized {
        return Err(ProgramError::InvalidArgument);
    } 
    
    if  !config.is_locked {
        return Err(ProgramError::InvalidArgument);
    } 

    config.is_locked = false;
    
    Config::pack(config, &mut config_info.data.borrow_mut())?;

    Ok(())

}

/// Open
pub fn Open(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let door_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    
    check_account_owner(program_id, door_info)?;
    check_account_owner(program_id, config_info)?;

    /// deserializing
    let mut config = Config::unpack(&config_info.data.borrow())?;

    if config.is_locked {
        return Err(ProgramError::InvalidArgument);
    } 
    

    let mut door = Door::unpack(&door_info.data.borrow())?;

    let expected_owner = door.key;

    validate_owner(&expected_owner, owner_info)?;
    
    if  door.is_opened {
        return Err(ProgramError::InvalidArgument);
    }

    door.is_opened = true;

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
    let config_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    
    check_account_owner(program_id, door_info)?;
    check_account_owner(program_id, config_info)?;
    
    /// deserializing
    let mut config = Config::unpack(&config_info.data.borrow())?;
    
    if config.is_locked {
        return Err(ProgramError::InvalidArgument);
    } 

    let mut door = Door::unpack(&door_info.data.borrow())?;

    let expected_owner = door.key;

    validate_owner(&expected_owner, owner_info);
    
    if !door.is_opened {
        return Err(ProgramError::InvalidArgument);
    }

    door.is_opened = false;

    Door::pack(
        door,
        &mut door_info.data.borrow_mut()
    )?;
    
    Ok(())
} 


/// AllocatePDA
pub fn AllocatePDA(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    // Create in iterator to safety reference accounts in the slice
    let account_info_iter = &mut accounts.iter();

    // Account info for the program being invoked
    let system_program_info = next_account_info(account_info_iter)?;
    // Account info to allocate
    let allocated_info = next_account_info(account_info_iter)?;

    let owner_info = next_account_info(account_info_iter)?;

    let (expected_allocated_key, bump) =
        Pubkey::find_program_address(&[b"You pass butter"], program_id);
    
    if *allocated_info.key != expected_allocated_key {
        // allocated key does not match the derived address
        return Err(ProgramError::InvalidArgument);
    }

    // Invoke the system program to allocate account data
    invoke_signed(
        &system_instruction::create_account(
            &owner_info.key,
            &allocated_info.key,
            1,
            SIZE as u64,
            &program_id, 
            ),
        // Order doesn't matter and this slice could include all the accounts and be:
        // `&accounts`
        &[
            system_program_info.clone(), // program being invoked also needs to be included
            allocated_info.clone(),
        ],

        &[&[b"You pass butter", &[bump]]],
    )?;

    Ok(())
}

/// Validate Owner 

pub fn validate_owner(
    expected_owner: &Pubkey,
    owner_account_info: &AccountInfo,
) -> ProgramResult {
    if !cmp_pubkeys(expected_owner, owner_account_info.key) {
        return Err(ProgramError::InvalidArgument);
    }
    if !owner_account_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    Ok(())
}

/// Checks that the account is owned by the expected program
pub fn check_account_owner(program_id: &Pubkey, account_info: &AccountInfo) -> ProgramResult {
    if !cmp_pubkeys(program_id, account_info.owner) {
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








