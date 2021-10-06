/**
 *  CopyRight @ Christopher K Y Chee (ketyung@techchee.com)
 */

use crate::{error::PoolError};
use crate::state::{unpack_bool}; 

use solana_program::{
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    clock::{Clock},
    sysvar::Sysvar, 
  
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

        token_to_lamport_ratio : u64, 

        is_finalized : bool,

        icon : u16, 

    },

    UpdateFundPool {

        manager : Pubkey,

        address : Pubkey, 

        token_address : Pubkey, 
        
        lamports : u64,

        token_count : u64, 

        token_to_lamport_ratio : u64, 

        is_finalized : bool,

        icon : u16, 

    },


    DeleteFundPool ,


    AddInvestor {
        investor : Pubkey, 

        pool_address : Pubkey, 
    
        address : Pubkey,
    
        amount : u64, 
    
        token_address : Pubkey,
    
        token_count : u64,
      
        date : i64, 

    },

    CreateMarket{

        creator : Pubkey, 
    },

    RegisterToMarket{

        fund_pool : Pubkey, 
    },

    DeleteFromMarket{

        fund_pool : Pubkey, 
    }
}


const MODULE_FUND_POOL : u8 = 1;

const MODULE_INVESTOR : u8 = 2;

const MODULE_MARKET : u8 = 3;


impl PoolInstruction {

    pub fn unpack(input : &[u8]) -> Result<Self, ProgramError> {

        // the first byte indicating the module, in this case
        // 1 is the FundPool
        let (module, rest) = input.split_first().ok_or(PoolError::InvalidModule)?;
        
       // msg!("Current module being accessed is :{}", module);

        Ok(match module {

            &MODULE_FUND_POOL => Self::unpack_fund_pool(rest)?,

            &MODULE_INVESTOR => Self::unpack_investor(rest)?,
         
            &MODULE_MARKET => Self::unpack_market(rest)?,
           
            _ => return Err(PoolError::InvalidModule.into()),

        })

    }
}


const ACTION_CREATE : u8  = 1;

const ACTION_UPDATE : u8  = 2;

const ACTION_REGISTER : u8 = 3;

const ACTION_DELETE : u8 = 44;

impl PoolInstruction {

    fn unpack_market(input : &[u8])-> Result<Self, ProgramError>{

        let (action,rest) = input.split_first().ok_or(PoolError::InvalidInstruction)?;

        Ok(match action  {

            &ACTION_CREATE => {

                let output = array_ref![rest, 0, PUBKEY_BYTES];
                let (creator,_) = array_refs![output, PUBKEY_BYTES, 0 ];
  
                Self::CreateMarket {
                    creator : unpack_pub_key(creator),
                }
            },

            &ACTION_REGISTER => {

                let output = array_ref![rest, 0, PUBKEY_BYTES];
                let (address,_) = array_refs![output, PUBKEY_BYTES, 0 ];
  
                Self::RegisterToMarket {
                    fund_pool : unpack_pub_key(address),
                }

            },


            &ACTION_DELETE => {

                let output = array_ref![rest, 0, PUBKEY_BYTES];
                let (address,_) = array_refs![output, PUBKEY_BYTES, 0 ];
  
                Self::DeleteFromMarket {
                    fund_pool : unpack_pub_key(address),
                }

            },

            _ => return Err(PoolError::InvalidAction.into()),

        })
    
    }
}

impl PoolInstruction{

    fn unpack_fund_pool(input : &[u8])-> Result<Self, ProgramError>{

        let (action,rest) = input.split_first().ok_or(PoolError::InvalidInstruction)?;

        
        Ok(match action  {

            &ACTION_CREATE => {

                let (manager,address,token_address, lamports, token_count, token_to_lamport_ratio, is_finalized, icon ) = 
                unpack_fund_pool_data(&rest);

                Self::CreateFundPool{

                    manager : manager,
                    address : address,
                    token_address : token_address, 
                    lamports : lamports,
                    token_count : token_count,
                    token_to_lamport_ratio : token_to_lamport_ratio, 
                    is_finalized : is_finalized,
                    icon : icon,
                    
                }

            },

            &ACTION_UPDATE => {

                let (manager,address,token_address, lamports, token_count, token_to_lamport_ratio, is_finalized, icon ) = 
                unpack_fund_pool_data(&rest);

                Self::UpdateFundPool{ 

                    manager : manager,
                    address : address,
                    token_address : token_address, 
                    lamports : lamports,
                    token_count : token_count,
                    token_to_lamport_ratio : token_to_lamport_ratio, 
                    is_finalized : is_finalized,
                    icon : icon,

                }
   
            },

            &ACTION_DELETE => Self::DeleteFundPool,
            
            _ => return Err(PoolError::InvalidAction.into()),

        })
    }
}

impl PoolInstruction {

    fn unpack_investor(input : &[u8])-> Result<Self, ProgramError>{

        let (action,rest) = input.split_first().ok_or(PoolError::InvalidInstruction)?;

        Ok(match action  {

            &ACTION_CREATE => {

                let (
                    investor, 
                    pool_address, 
                    address,
                    token_address,
                    amount, 
                    token_count,
                    date, 
                ) =  unpack_investor_data(&rest);

                Self::AddInvestor{

                    investor : investor, 
                    pool_address :pool_address, 
                    address : address,
                    token_address : token_address,
                    amount : amount, 
                    token_count : token_count,
                    date : date , 
                }

            },

            _ => return Err(PoolError::InvalidAction.into()),

        })
    }

}


fn unpack_investor_data(input : &[u8]) -> (Pubkey, Pubkey, Pubkey, Pubkey, u64, u64, i64){

    const L : usize = 144; 
    let output = array_ref![input, 0, L];
    let (
        investor, 
        pool_address, 
        address,
        token_address,
        amount, 
        token_count,
    ) = 
    array_refs![output, PUBKEY_BYTES, PUBKEY_BYTES, PUBKEY_BYTES,PUBKEY_BYTES,8,8 ];

    let currtime =  Clock::get().unwrap().unix_timestamp;

    (Pubkey::new_from_array(*investor),
    Pubkey::new_from_array(*pool_address),
    Pubkey::new_from_array(*address),
    Pubkey::new_from_array(*token_address),
    u64::from_le_bytes(*amount),
    u64::from_le_bytes(*token_count),currtime)


}


// [u8;32], [u8;32],[u8;32], [u8;8], [u8;8] ,[u8;8] , [u8;1], [u8;2] 
fn unpack_fund_pool_data(input : &[u8]) -> (Pubkey, Pubkey, Pubkey, u64, u64, u64,  bool, u16){

    const L : usize = 123; 
    let output = array_ref![input, 0, L];
    let (manager,address, token_address, lamports,token_count,token_to_lamport_ratio, is_finalized,icon) = 
    array_refs![output, PUBKEY_BYTES, PUBKEY_BYTES, PUBKEY_BYTES,8,8,8, 1, 2 ];

    (  Pubkey::new_from_array(*manager),
    Pubkey::new_from_array(*address),
    Pubkey::new_from_array(*token_address),
    u64::from_le_bytes(*lamports),
    u64::from_le_bytes(*token_count),
    u64::from_le_bytes(*token_to_lamport_ratio),
    unpack_bool(is_finalized).unwrap(),
    u16::from_le_bytes(*icon))
}



fn unpack_pub_key(array : &[u8]) -> Pubkey{

    let mut a : [u8; 32] = [1; 32];
    a.copy_from_slice(array);
    return Pubkey::new_from_array(a);
}