// use crate entrypoint;

use solana_program::{
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
};

/// Instructions.
// #[derive(Clone, Debug, PartialEq)]
pub enum TypeInstruction {
    /// Initialize user
    InitializeUser,
    
    ///Initialize meta
    InitializeMeta,

    /// TypeConfusion test
    Test,
}

impl TypeInstruction {
    /// Unpacks a byte buffer into a [DoorInstruction](enum.DoorInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(0)?;
        Ok(match tag {
            0 => {
                Self::InitializeUser
            }
            1 => {
                Self::InitializeMeta
            }
            2 => {
                Self::Test
            }
            _ => {
                return Err(ProgramError::InvalidArgument)
            }
        })
    }
}


                
