//! Program instruction processor
use crate::{state::{Multisig, Transaction}, instruction::{MultisigInstruction, is_valid_signer_index, MAX_SIGNERS}};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    borsh::try_from_slice_unchecked,
    account_info::{next_account_info, AccountInfo},
    program_memory::{sol_memcmp, sol_memset},
    entrypoint::ProgramResult, msg,
    pubkey::{Pubkey,PUBKEY_BYTES},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    instruction::Instruction,
    program::invoke_signed,
    system_instruction
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
    let instruction = MultisigInstruction::unpack(instruction_data)?;

    match instruction {
        MultisigInstruction::AllocatePDA => {
            msg!("Instruction: Allocate");
            AllocatePDA(program_id, accounts)
        }
        MultisigInstruction::InitializeMultisig {
             m
        } => {
            msg!("Instruction: InitializeMultisig");
            InitializeMultisig(program_id, accounts, m)
        }
        MultisigInstruction::CreateTransaction {
            target_program_id,
            data
        }=> {
            msg!("Instruction: CreateTransaction");
            CreateTransaction(program_id, accounts, target_program_id, data)
        }
        MultisigInstruction::Approve => {
            msg!("Instruction: Approve");
            Approve(program_id, accounts)
        }
        MultisigInstruction::ExecuteTransaction => {
            msg!("Instruction: ExecuteTransaction");
            ExecuteTransaction(program_id, accounts)
        }        
    }
}


/// Allocate PDA
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

    // Invoke the system program to create account 
    invoke_signed(
        &system_instruction::create_account(
            &owner_info.key,
            &allocated_info.key,
            /// lamports
            80179200,
            SIZE as u64,
            &program_id, 
            ),
        // Order doesn't matter and this slice could include all the accounts and be:
        // `&accounts`
        &[  owner_info.clone(),
            system_program_info.clone(), // program being invoked also needs to be included
            allocated_info.clone(),
        ],

        &[&[b"You pass butter", &[bump]]],
    )?;

    Ok(())
}

/// Initialize Multisig
pub fn InitializeMultisig(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    m: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let multisig_info = next_account_info(account_info_iter)?;
     
    let (expected_allocated_key, bump) =
        Pubkey::find_program_address(&[b"You pass butter"], program_id);
    
    if *multisig_info.key != expected_allocated_key {
        return Err(ProgramError::InvalidArgument);
    }
         
    /// deserializing 
    let mut multisig = Multisig::unpack_unchecked(&multisig_info.data.borrow())?;
    if multisig.is_initialized {
       return Err(ProgramError::InvalidArgument);
    }
     
    let signer_infos = account_info_iter.as_slice();
    multisig.m = m;
    multisig.n = signer_infos.len() as u8;
    if !is_valid_signer_index(multisig.n as usize) {
        return Err(ProgramError::InvalidArgument);
    }
    if !is_valid_signer_index(multisig.m as usize) {
        return Err(ProgramError::InvalidArgument);
    }
    for (i, signer_info) in signer_infos.iter().enumerate() {
        multisig.signers[i] = *signer_info.key;
    }
    multisig.is_initialized = true;
     
    /// serializing
    Multisig::pack(multisig, &mut multisig_info.data.borrow_mut())?;

    Ok(())
}

/// Create Transaction
pub fn CreateTransaction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    target_program_id: &Pubkey,
    data: u8,
) -> ProgramResult {
     let account_info_iter = &mut accounts.iter();
     let transaction_info = next_account_info(account_info_iter)?;
     let account1_info = next_account_info(account_info_iter)?;
     let account2_info = next_account_info(account_info_iter)?;

     /// deserializing 
     let mut transaction = Transaction::unpack_unchecked(&transaction_info.data.borrow())?;
     if transaction.is_initialized {
        return Err(ProgramError::InvalidArgument);
     }
     
     let (expected_allocated_key, bump) =
        Pubkey::find_program_address(&[b"You pass butter"], program_id);
     
     transaction.multisig = expected_allocated_key;
     transaction.program_id = target_program_id;
     transaction.accounts = [account1_info, account2_info];
     transaction.data = data;
     transaction.signers = [false; MAX_SIGNERS];
     transaction.did_execute = false;
     transaction.is_initialized = true;
     
     
     /// serializing
     Transaction::pack(transaction, &mut transaction_info.data.borrow_mut())?;

     Ok(())
}


/// Approve 
pub fn Approve(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let transaction_info = next_account_info(account_info_iter)?;
    let multisig_info = next_account_info(account_info_iter)?;
    let your_info = next_account_info(account_info_iter)?;
     
    check_account_owner(program_id, transaction_info)?;
    check_account_owner(program_id, multisig_info)?;
    /// deserializing
    let mut transaction = Transaction::unpack_unchecked(&transaction_info.data.borrow())?;
    let mut multisig = Multisig::unpack_unchecked(&multisig_info.data.borrow())?;
    
    for (position, key) in multisig.signers[0..multisig.n as usize].iter().enumerate() {
        if cmp_pubkeys(key, your_info.key) && !transaction.signers[position] {
            if !your_info.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
            transaction.signers[position] = true;
        }     
    }
    Transaction::pack(transaction, &mut transaction_info.data.borrow_mut())?;

    Ok(())

}

/// Execute Transaction
pub fn ExecuteTransaction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let transaction_info = next_account_info(account_info_iter)?;
    let multisig_info = next_account_info(account_info_iter)?;
     
    check_account_owner(program_id, transaction_info)?;
    check_account_owner(program_id, multisig_info)?;

    /// deserializing
    let mut transaction = Transaction::unpack_unchecked(&transaction_info.data.borrow())?;
    let mut multisig = Multisig::unpack_unchecked(&multisig_info.data.borrow())?;
    
    /// number of valid signatures
    let mut num_signers = 0;
    for (position, _) in transaction.signers[0..multisig.n as usize].iter().enumerate() {
        if  transaction.signers[position] {
            num_signers += 1
        }     
    }
    
    if num_signers < multisig.m {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    let mut ix = Instruction {
        program_id: transaction.program_id, 
        accounts: transaction.accounts,
        data: transaction.data,
    };
    
    let (expected_allocated_key, bump) =
        Pubkey::find_program_address(&[b"You pass butter"], program_id);
        
    invoke_signed(
        &ix,
        // Order doesn't matter and this slice could include all the accounts and be:
        // `&accounts`
        &transaction.accounts,
        &[&[b"You pass butter", &[bump]]],
    )?; 

    transaction.did_execute = true;
    

    Transaction::pack(transaction, &mut transaction_info.data.borrow_mut())?;


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








