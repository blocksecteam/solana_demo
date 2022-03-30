
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::{Pubkey, PUBKEY_BYTES},
};



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
    const LEN: usize = 34;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 34];
        let (key, is_initialized, is_opened) =
            array_refs![src, 32, 1, 1];
        msg!("key: {:?}", key);
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
pub struct Account {
    /// The door associated with this account
    pub door: Pubkey,

    /// The owner of the account 
    pub owner: Pubkey,

    /// Default to false  
    pub is_initialized: bool
}


impl Sealed for Account {}
impl IsInitialized for Account {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Account {
    const LEN: usize = 65;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 65];
        let (door, owner, is_initialized) =
            array_refs![src, 32, 32, 1];
        
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Account {
            door: Pubkey::new_from_array(*door),
            owner: Pubkey::new_from_array(*owner),
            is_initialized: is_initialized,
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 65];
        let (door_dst, owner_dst, is_initialized_dst) = 
             mut_array_refs![dst, 32, 32, 1];

        let &Account {
            ref door,
            ref owner,
            is_initialized,
        } = self;

        door_dst.copy_from_slice(door.as_ref());
        owner_dst.copy_from_slice(owner.as_ref());
        is_initialized_dst[0] = is_initialized as u8;
    }
}