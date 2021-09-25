use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        program_error::ProgramError,
        //program::{invoke},
        // system_instruction,
       // instruction::{AccountMeta},
       // system_program,
    },
    
    crate::instruction::PoolInstruction, 
    crate::state::{PoolWallet},
};


pub fn process_instruction(program_id: &Pubkey,accounts: &[AccountInfo], _instruction_data: &[u8],) 
-> ProgramResult {


   
    let instruction = PoolInstruction::unpack(_instruction_data)?;
    
    match instruction {

        PoolInstruction::CreatePoolWallet{wallet} => {

            create_pool_wallet(wallet, program_id, accounts)
        }

        PoolInstruction::UpdatePoolWallet{wallet} => {
           // update_pool_wallet(wallet, program_id, accounts) 
           msg!("Going to update!:{:?}", wallet);
           Ok(())
        }
    }

    
}


fn is_account_program_owner(program_id : &Pubkey, account : &AccountInfo) -> Result<bool, ProgramError>{

    msg!("Checking acc is owner, {:?}, {:?}", account.owner, program_id);

    if account.owner != program_id {

        msg!("Account is not owner of program!");
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(true)

}


fn create_pool_wallet(wallet : PoolWallet, program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {


    let account_info_iter = &mut accounts.iter();

    let account = next_account_info(account_info_iter)?;

    if is_account_program_owner(program_id, account).unwrap() {

        msg!("Proceed!");   
    }
    Ok(())


}