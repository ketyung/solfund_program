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
    crate::state::{FundPool,PoolMarket, UserPool},
    crate::{error::PoolError},

};


pub fn process_instruction(program_id: &Pubkey,accounts: &[AccountInfo], _instruction_data: &[u8],) 
-> ProgramResult {


   
    let instruction = PoolInstruction::unpack(_instruction_data)?;
    
    match instruction {

        PoolInstruction::CreateFundPool{manager, address, lamports, token_count, is_finalized, icon} => {

            create_fund_pool(manager, address, lamports, token_count, is_finalized, icon, program_id, accounts)
        },

        PoolInstruction::UpdateFundPool{manager, address, lamports, token_count, is_finalized, icon} => {
            update_fund_pool(manager, address, lamports, token_count, is_finalized, icon, program_id, accounts) 
        },

        PoolInstruction::DeleteFundPool => {

            delete_fund_pool(program_id, accounts)

        },
       
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


fn fund_pool_exists(fund_pool_account : &AccountInfo) -> Result<bool, PoolError> {

    let stored_fund_pool = FundPool::unpack_unchecked(&fund_pool_account.data.borrow());

    match stored_fund_pool{

        Ok(s) => {

            if s.is_initialized {

                return Err(PoolError::ObjectAlreadyCreated);
            }
        
        },

        Err(e) => {

            msg!("Failed to unpack!!! error is ::{:?}", e);
            return Ok(false)

        } 
        
    }
             
    return Ok(false) ;
}


fn create_fund_pool(  manager : Pubkey,
    address : Pubkey, lamports : u64,token_count : u64, is_finalized : bool,
    icon : u16, program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {


    let account_info_iter = &mut accounts.iter();

    let fund_pool_account = next_account_info(account_info_iter)?;
    let user_pool_account = next_account_info(account_info_iter)?;
    let market_pool_account = next_account_info(account_info_iter)?;
    let signer_account = next_account_info(account_info_iter)?;


    // check for signer
    if !signer_account.is_signer {

        return Err(ProgramError::MissingRequiredSignature);
    }


    if is_account_program_owner(program_id, fund_pool_account).unwrap() {

       
        if !fund_pool_exists(fund_pool_account).unwrap() {
        
            let mut w = FundPool::new(true);
            w.is_finalized = is_finalized;
            w.token_count = token_count;
            w.lamports = lamports;
            w.manager = manager;
            w.icon = icon ; 
            w.address = address;
    
            FundPool::pack(w, &mut fund_pool_account.data.borrow_mut())?;

         
            if user_pool_account.owner == program_id  {

                register_address_to_user_pool(address, manager, user_pool_account)
            }
        
           
            if market_pool_account.owner == program_id /* && is_finalized */ {

                register_address_to_market_pool(address, market_pool_account)
            }
        
        }
    
    }
    Ok(())
}

fn update_fund_pool(manager : Pubkey,
    address : Pubkey, lamports : u64,token_count : u64, is_finalized : bool,
    icon : u16, program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();

    let account = next_account_info(account_info_iter)?;

    if is_account_program_owner(program_id, account).unwrap() {

        let mut w = FundPool::unpack_unchecked(&account.data.borrow())?;

        if w.manager == manager && w.address == address {
            w.token_count = token_count;
            w.is_finalized = is_finalized;
            w.lamports = lamports;
            w.icon = icon;
            FundPool::pack(w, &mut account.data.borrow_mut())?;
        }
        else {

            msg!("No update, different manager, can't change manager!!");
        }
       
    }
    Ok(())
}

fn delete_fund_pool(program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {


    msg!("Deleting fund pool...");
    
    let account_info_iter = &mut accounts.iter();

    let account = next_account_info(account_info_iter)?;
    let user_pool_account = next_account_info(account_info_iter)?;
    let market_pool_account = next_account_info(account_info_iter)?;
    let signer_account = next_account_info(account_info_iter)?;


    // check for signer
    if !signer_account.is_signer {

        return Err(ProgramError::MissingRequiredSignature);
    }


    if is_account_program_owner(program_id, account).unwrap() {


        let fund_pool = FundPool::unpack_unchecked(&account.data.borrow())?;

    
        let zeros = &vec![0; account.data_len()];

        account.data.borrow_mut()[0..zeros.len()].copy_from_slice(zeros);

       
        if user_pool_account.owner == program_id  {

            remove_address_from_user_pool(fund_pool.address, fund_pool.manager, user_pool_account)
        }
        
       
        if market_pool_account.owner == program_id  {

            remove_address_from_pool_market(fund_pool.address, market_pool_account)
        }
      

    }
    Ok(())
}



fn register_address_to_market_pool(address : Pubkey, market_pool_account : &AccountInfo) {


    let market_pool = PoolMarket::unpack_unchecked(&market_pool_account.data.borrow());

    match market_pool{

        Ok(mut pool) => {

            //msg!("MarketPool.Registering address::...current:{:?}", pool);
            pool.add_fund_pool(address);
            
            let _ = PoolMarket::pack(pool, &mut market_pool_account.data.borrow_mut());

        },

        Err(_) => {

            msg!("Failed to unpack pool market, create .default !");

            let pool = PoolMarket::new();
                     
            let _ = PoolMarket::pack(pool, &mut market_pool_account.data.borrow_mut());

        }

    }

}


fn remove_address_from_pool_market(address : Pubkey, market_pool_account : &AccountInfo)  {

 
    let market_pool = PoolMarket::unpack_unchecked(&market_pool_account.data.borrow());

    match market_pool{

        Ok(mut pool) => {

            msg!("Going to remove addr from pool..market :{:?}, size::{}", address, pool.len());
                
            // Print the pool causing exceeding compute cycle, so don't do it!!
           // msg!("Removing address from pool market::...current: {:?}", pool);
            pool.remove_fund_pool(address);
            
            let _ = PoolMarket::pack(pool, &mut market_pool_account.data.borrow_mut());

        },

        Err(_) => {

            msg!("Failed to unpack pool market !");
        }

    }


}


fn register_address_to_user_pool(address : Pubkey, user : Pubkey, user_pool_account : &AccountInfo) {


    let stored_pool = UserPool::unpack_unchecked(&user_pool_account.data.borrow());

    match stored_pool{

        Ok(mut pool) => {

          //  msg!("UserPool.Registering address::... current:{:?}", pool);
            

            if pool.user == user || pool.user == Pubkey::default()   {

              
                pool.user = user;
                pool.add_address(address);
                // Ignore the error  

                let s = UserPool::pack(pool, &mut user_pool_account.data.borrow_mut());

                match s {

                    Ok(p) => {
                        msg!("Successfully packed m.pool:{:?}", p );
                    },
                    Err(e) => {
                        msg!("Error.packing manager pool :{:?}",e);
                    }
                }

            }
   
          
        },

        Err(e) => {

            msg!("Failed to unpack user_pool, create .default !, {:?}", e);

            let mut pool = UserPool::new();
           
            pool.user = user;

            let _ = UserPool::pack(pool, &mut user_pool_account.data.borrow_mut());

        }

    }
 
}


fn remove_address_from_user_pool(address : Pubkey, user : Pubkey, user_pool_account : &AccountInfo) {


    let stored_pool = UserPool::unpack_unchecked(&user_pool_account.data.borrow());

    match stored_pool{

        Ok(mut pool) => {

            
            msg!("Going to remove addr from user_pool :{:?}, len::{}", address, pool.len());

            if pool.user == user{

                 
                pool.remove_address(address);
          
                let s = UserPool::pack(pool, &mut user_pool_account.data.borrow_mut());

                match s {

                    Ok(p) => {
                        msg!("Successfully packed m.pool:{:?}", p );
                    },
                    Err(e) => {
                        msg!("Error.packing manager pool :{:?}",e);
                    }
                }
            }
   
          
        },

        Err(_) => {

            msg!("Failed to unpack user_pool, create .default !");
        }

    }
 
}

/*
fn increment_counter(counter_account : &AccountInfo) {

   
    let stored_counter = Counter::unpack_unchecked(&counter_account.data.borrow());

        
    match stored_counter{

        Ok(mut c) => {

            msg!("Increment counter!!, curr.count::{}",c.count);
            c.increment();
   
            // Ignore the error  

            let _ = Counter::pack(c, &mut counter_account.data.borrow_mut());

        },

        Err(_) => {

            msg!("Failed to unpack counter, create .default !");

            let mut c = Counter::new();
            c.increment();
   
            // Ignore the error  
            let _ = Counter::pack(c, &mut counter_account.data.borrow_mut());

        }

    }
   

}*/