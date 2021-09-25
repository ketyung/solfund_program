use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
       // program_error::ProgramError,
        //program::{invoke},
        // system_instruction,
       // instruction::{AccountMeta},
       // system_program,
    },
    
    //crate::instruction::PoolInstruction, 
    //crate::state::{PoolWallet},
};


pub fn process_instruction(program_id: &Pubkey,accounts: &[AccountInfo], _instruction_data: &[u8],) -> ProgramResult {


    let account_info_iter = &mut accounts.iter();

    let acc = next_account_info(account_info_iter)?;

    msg!("Will do later, {:?}, {:?}", acc, program_id);

    Ok(())
    
}

