use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        program_error::ProgramError,
        program_pack::{Pack},
        //program::{invoke},
        // system_instruction,
       // instruction::{AccountMeta},
       // system_program,
    },
    
    crate::instruction::PoolInstruction, 
    crate::state::{FundPool,PoolMarket},
};


pub fn process_instruction(program_id: &Pubkey,accounts: &[AccountInfo], _instruction_data: &[u8],) 
-> ProgramResult {


   
    let instruction = PoolInstruction::unpack(_instruction_data)?;
    
    match instruction {

        PoolInstruction::CreateFundPool{manager, lamports, token_count, is_finalized} => {

            create_fund_pool(manager, lamports, token_count, is_finalized, program_id, accounts)
        },

        PoolInstruction::UpdateFundPool{pool} => {
            update_fund_pool(pool, program_id, accounts) 
        },

        PoolInstruction::CreatePoolMarket => {

            create_pool_market(program_id, accounts)
        },

        PoolInstruction::RegisterAddrInPoolMarket{address} => {

            register_addr_to_pool_market(address, program_id, accounts)
        },

        PoolInstruction::RemoveAddrFromPoolMarket{address} => {

            remove_addr_from_pool_market(address, program_id, accounts)
        },

        PoolInstruction::RemoveAAllAddrsFromPoolMarket => {

            remove_all_addrs_from_pool_market(program_id, accounts)
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


fn create_fund_pool(  manager : Pubkey, lamports : u64,
token_count : u64, is_finalized : bool,
program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {


    let account_info_iter = &mut accounts.iter();

    let account = next_account_info(account_info_iter)?;

    if is_account_program_owner(program_id, account).unwrap() {

        
        let mut w = FundPool::new(true);
        w.is_finalized = is_finalized;
        w.token_count = token_count;
        w.lamports = lamports;
        w.manager = manager;

        FundPool::pack(w, &mut account.data.borrow_mut())?;

        msg!("Created fund pool {:?}", w);   

    }
    Ok(())
}

fn update_fund_pool(pool : FundPool, program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {


    let account_info_iter = &mut accounts.iter();

    let account = next_account_info(account_info_iter)?;

    if is_account_program_owner(program_id, account).unwrap() {

        let mut w = FundPool::unpack_unchecked(&account.data.borrow())?;

        w.token_count = pool.token_count;
     
        FundPool::pack(w, &mut account.data.borrow_mut())?;
    }
    Ok(())
}


fn create_pool_market(program_id: &Pubkey,accounts: &[AccountInfo])  -> ProgramResult{

    let account_info_iter = &mut accounts.iter();

    let account = next_account_info(account_info_iter)?;

    if is_account_program_owner(program_id, account).unwrap() {

        let stored_pool_market = PoolMarket::unpack_unchecked(&account.data.borrow())?;

        if stored_pool_market.pool_size > 0 {

            msg!("PoolMarket already exists ::{:?}",stored_pool_market);

        }
        else {

            let pool_market = PoolMarket::new();
        
            msg!("Creating pool_market::{:?}", pool_market);
    
            PoolMarket::pack(pool_market, &mut account.data.borrow_mut())?;
    
        }

 
    }

    Ok(())

}

fn register_addr_to_pool_market(address : Pubkey, program_id: &Pubkey,accounts: &[AccountInfo])  -> ProgramResult{

    let account_info_iter = &mut accounts.iter();

    let account = next_account_info(account_info_iter)?;

    if is_account_program_owner(program_id, account).unwrap() {

        let mut pool_market = PoolMarket::unpack_unchecked(&account.data.borrow())?;


        msg!("Unpack poolmaket::{:?}", pool_market);

        pool_market.add_fund_pool(address);

        PoolMarket::pack(pool_market, &mut account.data.borrow_mut())?;

    }

    Ok(())

}


fn remove_addr_from_pool_market(address : Pubkey, program_id: &Pubkey,accounts: &[AccountInfo])  -> ProgramResult{

    let account_info_iter = &mut accounts.iter();

    let account = next_account_info(account_info_iter)?;

    if is_account_program_owner(program_id, account).unwrap() {

        let mut pool_market = PoolMarket::unpack_unchecked(&account.data.borrow())?;

        pool_market.remove_fund_pool(address);

        PoolMarket::pack(pool_market, &mut account.data.borrow_mut())?;

    }

    Ok(())

}

fn remove_all_addrs_from_pool_market(program_id: &Pubkey,accounts: &[AccountInfo])  -> ProgramResult{

    let account_info_iter = &mut accounts.iter();

    let account = next_account_info(account_info_iter)?;


    if is_account_program_owner(program_id, account).unwrap() {

        // when deleting set all its data to zeros

        let zeros = &vec![0; account.data_len()];

        account.data.borrow_mut()[0..zeros.len()].copy_from_slice(zeros);


    }

    Ok(())
}