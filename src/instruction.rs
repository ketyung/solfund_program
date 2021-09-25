use crate::{error::PoolError};
use crate::state::{FundPool};

use solana_program::{
    program_error::ProgramError,
    msg, 
    program_pack::{Pack},
};

#[derive(Clone, Debug, PartialEq)]
pub enum PoolInstruction {

    CreateFundPool {

        wallet : FundPool, 
    },

    UpdateFundPool {

        wallet : FundPool,
    },

}


const MODULE_POOL_WALLET : u8 = 1;


impl PoolInstruction {

    pub fn unpack(input : &[u8]) -> Result<Self, ProgramError> {

        // the first byte indicating the module, in this case
        // 1 is the FundPool
        let (module, rest) = input.split_first().ok_or(PoolError::InvalidModule)?;
        
        msg!("Current module being accessed is :{}", module);

        Ok(match module {

            &MODULE_POOL_WALLET => Self::unpack_pool_wallet(rest)?,

            _ => return Err(PoolError::InvalidModule.into()),

        })

    }
}


const ACTION_CREATE : u8  = 1;

const ACTION_UPDATE : u8  = 2;


impl PoolInstruction{

    fn unpack_pool_wallet(input : &[u8])-> Result<Self, ProgramError>{

        let (action,rest) = input.split_first().ok_or(PoolError::InvalidInstruction)?;

        msg!("Wallet's action is {}",action);
        
        Ok(match action  {

            &ACTION_CREATE => {

                let w = FundPool::unpack(rest).unwrap();

                Self::CreateFundPool{ wallet : w}

            },

            &ACTION_UPDATE => {

                let w = FundPool::unpack(rest).unwrap();

                Self::UpdateFundPool{ wallet : w}
   
            }
            _ => return Err(PoolError::InvalidAction.into()),

        })
    }
}

