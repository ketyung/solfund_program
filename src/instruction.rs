use crate::{error::PoolError};
use crate::state::{PoolWallet};

use solana_program::{
    program_error::ProgramError,
    msg, 
    program_pack::{Pack},
};

#[derive(Clone, Debug, PartialEq)]
pub enum PoolInstruction {

    CreatePoolWallet {

        wallet : PoolWallet, 
    },

    UpdatePoolWallet {

        wallet : PoolWallet,
    },

}

impl PoolInstruction {

    pub fn unpack(input : &[u8]) -> Result<Self, ProgramError> {

        // the first byte indicating the module, in this case
        // 1 is the PoolWallet
        let (module, rest) = input.split_first().ok_or(PoolError::InvalidModule)?;
        
        msg!("Current module being accessed is :{}", module);

        Ok(match module {

            1 => Self::unpack_pool_wallet(rest)?,

            _ => return Err(PoolError::InvalidModule.into()),

        })

    }
}


impl PoolInstruction{

    fn unpack_pool_wallet(input : &[u8])-> Result<Self, ProgramError>{

        let (action,rest) = input.split_first().ok_or(PoolError::InvalidInstruction)?;

        msg!("Wallet's action is {}",action);
        
        Ok(match action  {

            1 => {

                let w = PoolWallet::unpack(rest).unwrap();

                Self::CreatePoolWallet{ wallet : w}

            },
            2 => {

                let w = PoolWallet::unpack(rest).unwrap();

                Self::UpdatePoolWallet{ wallet : w}
   
            }
            _ => return Err(PoolError::InvalidAction.into()),

        })
    }
}
