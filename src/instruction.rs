use crate::{error::PoolError};
use crate::state::{FundPool,unpack_bool}; //, PoolMarket};

use solana_program::{
    program_error::ProgramError,
    msg, 
    program_pack::{Pack},
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use arrayref::{array_ref,  array_refs};



#[derive(Clone, Debug, PartialEq)]
pub enum PoolInstruction {

    CreateFundPool {

        manager : Pubkey,

        lamports : u64,

        token_count : u64, 

        is_finalized : bool,

        icon : u16, 

    },

    UpdateFundPool {

        pool : FundPool,
    },

    CreatePoolMarket,

    RegisterAddrInPoolMarket {

        address : Pubkey, 
    },

    RemoveAddrFromPoolMarket {

        address : Pubkey, 
    },

    RemoveAAllAddrsFromPoolMarket,

}

const MODULE_POOL_MARRKET : u8 = 33;


const MODULE_FUND_POOL : u8 = 1;


impl PoolInstruction {

    pub fn unpack(input : &[u8]) -> Result<Self, ProgramError> {

        // the first byte indicating the module, in this case
        // 1 is the FundPool
        let (module, rest) = input.split_first().ok_or(PoolError::InvalidModule)?;
        
        msg!("Current module being accessed is :{}", module);

        Ok(match module {

            &MODULE_FUND_POOL => Self::unpack_fund_pool(rest)?,

            &MODULE_POOL_MARRKET => Self::unpack_pool_market(rest)?,

            _ => return Err(PoolError::InvalidModule.into()),

        })

    }
}


const ACTION_CREATE : u8  = 1;

const ACTION_UPDATE : u8  = 2;


impl PoolInstruction{

    fn unpack_fund_pool(input : &[u8])-> Result<Self, ProgramError>{

        let (action,rest) = input.split_first().ok_or(PoolError::InvalidInstruction)?;

        msg!("Wallet's action is {}",action);
        
        Ok(match action  {

            &ACTION_CREATE => {

                const L : usize = 51; 

                let output = array_ref![rest, 0, L];

                let (manager,lamports, token_count,is_finalized, icon ) = 
                array_refs![output, PUBKEY_BYTES, 8,8,1, 2 ];
        
                Self::CreateFundPool{

                    manager : Pubkey::new_from_array(*manager),
                    lamports : u64::from_le_bytes(*lamports),
                    token_count : u64::from_le_bytes(*token_count),
                    is_finalized : unpack_bool(is_finalized).unwrap(),
                    icon : u16::from_le_bytes(*icon),
                    
                }

            },

            &ACTION_UPDATE => {

                let w = FundPool::unpack(rest).unwrap();

                Self::UpdateFundPool{ pool : w}
   
            }
            _ => return Err(PoolError::InvalidAction.into()),

        })
    }
}

const ACTION_REGISTER_ADDR : u8 = 3;

const ACTION_REMOVE_ADDR : u8 = 4;

const ACTION_REMOVE_ALL_ADDRS : u8 = 44;

impl PoolInstruction {


    fn unpack_pool_market(input : &[u8])-> Result<Self, ProgramError>{

        let (action,rest) = input.split_first().ok_or(PoolError::InvalidInstruction)?;
        
        msg!("PoolMarket's action is {}",action);
      
        Ok(match action  {

            &ACTION_CREATE => Self::CreatePoolMarket,

            &ACTION_REGISTER_ADDR => {
                Self::RegisterAddrInPoolMarket{ address : unpack_pub_key(rest) }   
            },

            &ACTION_REMOVE_ADDR => {
                Self::RemoveAddrFromPoolMarket{ address : unpack_pub_key(rest) }   
            },

            &ACTION_REMOVE_ALL_ADDRS => Self::RemoveAAllAddrsFromPoolMarket, 

            
            _ => return Err(PoolError::InvalidAction.into()),

        })

    }
}


fn unpack_pub_key(array : &[u8]) -> Pubkey{

    let mut a : [u8; 32] = [1; 32];
    a.copy_from_slice(array);
    return Pubkey::new_from_array(a);
}