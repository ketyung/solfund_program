/**
 *  CopyRight @ Christopher K Y Chee (ketyung@techchee.com)
 */
use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        program_error::ProgramError,
        program_pack::{Pack},
        system_instruction,
        program::{invoke},
        // system_instruction,
       // instruction::{AccountMeta},
       // system_program,
    },
    
    crate::instruction::PoolInstruction, 
    crate::state::{FundPool,Market, UserPool, Investor},
    crate::{error::PoolError},
    //spl_token::instruction::initialize_account;
    spl_token::instruction::{initialize_mint},//,mint_to} 
    //spl_token ::{ initialize_mint }

};


pub fn process_instruction(program_id: &Pubkey,accounts: &[AccountInfo], _instruction_data: &[u8],) 
-> ProgramResult {


   
    let instruction = PoolInstruction::unpack(_instruction_data)?;
    
    match instruction {

        PoolInstruction::CreateFundPool{manager, address, token_address, lamports, token_count, token_to_lamport_ratio, is_finalized, icon} => {

            create_fund_pool(manager, address, token_address, lamports, token_count,  token_to_lamport_ratio,  is_finalized, icon, program_id, accounts)
        },

        PoolInstruction::UpdateFundPool{manager, address, token_address,  lamports, token_count, token_to_lamport_ratio, is_finalized, icon} => {
            update_fund_pool(manager, address,token_address, lamports, token_count, token_to_lamport_ratio,  is_finalized, icon, program_id, accounts) 
        },

        PoolInstruction::DeleteFundPool => {

            delete_fund_pool(program_id, accounts)

        },
       
        PoolInstruction::AddInvestor{
            investor, 
            pool_address, 
            address,
            token_address,
            amount, 
            token_count,
            date, 
      
        } => {
            add_investor(investor, pool_address, address, token_address,amount, 
                 token_count, date, program_id, accounts)

        },

        PoolInstruction::CreateMarket {creator} => {

            create_market(creator, program_id, accounts)
        },


        PoolInstruction::RegisterToMarket {fund_pool} => {

            register_to_market(fund_pool, program_id, accounts)

        },

        PoolInstruction::DeleteFromMarket {fund_pool} => {

            delete_from_market(fund_pool, program_id, accounts)

        },

       
    }

}


fn is_account_program_owner(program_id : &Pubkey, account : &AccountInfo) -> Result<bool, ProgramError>{

   // msg!("Checking acc is owner, {:?}, {:?}", account.owner, program_id);

    if account.owner != program_id {

        msg!("Account is not owner of program!");
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(true)

}

fn register_to_market( address : Pubkey, program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();
    let market_account = next_account_info(account_info_iter)?;
    let signer_account = next_account_info(account_info_iter)?;


    if is_account_program_owner(program_id, market_account).unwrap() {

        let stored_market = Market::unpack_unchecked(&market_account.data.borrow());
    
        match stored_market{
    
            Ok(mut s) => {
    
                if s.creator != *signer_account.key {
    
                    return Err(ProgramError::from( PoolError::UnmatchedCreator) );           
                }
    
                s.add_fund_pool(address);

                Market::pack(s, &mut market_account.data.borrow_mut())?;
          
            },
    
            Err(e) => {
    
                msg!("No market::error:{:?}",e);
            } 
            
        }
       
    }
              
    Ok(())

}


fn delete_from_market( address : Pubkey, program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();
    let market_account = next_account_info(account_info_iter)?;
    let signer_account = next_account_info(account_info_iter)?;


    if is_account_program_owner(program_id, market_account).unwrap() {

        let stored_market = Market::unpack_unchecked(&market_account.data.borrow());
    
        match stored_market{
    
            Ok(mut s) => {
    
                if s.creator != *signer_account.key {
    
                    return Err(ProgramError::from( PoolError::UnmatchedCreator) );           
                }
    
                s.remove_fund_pool(address);
    
                Market::pack(s, &mut market_account.data.borrow_mut())?;
          
            },
    
            Err(e) => {
    
                msg!("No market::error:{:?}",e);
            } 
            
        }
       
    }
              
    Ok(())

}



fn create_market(  creator : Pubkey, program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();
    let market_account = next_account_info(account_info_iter)?;
    
    if is_account_program_owner(program_id, market_account).unwrap() {

        let stored_market = Market::unpack_unchecked(&market_account.data.borrow());
   
        match stored_market{

            Ok(s) => {
    
                if s.creator != Pubkey::default() {
    
                    return Err(ProgramError::from( PoolError::ObjectAlreadyCreated) );
                }
            
                let mut market = Market::new();
                market.creator = creator;
                Market::pack(market, &mut market_account.data.borrow_mut())?;
    
            },
    
            Err(_) => {
    
                let mut market = Market::new();
                market.creator = creator;
                Market::pack(market, &mut market_account.data.borrow_mut())?;
            } 
            
        }
    
    }
    Ok(())
             
   
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
    address : Pubkey, token_address : Pubkey,
    lamports : u64,token_count : u64, 
    token_to_lamport_ratio : u64, 
    is_finalized : bool,
    icon : u16, program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {


    let account_info_iter = &mut accounts.iter();

    let fund_pool_account = next_account_info(account_info_iter)?;
    let user_pool_account = next_account_info(account_info_iter)?;
    let market_account = next_account_info(account_info_iter)?;
    let signer_account = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?; // expecting the last acc is the token acc

    // check for signer
    if !signer_account.is_signer {

        return Err(ProgramError::MissingRequiredSignature);
    }


    if is_account_program_owner(program_id, fund_pool_account).unwrap() {

       
        if !fund_pool_exists(fund_pool_account).unwrap() {
        
            let mut w = FundPool::new(true);
            w.is_finalized = is_finalized;
            w.token_count = token_count;
            w.rm_token_count = token_count;
            w.token_to_lamport_ratio = token_to_lamport_ratio; 
            w.lamports = lamports;
            w.manager = manager;
            w.icon = icon ; 
            w.address = address;
            w.token_address = token_address;
    
            FundPool::pack(w, &mut fund_pool_account.data.borrow_mut())?;

            if *token_account.owner == spl_token::id() {
                mint_token(signer_account, token_account, token_count, token_address);
            }
        

            if user_pool_account.owner == program_id  {

                register_address_to_user_pool(address, manager, user_pool_account)
            }
        
           
            if market_account.owner == program_id && is_finalized  {

                register_address_to_market(address, market_account)
            }

          
        }
    
    }
    Ok(())
}

fn mint_token (signer_account : &AccountInfo, token_account : &AccountInfo, 
    token_count : u64, token_address : Pubkey) {

    msg!("Going to mint {} tokens by {:?}, address: {:?}", token_count, token_account.key, token_address );

    let _init_mint_ins = initialize_mint(
        &spl_token::ID,
        &token_account.key,
        &signer_account.key,
        Some(signer_account.key),
        3,
    )
    .unwrap();

    //let _mint_to = mint_to(token_program_id: &spl_token::ID, 
    //    mint_pubkey: &Pubkey, account_pubkey: &Pubkey, 
    //    owner_pubkey: &Pubkey, signer_pubkeys: 
    //    &[signer_account.key],   )
    //.unwrap(); 

}

fn update_fund_pool(manager : Pubkey,
    address : Pubkey, _token_address : Pubkey, 
    lamports : u64,token_count : u64, 
    token_to_lamport_ratio : u64, 
    is_finalized : bool,
    icon : u16, program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();

    let account = next_account_info(account_info_iter)?;

    if is_account_program_owner(program_id, account).unwrap() {

        let mut w = FundPool::unpack_unchecked(&account.data.borrow())?;

        if w.manager == manager && w.address == address {
            w.token_count = token_count;
            w.token_to_lamport_ratio = token_to_lamport_ratio; 
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
    let market_account = next_account_info(account_info_iter)?;
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
        
       
        if market_account.owner == program_id  {

            remove_address_from_market(fund_pool.address, market_account)
        }
      

    }
    Ok(())
}



fn register_address_to_market(address : Pubkey, market_account : &AccountInfo) {


    let market = Market::unpack_unchecked(&market_account.data.borrow());

    match market{

        Ok(mut pool) => {

            //msg!("MarketPool.Registering address::...current:{:?}", pool);
            pool.add_fund_pool(address);
            
            let _ = Market::pack(pool, &mut market_account.data.borrow_mut());

        },

        Err(_) => {

            msg!("Failed to unpack pool market, create .default !");

            let pool = Market::new();
                     
            let _ = Market::pack(pool, &mut market_account.data.borrow_mut());

        }

    }

}


fn remove_address_from_market(address : Pubkey, market_account : &AccountInfo)  {

 
    let market = Market::unpack_unchecked(&market_account.data.borrow());

    match market{

        Ok(mut pool) => {

            msg!("Going to remove addr from pool..market :{:?}, size::{}", address, pool.len());
                
            pool.remove_fund_pool(address);
            
            let _ = Market::pack(pool, &mut market_account.data.borrow_mut());

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

        
            if pool.user == user || pool.user == Pubkey::default()   {

              
                pool.user = user;
                pool.add_address(address);
        
                let _ = UserPool::pack(pool, &mut user_pool_account.data.borrow_mut());

                /*
                match s {

                    Ok(p) => {
                        msg!("Successfully packed m.pool:{:?}", p );
                    },
                    Err(e) => {
                        msg!("Error.packing manager pool :{:?}",e);
                    }
                }*/

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
          
                let _ = UserPool::pack(pool, &mut user_pool_account.data.borrow_mut());

                /*
                match s {

                    Ok(p) => {
                        msg!("Successfully packed m.pool:{:?}", p );
                    },
                    Err(e) => {
                        msg!("Error.packing manager pool :{:?}",e);
                    }
                }*/

            }
   
          
        },

        Err(_) => {

            msg!("Failed to unpack user_pool, create .default !");
        }

    }
 
}

fn add_investor(investor : Pubkey,
    pool_address : Pubkey,
    address : Pubkey, 
    token_address : Pubkey, 
    amount : u64,token_count : u64, date : i64,
    program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult{

    let account_info_iter = &mut accounts.iter();

    let investor_account = next_account_info(account_info_iter)?;
    let investor_pool_account = next_account_info(account_info_iter)?;
    let fund_pool_account = next_account_info(account_info_iter)?;
    let signer_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
       

     // check for signer
    if !signer_account.is_signer {

        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // check each account if belongs program 

    if investor_account.owner != program_id {

        return Err(ProgramError::IncorrectProgramId);
    }

    if investor_pool_account.owner != program_id {

        return Err(ProgramError::IncorrectProgramId);
    }

    if fund_pool_account.owner != program_id {

        return Err(ProgramError::IncorrectProgramId);   
    }

    
    let mut fp = FundPool::unpack_unchecked(&fund_pool_account.data.borrow())?;

    if fp.address != pool_address{

        return Err( ProgramError::from( PoolError::UnmatchedPoolAddress) );
    }

    //msg!("fp.address:{:?}", fp.address);

    if *investor_account.key != address {

        return Err( ProgramError::from( PoolError::UnmatchedInvestorAccountAddress) );

    }


    let mut i = Investor::new();
    i.investor = investor;
    i.amount = amount;
    i.date = date;
    i.token_count = token_count;
    i.address = address;
    i.pool_address = pool_address;
    i.token_address = token_address;

   // msg!("i.investor::{:?}, i.address:{:?}", i.investor, i.address);
   
 //   msg!("fp.rm_token_count::{}",fp.rm_token_count);

/*
The lamports field might need to be ommited, temporary keep it here first
ketyung@techchee.com 
*/
//    let fpa = fund_pool_account.clone();
//    let lamports: & u64 = & fpa.lamports.borrow();
//    fp.lamports = *lamports;


    let token_to_lamport_ratio = fp.token_to_lamport_ratio;
    let amount_in_lamports = token_to_lamport_ratio * token_count;

    //  msg!("Amount to tx in SOL ::{}", 
    //  amount_in_lamports / solana_program::native_token::LAMPORTS_PER_SOL);


    invoke(
        &system_instruction::transfer(signer_account.key, &fund_pool_account.key, amount_in_lamports),
        &[
            signer_account.clone(),
            fund_pool_account.clone(),
            system_program.clone(),
        ],
    )?;

    
    let inv = i.clone();

    let _ = Investor::pack(i, &mut investor_account.data.borrow_mut());

    fp.rm_token_count = fp.rm_token_count - token_count;
    let _ = fp.register_investor(inv);//
    let _ = FundPool::pack(fp, &mut fund_pool_account.data.borrow_mut());


    register_address_to_user_pool(address, investor, investor_pool_account);

    Ok(())
}

