// use crate entrypoint;

use solana_program::{
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use std::mem::size_of;


/// Minimum number of multisignature signers (min N)
pub const MIN_SIGNERS: usize = 1;
/// Maximum number of multisignature signers (max N)
pub const MAX_SIGNERS: usize = 11;

/// Instructions.
// #[derive(Clone, Debug, PartialEq)]
pub enum DoorInstruction {
    /// InitializeDoor
    InitializeDoor {
       /// pubkey
       key: Pubkey
    },
    ///
    InitializeConfig {
       /// pubkey
       key: Pubkey
    },
    /// lock
    Lock,
    /// unlock
    Unlock,
    /// Open
    Open,
    /// Close
    Close,
    /// AllocatePDA
    AllocatePDA
    /// InitializeMultisig
    InitializeMultisig {
        /// The number of signers (M) required to validate this multisignature
        /// account.
        m: u8,
    },
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
                let (key, _rest) = Self::unpack_pubkey(rest)?;
                Self::InitializeConfig { key }
            }
            2 => {
                Self::Lock
            }
            3 => {
                Self::Unlock
            }
            4 => {
                Self::Open
            }
            5 => {
                Self::Close
            }
            6 => {
                Self::AllocatePDA
            }
            7 => {
                let &m = rest.get(0).ok_or(ProgramError::InvalidArgument)?;
                Self::InitializeMultisig { m }
            }
            _ => {
                return Err(ProgramError::InvalidArgument)
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

/// Utility function that checks index is between MIN_SIGNERS and MAX_SIGNERS
pub fn is_valid_signer_index(index: usize) -> bool {
    (MIN_SIGNERS..=MAX_SIGNERS).contains(&index)
}                
