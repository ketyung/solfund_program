//use crate::{error::PoolError};
//use crate::state::{PoolWallet};


#[derive(Clone, Debug, PartialEq)]
pub enum PoolInstruction {

    InitPoolWallet,

    UpdatePoolWallet,

    Transfer{
        lamports : u64, 
    },

}
