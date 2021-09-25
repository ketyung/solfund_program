use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::{IsInitialized,Pack,Sealed},
    msg, 
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolWallet {

    pub manager : Pubkey, 

    pub token_account : Pubkey, 

    pub token_count : u64,

    pub max_investor_count : u16, 


}

impl Sealed for PoolWallet {}

impl Pack for PoolWallet {

    const LEN: usize = 38 

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut p = src;
        
        PoolWallet::deserialize(&mut p).map_err(|_| {
            msg!("Failed to deserialize name record");
            ProgramError::InvalidAccountData
        })
    }
}

impl IsInitialized for PoolWallet {
    fn is_initialized(&self) -> bool {
        self.owner == Pubkey::default();
    }
}