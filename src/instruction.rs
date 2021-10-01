/**
 *  CopyRight @ Christopher K Y Chee (ketyung@techchee.com)
 */

use crate::{error::PoolError};
use crate::state::{unpack_bool}; 

use solana_program::{
    program_error::ProgramError,
    msg, pubkey::{Pubkey, PUBKEY_BYTES},
};
use arrayref::{array_ref,  array_refs};



#[derive(Clone, Debug, PartialEq)]
pub enum PoolInstruction {

    CreateFundPool {

        manager : Pubkey,

        address : Pubkey, 

        token_address : Pubkey, 

        lamports : u64,

        token_count : u64, 

        is_finalized : bool,

        icon : u16, 

    },

    UpdateFundPool {

        manager : Pubkey,

        address : Pubkey, 

        token_address : Pubkey, 
        
        lamports : u64,

        token_count : u64, 

        is_finalized : bool,

        icon : u16, 

    },


    DeleteFundPool ,

   
}


const MODULE_FUND_POOL : u8 = 1;


impl PoolInstruction {

    pub fn unpack(input : &[u8]) -> Result<Self, ProgramError> {

        // the first byte indicating the module, in this case
        // 1 is the FundPool
        let (module, rest) = input.split_first().ok_or(PoolError::InvalidModule)?;
        
        msg!("Current module being accessed is :{}", module);

        Ok(match module {

            &MODULE_FUND_POOL => Self::unpack_fund_pool(rest)?,

            _ => return Err(PoolError::InvalidModule.into()),

        })

    }
}


const ACTION_CREATE : u8  = 1;

const ACTION_UPDATE : u8  = 2;

const ACTION_DELETE : u8 = 44;

impl PoolInstruction{

    fn unpack_fund_pool(input : &[u8])-> Result<Self, ProgramError>{

        let (action,rest) = input.split_first().ok_or(PoolError::InvalidInstruction)?;

        
        Ok(match action  {

            &ACTION_CREATE => {

                let (manager,address,token_address, lamports, token_count,is_finalized, icon ) = 
                unpack_fund_pool_first_115(&rest);

                Self::CreateFundPool{

                    manager : manager,
                    address : address,
                    token_address : token_address, 
                    lamports : lamports,
                    token_count : token_count,
                    is_finalized : is_finalized,
                    icon : icon,
                    
                }

            },

            &ACTION_UPDATE => {

                let (manager, address,token_address, lamports, token_count,is_finalized, icon ) = 
                unpack_fund_pool_first_115(&rest);

                Self::UpdateFundPool{ 

                    manager : manager,
                    address : address,
                    token_address : token_address, 
                    lamports : lamports,
                    token_count : token_count,
                    is_finalized : is_finalized,
                    icon : icon,

                }
   
            },

            &ACTION_DELETE => Self::DeleteFundPool,
            
            _ => return Err(PoolError::InvalidAction.into()),

        })
    }
}
// [u8;32], [u8;32],[u8;32], [u8;8], [u8;8] , [u8;1], [u8;2] 
fn unpack_fund_pool_first_115(input : &[u8]) -> (Pubkey, Pubkey, Pubkey, u64, u64, bool, u16){

    const L : usize = 115; 
    let output = array_ref![input, 0, L];
    let (manager,address, token_address,  lamports,token_count,is_finalized,icon) = 
    array_refs![output, PUBKEY_BYTES, PUBKEY_BYTES, PUBKEY_BYTES,8,8,1, 2 ];

    (  Pubkey::new_from_array(*manager),
    Pubkey::new_from_array(*address),
    Pubkey::new_from_array(*token_address),
    u64::from_le_bytes(*lamports),
    u64::from_le_bytes(*token_count),
    unpack_bool(is_finalized).unwrap(),
    u16::from_le_bytes(*icon))
}



/*
// maybe needed later
fn unpack_pub_key(array : &[u8]) -> Pubkey{

    let mut a : [u8; 32] = [1; 32];
    a.copy_from_slice(array);
    return Pubkey::new_from_array(a);
}*/