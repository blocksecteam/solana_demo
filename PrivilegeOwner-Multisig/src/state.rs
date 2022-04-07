
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::{Pubkey, PUBKEY_BYTES},
    msg,
};
use crate::instruction::MAX_SIGNERS;

/// Door 
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Door {
    /// The key that can open the door
    pub key: Pubkey,
    /// Default to false  
    pub is_initialized: bool,
    /// Default to closed 
    pub is_opened: bool 
}


impl Sealed for Door {}
impl IsInitialized for Door {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Door {
    const LEN: usize = 1024;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 34];
        let (key, is_initialized, is_opened) =
            array_refs![src, 32, 1, 1];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_opened = match is_opened {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Door {
            key: Pubkey::new_from_array(*key),
            is_initialized,
            is_opened,
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 34];
        let (key_dst, is_initialized_dst, is_opened_dst) = 
             mut_array_refs![dst, 32, 1, 1];

        let &Door {
            ref key,
            is_initialized,
            is_opened,
        } = self;

        key_dst.copy_from_slice(key.as_ref());
        is_initialized_dst[0] = is_initialized as u8;
        is_opened_dst[0] = is_opened as u8;
    }
}


/// Account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Config {
    /// The admin of the config
    pub admin: Pubkey,

    /// locked or not  
    pub is_locked: bool,

    /// Default to false  
    pub is_initialized: bool
}


impl Sealed for Config {}
impl IsInitialized for Config {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Config {
    const LEN: usize = 1024;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 34];
        let (admin, is_locked, is_initialized) =
            array_refs![src, 32, 1, 1];
        
        let is_locked = match is_locked {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
       
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Config {
            admin: Pubkey::new_from_array(*admin),
            is_locked: is_locked,
            is_initialized: is_initialized,
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 34];
        let (admin_dst, is_locked_dst, is_initialized_dst) = 
             mut_array_refs![dst, 32, 1, 1];

        let &Config {
            ref admin,
            is_locked,
            is_initialized,
        } = self;

        admin_dst.copy_from_slice(admin.as_ref());
        is_locked_dst[0] = is_locked as u8;
        is_initialized_dst[0] = is_initialized as u8;
    }
}


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
    const LEN: usize = 355;
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