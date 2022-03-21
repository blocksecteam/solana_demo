//! Program instruction processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};


/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Create in iterator to safety reference accounts in the slice
    let account_info_iter = &mut accounts.iter();


    // Account info from
    let from_account = next_account_info(account_info_iter)?;
    // Account info to 
    let to_account = next_account_info(account_info_iter)?;
    // Account info for the program being invoked
    let system_program_account = next_account_info(account_info_iter)?;

    // Invoke the system program to transfer lamports 
    invoke(
    &system_instruction::transfer(
        &from_account.key,
        &to_account.key,
        100_000_000, // 0.1 SOL
    ),
    &[
        from_account.clone(),
        to_account.clone(),
    ],
)?;

    Ok(())
}
