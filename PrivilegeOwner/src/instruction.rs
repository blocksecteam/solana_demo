// use crate entrypoint;

use solana_program::{
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use std::mem::size_of;

/// Instructions supported by the lending program.
// #[derive(Clone, Debug, PartialEq)]
pub enum DoorInstruction {
    /// InitializeDoor
    InitializeDoor {key: Pubkey},
    /// InitializeAccount
    InitializeAccount,
    /// Open
    Open,
    /// Close
    Close,
}

impl DoorInstruction {
    /// Unpacks a byte buffer into a [DoorInstruction](enum.DoorInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(0)?;
        Ok(match tag {
            0 => {
                let (key, _rest) = Self::unpack_pubkey(rest)?;
                Self::InitializeDoor { key }
            }
            1 => {
                Self::InitializeAccount
            }
            2 => {
                Self::Open
            }
            3 => {
                Self::Close
            }
            _ => {
                msg!("Instruction cannot be unpacked");
                return Err(ProgramError::InvalidArgument);
            }
        })
    }
    /// unpack pubkey
    pub fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() < PUBKEY_BYTES {
            msg!("Pubkey cannot be unpacked");
            return Err(solana_program::program_error::ProgramError::Custom(0));
        }
        let (key, rest) = input.split_at(PUBKEY_BYTES);
        let pk = Pubkey::new(key);
        Ok((pk, rest))
    }
    

}


                
