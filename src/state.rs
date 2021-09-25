use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::{Pubkey, PUBKEY_BYTES},
    program_error::ProgramError,
    program_pack::{IsInitialized,Pack,Sealed},
    msg, 
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolWallet {

    pub is_initialized: bool,

    pub manager : Pubkey, 
   // pub token_account : Pubkey, 

    pub token_count : u64,

    pub max_investor_count : u16, 

}


impl Sealed for PoolWallet {}


const POOL_WALLET_LENGTH : usize = 43 ; // 1 + 32 + 8 + 2

impl Pack for PoolWallet {

    const LEN: usize = POOL_WALLET_LENGTH;

    fn pack_into_slice(&self, dst: &mut [u8]) {

        let output = array_mut_ref![dst, 0, POOL_WALLET_LENGTH];
       
        let (
        is_initialized,
        manager, 
        token_count,
        max_investor_count,
        ) = mut_array_refs![ output,1,PUBKEY_BYTES,8,2];

    
        pack_bool(self.is_initialized, is_initialized);

        manager.copy_from_slice(self.manager.as_ref());
        //token_count.copy_from_slice(self.token_count);
        *token_count = self.token_count.to_le_bytes();
        *max_investor_count = self.max_investor_count.to_le_bytes();

    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
       
        let input = array_ref![src, 0, POOL_WALLET_LENGTH];
       
        let (is_initialized, manager, token_count, max_investor_count) = array_refs![input, 
        1, PUBKEY_BYTES, 8, 2];

        let is_init = unpack_bool(is_initialized).unwrap();

        Ok (Self {
            is_initialized : is_init, 
            manager : Pubkey::new_from_array(*manager),
            token_count : u64::from_le_bytes(*token_count),
            max_investor_count : u16::from_le_bytes(*max_investor_count),
            
        })
       
    }
}

impl IsInitialized for PoolWallet {
    fn is_initialized(&self) -> bool {
        
        self.is_initialized
    }
}



fn pack_bool(boolean: bool, dst: &mut [u8; 1]) {
    *dst = (boolean as u8).to_le_bytes()
}

fn unpack_bool(src: &[u8; 1]) -> Result<bool, ProgramError> {
    let b = u8::from_le_bytes(*src);
    match  b {
        0 => Ok(false),
        1 => Ok(true),
        _ => {
            msg!("Boolean cannot be unpacked");
            Err(ProgramError::InvalidAccountData)
        }
    }
}