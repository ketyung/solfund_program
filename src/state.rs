/**
 * CopyRight ketyung@techchee.com (Christopher K Y Chee)
 */

use solana_program::{
    pubkey::{Pubkey, PUBKEY_BYTES},
    program_error::ProgramError,
    program_pack::{IsInitialized,Pack,Sealed},
    clock::{Clock,UnixTimestamp},
    sysvar::Sysvar, 
    msg, 
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use crate::{error::PoolError};
use std::convert::{TryFrom};


pub const USER_POOL_SIZE_LIMIT : usize = 50;

#[derive(Clone, Debug, PartialEq)]
pub struct UserPool {

    pub user : Pubkey, 
    
    addresses : Vec<Pubkey>,

}

impl UserPool {

    pub fn new() -> Self {

        UserPool{

            user : Pubkey::default(),
            
            addresses : Vec::with_capacity(USER_POOL_SIZE_LIMIT),
            
        }
    }
}

impl Sealed for UserPool{}

impl Pack for UserPool {

    const LEN: usize = PUBKEY_BYTES + 1 + (PUBKEY_BYTES * USER_POOL_SIZE_LIMIT) ;

    fn pack_into_slice(&self, dst: &mut [u8]) {

        const L : usize =  PUBKEY_BYTES + 1 + (PUBKEY_BYTES *  USER_POOL_SIZE_LIMIT); 

        let output = array_mut_ref![dst, 0, L];

        let (user,addrs_len, addr_as_data_flat) = 
        mut_array_refs![output, PUBKEY_BYTES, 1, (PUBKEY_BYTES * USER_POOL_SIZE_LIMIT) ];

        
        *addrs_len = u8::try_from(self.addresses.len()).unwrap().to_le_bytes();
       
        user.copy_from_slice(self.user.as_ref());
      
        let mut offset = 0;

        for a in &self.addresses {

            let addr_flat = array_mut_ref![addr_as_data_flat, offset, PUBKEY_BYTES];

            let (pack_addr, _) = mut_array_refs![addr_flat, PUBKEY_BYTES, 0];

            pack_addr.copy_from_slice(a.as_ref());

            offset += PUBKEY_BYTES;
        }

       
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {

        const L : usize = PUBKEY_BYTES + 1 + (PUBKEY_BYTES * USER_POOL_SIZE_LIMIT) ; 

        let input = array_ref![src, 0, L];
        
        let (user, addr_len, pools) = array_refs![input, PUBKEY_BYTES ,1, 
        (PUBKEY_BYTES * USER_POOL_SIZE_LIMIT) ];

        let addr_len = u8::from_le_bytes(*addr_len);

        let mut offset = 0 ;

        let mut addresses =  Vec::with_capacity(addr_len as usize);

        for _ in 0..addr_len {

            let pk = array_ref![pools, offset, PUBKEY_BYTES];

            addresses.push(Pubkey::new_from_array(*pk));

            offset += PUBKEY_BYTES;
        }

        Ok(Self{
            user : Pubkey::new_from_array(*user) ,
            addresses : addresses,
        })
    }
}


impl UserPool {

    pub fn add_address (&mut self,  pubkey : Pubkey){

        if self.addresses.len() < USER_POOL_SIZE_LIMIT  {

            if !self.addresses.contains(&pubkey){

                self.addresses.push(pubkey);
            }
        }
    }


    pub fn remove_address(&mut self, pubkey : Pubkey) {


      //  self.addresses.retain(|&x| x != pubkey);

        let idx = self.addresses.iter().position(|&r| r == pubkey);
        if idx.is_some() {

            self.addresses.remove(idx.unwrap());
        }
    }

    pub fn all(&self) -> Vec<Pubkey>{

        self.addresses.clone()
    }

    pub fn len(&self) -> usize{

        self.addresses.len()
    }

}


#[derive(Clone, Debug, PartialEq)]
pub struct Market {

    pub pool_size : u16,

    fund_pools : Vec<Pubkey>,

    pub creator : Pubkey,

}

pub const POOL_MARKET_SIZE_LIMIT : usize = 100;

impl Market {

    pub fn new() -> Self {

        Market{

            pool_size : 0,
            
            fund_pools : Vec::with_capacity(POOL_MARKET_SIZE_LIMIT),

            creator : Pubkey::default(),
            
        }
    }
}



impl Market {

    
    pub fn add_fund_pool (&mut self,  pubkey : Pubkey){

        if self.fund_pools.len() < POOL_MARKET_SIZE_LIMIT  {

            if !self.fund_pools.contains(&pubkey){

                self.fund_pools.push(pubkey);

                self.pool_size = self.fund_pools.len() as u16;
            }
        }

    }


    pub fn remove_fund_pool(&mut self, pubkey : Pubkey) {


        self.fund_pools.retain(|&x| x != pubkey);
    }


    pub fn len(&self) -> usize{

        self.fund_pools.len()
    }


    pub fn all(&self) -> Vec<Pubkey>{

        self.fund_pools.clone()
    }

    pub fn clear(&mut self){

        self.fund_pools.clear();
        self.pool_size = self.fund_pools.len() as u16;
       
    }
}


impl Sealed for Market{}

impl Pack for Market {

    const LEN: usize = 2 + (PUBKEY_BYTES * POOL_MARKET_SIZE_LIMIT) + PUBKEY_BYTES ;

    fn pack_into_slice(&self, dst: &mut [u8]) {

        const L : usize =  2 + (PUBKEY_BYTES * POOL_MARKET_SIZE_LIMIT) + PUBKEY_BYTES; 

        let output = array_mut_ref![dst, 0, L];

        let (pools_size, pk_as_data_flat, creator ) = mut_array_refs![output, 2, 
        (PUBKEY_BYTES * POOL_MARKET_SIZE_LIMIT), PUBKEY_BYTES ];


        *pools_size = self.pool_size.to_le_bytes();
        
        creator.copy_from_slice(self.creator.as_ref());


        let mut offset = 0;

        for pk in &self.fund_pools {

            let pk_flat = array_mut_ref![pk_as_data_flat, offset, PUBKEY_BYTES];

            let (pack_pk, _) = mut_array_refs![pk_flat, PUBKEY_BYTES, 0];

            pack_pk.copy_from_slice(pk.as_ref());

            offset += PUBKEY_BYTES;
        }
       
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {

        const L : usize = 2 + (PUBKEY_BYTES * POOL_MARKET_SIZE_LIMIT) + PUBKEY_BYTES ; 

        let input = array_ref![src, 0, L];
        
        let (pools_len, pools, creator) = array_refs![input, 2, L-34, 32 ];

        let pools_len = u16::from_le_bytes(*pools_len);

        let creator = Pubkey::new_from_array(*creator);

        let mut offset = 0 ;

        let mut new_pools =  Vec::with_capacity(pools_len as usize);

        for _ in 0..pools_len {

            let pk = array_ref![pools, offset, PUBKEY_BYTES];

            new_pools.push(Pubkey::new_from_array(*pk));

            offset += PUBKEY_BYTES;
        }

        Ok(Self{
            pool_size : pools_len as u16 ,
            fund_pools : new_pools,
            creator : creator, 
        })
    }
}



#[derive(Clone, Debug)]
pub struct FundPoolInvestor {

    pub investor : Pubkey,

    pub address : Pubkey,
   
    pub token_count : u64, 

    pub date : UnixTimestamp, 
}


impl PartialEq for FundPoolInvestor {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}


const FUND_POOL_INVESTOR_LEN : usize = 80; 


pub const FUND_POOL_INVESTOR_LIMIT : usize = 100;

pub const FUND_POOL_WITHDRAWER_LIMIT : usize = 100;


#[derive(Clone, Debug, PartialEq)]
pub struct FundPool {

    pub is_initialized: bool,

    pub manager : Pubkey, 
   
    pub address : Pubkey, 

    pub token_address : Pubkey, // need to store the token mint, the temp token acc

    // the commission
    // currently supports flat rate commission
    // for fund manager only in SOL
    pub fee_in_lamports : u64,
   
    pub token_count : u64,

    pub rm_token_count : u64, 

    pub token_to_lamport_ratio : u64, 

    pub is_finalized : bool,

    pub icon : u16,
       
    investors : Vec<FundPoolInvestor>,
    
    withdrawers : Vec<FundPoolInvestor>,
    
}


impl Sealed for FundPool {}


// 1 + 32 + 32 + 32 + 8 + 8 + 1 + ((32 + 32 + 8) * FUND_POOL_INVESTOR_LIMIT)
// (32 + 32 + 8 + 8) * + FUND_POOL_WITHDRAWER_LIMIT
// 84 + 2 // for the two lengths 
const FUND_POOL_LENGTH : usize = 132 + 
(80 * FUND_POOL_INVESTOR_LIMIT) + (80 * FUND_POOL_WITHDRAWER_LIMIT)  + 2; 

impl Pack for FundPool {

    const LEN: usize = FUND_POOL_LENGTH;

    fn pack_into_slice(&self, dst: &mut [u8]) {

        let output = array_mut_ref![dst, 0, FUND_POOL_LENGTH];
       
        let (is_initialized,manager, address, token_address, lamports, 
        token_count,rm_token_count, token_to_lamport_ratio, 
        is_finalized,icon,ivs_len, 
        wds_len,iv_data_flat,wd_data_flat) = 
        mut_array_refs![ output,1,PUBKEY_BYTES,PUBKEY_BYTES,PUBKEY_BYTES,
        8, 8,8,8,1,2,1,1, FUND_POOL_INVESTOR_LEN * FUND_POOL_INVESTOR_LIMIT, 
        FUND_POOL_INVESTOR_LEN * FUND_POOL_WITHDRAWER_LIMIT];

    
        pack_bool(self.is_initialized, is_initialized);
        manager.copy_from_slice(self.manager.as_ref());
        address.copy_from_slice(self.address.as_ref());
        token_address.copy_from_slice(self.token_address.as_ref());
        *lamports = self.fee_in_lamports.to_le_bytes();
        *token_count = self.token_count.to_le_bytes();
        *rm_token_count = self.rm_token_count.to_le_bytes();
        *token_to_lamport_ratio = self.token_to_lamport_ratio.to_le_bytes();
        *icon = self.icon.to_le_bytes();
        pack_bool(self.is_finalized, is_finalized);
       
        *ivs_len = u8::try_from(self.investors.len()).unwrap().to_le_bytes();
        *wds_len = u8::try_from(self.withdrawers.len()).unwrap().to_le_bytes();


        let mut offset = 0 ;

        for iv in &self.investors {

            let iv_flat = array_mut_ref![iv_data_flat, offset, FUND_POOL_INVESTOR_LEN];

            let (address,
                investor, 
                token_count, 
                date) = 
                mut_array_refs![iv_flat, PUBKEY_BYTES, PUBKEY_BYTES, 8, 8];

            address.copy_from_slice(iv.address.as_ref());
            investor.copy_from_slice(iv.investor.as_ref());
            *date = iv.date.to_le_bytes();
            *token_count = iv.token_count.to_le_bytes();
            

            offset += FUND_POOL_INVESTOR_LEN;

        }


        for wd in &self.withdrawers {

            let wd_flat = array_mut_ref![wd_data_flat, offset, FUND_POOL_INVESTOR_LEN];

            let (address,investor,token_count,date) = mut_array_refs![wd_flat, PUBKEY_BYTES, PUBKEY_BYTES,8, 8];

            address.copy_from_slice(wd.address.as_ref());
            investor.copy_from_slice(wd.investor.as_ref());
            *date = wd.date.to_le_bytes();
            *token_count = wd.token_count.to_le_bytes();
          
            offset += FUND_POOL_INVESTOR_LEN;

        }


    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
       
        let input = array_ref![src, 0, FUND_POOL_LENGTH];
       
        let (is_initialized,manager, address, token_address, lamports, 
            token_count,rm_token_count, token_to_lamport_ratio,
            is_finalized, icon, invs_len, wds_len, invs_flat,wds_flat) =

        array_refs![input, 
        1, PUBKEY_BYTES, PUBKEY_BYTES, PUBKEY_BYTES,
        8, 8, 8,8, 1, 2, 1,1, (FUND_POOL_INVESTOR_LEN * FUND_POOL_INVESTOR_LIMIT), 
        (FUND_POOL_INVESTOR_LEN * FUND_POOL_WITHDRAWER_LIMIT)];

        let is_init = unpack_bool(is_initialized).unwrap();
        let is_final = unpack_bool(is_finalized).unwrap();
        let mgr = Pubkey::new_from_array(*manager);
        let addr = Pubkey::new_from_array(*address);
        let tk_addr = Pubkey::new_from_array(*token_address);
        let lp = u64::from_le_bytes(*lamports);
        let tkc = u64::from_le_bytes(*token_count);
        let rm_tkc = u64::from_le_bytes(*rm_token_count);
        let tk_ratio = u64::from_le_bytes(*token_to_lamport_ratio);
        let ic = u16::from_le_bytes(*icon);
    
        
        let invs_len = u8::from_le_bytes(*invs_len);
        let mut invs =  Vec::with_capacity(invs_len as usize);

        let mut offset = 0 ;

        for _ in 0..invs_len {

            let iv_flat = array_ref![invs_flat, offset, FUND_POOL_INVESTOR_LEN];

            let (address,investor,token_count, date) = array_refs![iv_flat, PUBKEY_BYTES, PUBKEY_BYTES,8, 8];

            invs.push(

                FundPoolInvestor{ 
                    investor : Pubkey::new_from_array(*investor), 
                    address :Pubkey::new_from_array(*address), 
                    token_count : u64::from_le_bytes(*token_count), 
                    date : i64::from_le_bytes(*date), 
                }

            );

            offset += FUND_POOL_INVESTOR_LEN;
        }


        let wds_len = u8::from_le_bytes(*wds_len);
        let mut wds =  Vec::with_capacity(wds_len as usize);
     
        for _ in 0..wds_len {

            let wd_flat = array_ref![wds_flat, offset, FUND_POOL_INVESTOR_LEN];

            #[allow(clippy::ptr_offset_with_cast)]
            let (add,inv,tkc, dt) = array_refs![wd_flat, PUBKEY_BYTES, PUBKEY_BYTES,8, 8];

            
            wds.push(

                FundPoolInvestor{ 
                    investor : Pubkey::new_from_array(*inv), 
                    address :Pubkey::new_from_array(*add), 
                    token_count : u64::from_le_bytes(*tkc), 
                    date : i64::from_le_bytes(*dt), 
                }

            );

            offset += FUND_POOL_INVESTOR_LEN;
        }


        Ok (Self {
            is_initialized : is_init, 
            manager : mgr,
            address : addr,
            token_address : tk_addr,
            fee_in_lamports : lp,
            token_count : tkc,
            rm_token_count: rm_tkc,
            token_to_lamport_ratio : tk_ratio, 
            is_finalized : is_final,
            icon : ic, 
            investors : invs,
            withdrawers : wds, 
        })
       
    }
}

impl IsInitialized for FundPool {
    fn is_initialized(&self) -> bool {
        
        self.is_initialized
    }
}


impl FundPool {

    pub fn new(is_initialized : bool) -> Self {

        FundPool{

            is_initialized : is_initialized,
            manager : Pubkey::default(),
            address : Pubkey::default(),
            token_address : Pubkey::default(), 
            fee_in_lamports : 0,
            token_count : 0,
            rm_token_count : 0,
            token_to_lamport_ratio : 0, 
            is_finalized : false,
            icon : 0,
            investors : Vec::with_capacity(FUND_POOL_INVESTOR_LIMIT),
            withdrawers : Vec::with_capacity(FUND_POOL_WITHDRAWER_LIMIT),
            
        }
    }
}

impl FundPool {


    pub fn register_investor(&mut self, 
        investor : Investor) -> Result<bool, PoolError> {

        self.register_fund_pool_investor(

            FundPoolInvestor {

                investor : investor.investor,
                address : investor.address,
                token_count : investor.token_count,
                date : investor.date, 
            }

        )
    
    }

    fn register_fund_pool_investor(&mut self, investor : FundPoolInvestor) -> Result<bool, PoolError> {

        if self.investors.len() < FUND_POOL_INVESTOR_LIMIT  {

            if !self.investors.contains(&investor){

                let mut inv = investor;

                inv.date = Clock::get().unwrap().unix_timestamp;
                //msg!("Current date time:: {}", inv.date);

                self.investors.push(inv);

                //msg!("investors.count::{}", self.investors.len());
                
                return Ok(true);
            }

            return Err(PoolError::InvestorAlreadyExists);
       
        }

        Err(PoolError::MaxInvestorReached)
       
    }


    pub fn investor_count(&self) -> usize {

        self.investors.len() 
    }

}


impl FundPool {

    pub fn register_withdrawer(&mut self, withdrawer : FundPoolInvestor) -> bool  {

        if self.withdrawers.len() < FUND_POOL_WITHDRAWER_LIMIT  {

            if !self.withdrawers.contains(&withdrawer){


                let mut wd = withdrawer;

                wd.date = Clock::get().unwrap().unix_timestamp;
                msg!("Current date time:: {}", wd.date);

                self.withdrawers.push(wd);

                return true; 
                
            }

        }

        return false; 
       
    }


    pub fn withdrawer_count(&self) -> usize {

        self.withdrawers.len() 
    }

 
}


#[derive(Clone, Debug, PartialEq)]
pub struct Investor {

    pub investor : Pubkey, 

    pub pool_address : Pubkey, 

    pub address : Pubkey,

    pub amount : u64, 

    pub token_address : Pubkey,

    pub token_count : u64,
  
    pub date : UnixTimestamp, 

}

impl Investor {

    pub fn new() -> Self {

        Investor {
            investor : Pubkey::default(),
            pool_address : Pubkey::default(),
            address : Pubkey::default(),
            amount : 0,
            token_address : Pubkey::default(),
            token_count : 0,
            date : Clock::get().unwrap().unix_timestamp, 
        }
    }
}

impl Sealed for Investor {}


const INVESTOR_DATA_SIZE : usize = PUBKEY_BYTES + PUBKEY_BYTES + 
PUBKEY_BYTES + 8 + PUBKEY_BYTES + 8 + 8;

impl Pack for Investor {

    const LEN: usize = INVESTOR_DATA_SIZE;

    fn pack_into_slice(&self, dst: &mut [u8]) {

        let output = array_mut_ref![dst, 0, INVESTOR_DATA_SIZE];
       
        let (investor,pool_address, address, amount,token_address,token_count, date) = 
        mut_array_refs![ output,PUBKEY_BYTES,PUBKEY_BYTES,PUBKEY_BYTES,8,PUBKEY_BYTES, 8, 8];


        investor.copy_from_slice(self.investor.as_ref());
        pool_address.copy_from_slice(self.pool_address.as_ref());
        address.copy_from_slice(self.address.as_ref());
        
        *token_count = self.token_count.to_le_bytes();
        *amount = self.amount.to_le_bytes();
        token_address.copy_from_slice(self.token_address.as_ref());
        *date = self.date.to_le_bytes();

    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
   
        let input = array_ref![src, 0, INVESTOR_DATA_SIZE];
       
        let (investor,pool_address, address, amount,token_address,token_count, date)  =

        array_refs![input, PUBKEY_BYTES, PUBKEY_BYTES,
        PUBKEY_BYTES, 8, PUBKEY_BYTES, 8, 8 ];

        let investor = Pubkey::new_from_array(*investor);
        let pool_address = Pubkey::new_from_array(*pool_address);
        let token_address = Pubkey::new_from_array(*token_address);
        let address = Pubkey::new_from_array(*address);
        let amount = u64::from_le_bytes(*amount);
        let token_count = u64::from_le_bytes(*token_count);
        let date = i64::from_le_bytes(*date);

        Ok( Investor{
            investor : investor,
            address : address,
            pool_address : pool_address,
            token_address : token_address,
            amount :amount, 
            token_count : token_count,
            date : date, 
        })
    }
}






fn pack_bool(boolean: bool, dst: &mut [u8; 1]) {
    *dst = (boolean as u8).to_le_bytes()
}

pub fn unpack_bool(src: &[u8; 1]) -> Result<bool, ProgramError> {
    let b = u8::from_le_bytes(*src);
    match  b {
        0 => Ok(false),
        1 => Ok(true),
        _ => {
            Err(ProgramError::InvalidAccountData)
        }
    }
}