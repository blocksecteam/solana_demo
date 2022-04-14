
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::{Pubkey, PUBKEY_BYTES},
    msg,
};
use crate::instruction::MAX_SIGNERS;
use std::{io::BufWriter, mem};




/// Multisig account 
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Multisig {
    /// Number of signers required
    pub m: u8,
    /// Number of valid signers
    pub n: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Signer public keys
    pub signers: [Pubkey; MAX_SIGNERS],
}

impl Sealed for Multisig {}
impl IsInitialized for Multisig {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Multisig {
    const LEN: usize = 1024;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 355];
        #[allow(clippy::ptr_offset_with_cast)]
        let (m, n, is_initialized, signers_flat) = array_refs![src, 1, 1, 1, 32 * MAX_SIGNERS];
        let mut result = Multisig {
            m: m[0],
            n: n[0],
            is_initialized: match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            signers: [Pubkey::new_from_array([0u8; 32]); MAX_SIGNERS],
        };
        for (src, dst) in signers_flat.chunks(32).zip(result.signers.iter_mut()) {
            *dst = Pubkey::new(src);
        }
        Ok(result)
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 355];
        #[allow(clippy::ptr_offset_with_cast)]
        let (m, n, is_initialized, signers_flat) = mut_array_refs![dst, 1, 1, 1, 32 * MAX_SIGNERS];
        *m = [self.m];
        *n = [self.n];
        *is_initialized = [self.is_initialized as u8];
        for (i, src) in self.signers.iter().enumerate() {
            let dst_array = array_mut_ref![signers_flat, 32 * i, 32];
            dst_array.copy_from_slice(src.as_ref());
        }
    }
}

/// Transaction Account 
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}


/// Multisig account 
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Transaction {
    /// The multisig account this transaction belongs to.
    pub multisig: Pubkey,
    /// Target program to execute against.
    pub program_id: Pubkey,
    /// accounts passed to target program 
    pub accounts: [TransactionAccount; 2],
    /// instruction data 
    pub data: u8,
    /// signers[index] is true iff multisig.owners[index] signed the transaction.
    pub signers: [bool; MAX_SIGNERS],
    /// Boolean ensuring one time execution.
    pub did_execute: bool,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
}

impl Sealed for Transaction {}
impl IsInitialized for Transaction {
    fn is_initialized(&self) -> bool { 
        self.is_initialized
    }
}

impl Pack for Transaction {
    const LEN: usize = 1024;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 146];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            multisig, 
            program_id, 
            pubkey1,
            is_signer1,
            is_writable1,
            pubkey2,
            is_signer2,
            is_writable2, 
            data, 
            signers_flat, 
            did_execute, 
            is_initialized
        ) = array_refs![
            src, 
            32, 
            32, 
            32,
            1,
            1,
            32,
            1,
            1, 
            1, 
            MAX_SIGNERS, 
            1, 
            1];
        let mut t1 = TransactionAccount {
            pubkey: Pubkey::new_from_array(*pubkey1),
            is_signer: match is_signer1 {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            is_writable: match is_writable1 {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
        }
        let mut t2 = TransactionAccount {
            pubkey: Pubkey::new_from_array(*pubkey2),
            is_signer: match is_signer2 {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            is_writable: match is_writable2 {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
        }
        let mut result = Transaction {
            multisig: Pubkey::new_from_array(*multisig),
            program_id: Pubkey::new_from_array(*program_id),
            accounts: [t1, t2],
            data: data[0],
            signers: [false; MAX_SIGNERS],
            did_execute: match did_execute {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            is_initialized: match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
        }
        for (src, dst) in signers_flat.chunks(1).zip(result.signers.iter_mut()) {
            if src = [0] {
               *dst = true; 
            } else {
               *dst = false; 
            }
        }
        Ok(result)
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 146];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            multisig, 
            program_id, 
            pubkey1,
            is_signer1,
            is_writable1,
            pubkey2,
            is_signer2,
            is_writable2, 
            data, 
            signers_flat, 
            did_execute, 
            is_initialized
        ) = mut_array_refs![
            dst,
            32, 
            32, 
            32,
            1,
            1,
            32,
            1,
            1, 
            1, 
            MAX_SIGNERS, 
            1, 
            1,
        ];
        multisig.copy_from_slice(self.multisig.as_ref());
        program_id.copy_from_slice(self.program_id.as_ref());
        pubkey1.copy_from_slice(self.pubkey1.as_ref());
        *is_signer1 = [self.is_signer1 as u8];
        *is_writable1 = [self.is_writable1 as u8];
        pubkey2.copy_from_slice(self.pubkey2.as_ref());
        *is_signer2 = [self.is_signer2 as u8];
        *is_writable2 = [self.is_writable2 as u8];
        *data = [self.data as u8];
        for i in 0..MAX_SIGNERS {
            signers_flat[i] = self.signers[i] as u8;
        }
        *did_execute_dst = [self.did_execute as u8];
        *is_initialized_dst = [self.is_initialized as u8];
    }
}


