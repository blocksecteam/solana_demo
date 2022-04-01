//! Program instruction processor
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    program_pack::{IsInitialized, Pack, Sealed},
};

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

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Create in iterator to safety reference accounts in the slice
    let account_info_iter = &mut accounts.iter();

    // Account info for the program being invoked
    let fake_info = next_account_info(account_info_iter)?;
    // Account info to allocate
    let owner_info = next_account_info(account_info_iter)?;
  
    let mut config = Config::unpack_unchecked(&fake_info.data.borrow())?;

    config.admin = owner_info.key; 
    config.is_locked = false;
    config.is_initialized = true;
    
    Config::pack(
        config,
        &mut fake_info.data.borrow_mut()
    )?;
    

    Ok(())
}
