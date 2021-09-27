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


#[derive(Clone, Debug, PartialEq)]
pub struct PoolMarket {

    pub pool_size : u16,

    fund_pools : Vec<Pubkey>,

}

pub const POOL_MARKET_SIZE_LIMIT : usize = 100;

impl PoolMarket {

    pub fn new() -> Self {

        PoolMarket{

            pool_size : 0,
            
            fund_pools : Vec::with_capacity(POOL_MARKET_SIZE_LIMIT),
            
        }
    }
}



impl PoolMarket {

    
    pub fn add_fund_pool (&mut self,  pubkey : Pubkey){

        if self.fund_pools.len() < POOL_MARKET_SIZE_LIMIT  {

            if !self.fund_pools.contains(&pubkey){

                self.fund_pools.push(pubkey);

                self.pool_size = self.fund_pools.len() as u16;
            }
        }

    }


    pub fn remove_fund_pool(&mut self, pubkey : Pubkey) {

        let idx = self.fund_pools.iter().position(|&r| r == pubkey);
        if idx.is_some() {

            self.fund_pools.remove(idx.unwrap());
            self.pool_size = self.fund_pools.len() as u16;

        }
    }

    pub fn all(&self) -> Vec<Pubkey>{

        self.fund_pools.clone()
    }

    pub fn clear(&mut self){

        self.fund_pools.clear();
        self.pool_size = self.fund_pools.len() as u16;
       
    }
}


impl Sealed for PoolMarket{}

impl Pack for PoolMarket {

    const LEN: usize = 2 + (PUBKEY_BYTES * POOL_MARKET_SIZE_LIMIT) ;

    fn pack_into_slice(&self, dst: &mut [u8]) {

        const L : usize =  2+ (PUBKEY_BYTES * POOL_MARKET_SIZE_LIMIT); 

        let output = array_mut_ref![dst, 0, L];

        let (pools_size, pk_as_data_flat) = mut_array_refs![output, 2, (PUBKEY_BYTES * POOL_MARKET_SIZE_LIMIT) ];


        *pools_size = self.pool_size.to_le_bytes();

        let mut offset = 0;

        for pk in &self.fund_pools {

            let pk_flat = array_mut_ref![pk_as_data_flat, offset, PUBKEY_BYTES];

            let (pack_pk, _) = mut_array_refs![pk_flat, PUBKEY_BYTES, 0];

            pack_pk.copy_from_slice(pk.as_ref());

            offset += PUBKEY_BYTES;
        }

       
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {

        const L : usize = 2 + (PUBKEY_BYTES * POOL_MARKET_SIZE_LIMIT) ; 

        let input = array_ref![src, 0, L];
        
        let (pools_len, pools) = array_refs![input, 2, L-2 ];

        let pools_len = u16::from_le_bytes(*pools_len);

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
        })
    }
}


#[derive(Clone, Debug)]
pub struct FundPoolInvestor {

    pub investor : Pubkey,

    pub address : Pubkey,
   
    pub amount : u64, 

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

    pub lamports : u64,

    pub token_count : u64,

    pub is_finalized : bool,
       
    investors : Vec<FundPoolInvestor>,
    
    withdrawers : Vec<FundPoolInvestor>,
    
}


impl Sealed for FundPool {}


// 1 + 32 + 32 + 64 + 64 + 1 + ((32 + 32 + 8) * FUND_POOL_INVESTOR_LIMIT)
// (32 + 32 + 8 + 8) * + FUND_POOL_WITHDRAWER_LIMIT
// 82 + 
const POOL_WALLET_LENGTH : usize = 82 + 
(80 * FUND_POOL_INVESTOR_LIMIT) + (80 * FUND_POOL_WITHDRAWER_LIMIT) ; 

impl Pack for FundPool {

    const LEN: usize = POOL_WALLET_LENGTH;

    fn pack_into_slice(&self, dst: &mut [u8]) {

        let output = array_mut_ref![dst, 0, POOL_WALLET_LENGTH];
       
        let (
        is_initialized,
        manager, 
        address, 
        lamports, 
        token_count,
        is_finalized,
        iv_data_flat,
        wd_data_flat,
        ) = mut_array_refs![ output,1,PUBKEY_BYTES,PUBKEY_BYTES,
        8, 8,1, FUND_POOL_INVESTOR_LEN * FUND_POOL_INVESTOR_LIMIT, 
        FUND_POOL_INVESTOR_LEN * FUND_POOL_WITHDRAWER_LIMIT];

    
        pack_bool(self.is_initialized, is_initialized);
        manager.copy_from_slice(self.manager.as_ref());
        address.copy_from_slice(self.address.as_ref());
        *lamports = self.lamports.to_le_bytes();
        *token_count = self.token_count.to_le_bytes();
        pack_bool(self.is_finalized, is_finalized);
       
        let mut offset = 0 ;

        for iv in self.investors {

            let iv_flat = array_mut_ref![iv_data_flat, offset, FUND_POOL_INVESTOR_LEN];

            let (address,
                investor, 
                amount, 
                date) = 
                mut_array_refs![iv_flat, PUBKEY_BYTES, PUBKEY_BYTES, 8, 8];

            address.copy_from_slice(iv.address.as_ref());
            investor.copy_from_slice(iv.investor.as_ref());
            *date = iv.date.to_le_bytes();
            *amount = iv.amount.to_le_bytes();
            

            offset += FUND_POOL_INVESTOR_LEN;

        }


        for wd in self.withdrawers {

            let wd_flat = array_mut_ref![wd_data_flat, offset, FUND_POOL_INVESTOR_LEN];

            let (address,investor,amount, date) = mut_array_refs![wd_flat, PUBKEY_BYTES, PUBKEY_BYTES,8, 8];

            address.copy_from_slice(wd.address.as_ref());
            investor.copy_from_slice(wd.investor.as_ref());
            *date = wd.date.to_le_bytes();
            *amount = wd.amount.to_le_bytes();
          
            offset += FUND_POOL_INVESTOR_LEN;

        }


    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
       
        let input = array_ref![src, 0, POOL_WALLET_LENGTH];
       

        let (
        is_initialized,
        manager, 
        address, 
        lamports, 
        token_count,
        is_finalized,
        ivs,
        wds,
        ) =

        array_refs![input, 
        1, PUBKEY_BYTES, PUBKEY_BYTES, 
        8, 8, 1, (FUND_POOL_INVESTOR_LEN * FUND_POOL_INVESTOR_LIMIT), 
        (FUND_POOL_INVESTOR_LEN * FUND_POOL_WITHDRAWER_LIMIT)];

        let is_init = unpack_bool(is_initialized).unwrap();
        let is_final = unpack_bool(is_finalized).unwrap();
        let mgr = Pubkey::new_from_array(*manager);
        let addr = Pubkey::new_from_array(*address);
        let lp = u64::from_le_bytes(*lamports);
        let tkc = u64::from_le_bytes(*token_count);

        
        let invs_len = FUND_POOL_INVESTOR_LEN * FUND_POOL_INVESTOR_LIMIT;
        let mut invs =  Vec::with_capacity(invs_len);

        let mut offset = 0 ;

        for _ in 0..invs_len {

            let iv = array_ref![invs, offset, FUND_POOL_INVESTOR_LEN];


            let (address,investor,amount, date) = mut_array_refs![iv, PUBKEY_BYTES, PUBKEY_BYTES,8, 8];

            let ivv = FundPoolInvestor{ 
                investor : Pubkey::new_from_array(*investor), 
                address :Pubkey::new_from_array(*address), 
                amount : u64::from_le_bytes(*amount), 
                date : i64::from_le_bytes(*date), 
            };

            invs.push(ivv);

            offset += FUND_POOL_INVESTOR_LEN;
        }


        let wds_len = FUND_POOL_INVESTOR_LEN * FUND_POOL_WITHDRAWER_LIMIT;
        let mut wds =  Vec::with_capacity(wds_len);

        for _ in 0..wds_len {

            let iv = array_ref![wds, offset, FUND_POOL_INVESTOR_LEN];

            let (address,investor,amount, date) = mut_array_refs![iv, PUBKEY_BYTES, PUBKEY_BYTES,8, 8];

            let wdd = FundPoolInvestor{ 
                investor : Pubkey::new_from_array(*investor), 
                address :Pubkey::new_from_array(*address), 
                amount : u64::from_le_bytes(*amount), 
                date : i64::from_le_bytes(*date), 
            };

            wds.push(wdd);

            offset += FUND_POOL_INVESTOR_LEN;
        }


        Ok (Self {
            is_initialized : is_init, 
            manager : Pubkey::new_from_array(*manager),
            address : Pubkey::new_from_array(*address),
            lamports : u64::from_le_bytes(*lamports),
            token_count : u64::from_le_bytes(*token_count),
            is_finalized : is_final,
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
            lamports : 0,
            token_count : 0,
            is_finalized : false,
            investors : Vec::with_capacity(FUND_POOL_INVESTOR_LIMIT),
            withdrawers : Vec::with_capacity(FUND_POOL_WITHDRAWER_LIMIT),
            
        }
    }
}

impl FundPool {

    pub fn register_investor(&mut self, investor : FundPoolInvestor) -> Result<bool, PoolError> {

        if self.investors.len() < FUND_POOL_INVESTOR_LIMIT  {

            if !self.investors.contains(&investor){

                investor.date = Clock::get().unwrap().unix_timestamp;
                msg!("Current date time:: {}", investor.date);

                self.investors.push(investor);
                return Ok(true);
            }

            return Err(PoolError::InvestorAlreadyExists);
       
        }

        Err(PoolError::MaxInvestorReached)
       
    }


    pub fn investor_count(&self) -> usize {

        self.investors.len() 
    }

    pub fn remove_investor(&mut self, investor : FundPoolInvestor) {


        let idx = self.investors.iter().position(|&r| r == investor);
        if idx.is_some() {

            self.investors.remove(idx.unwrap());
        
        }
    }

}


impl FundPool {

    pub fn register_withdrawer(&mut self, withdrawer : FundPoolInvestor) -> bool  {

        if self.withdrawers.len() < FUND_POOL_WITHDRAWER_LIMIT  {

            if !self.withdrawers.contains(&withdrawer){

                withdrawer.date = Clock::get().unwrap().unix_timestamp;
                msg!("Current date time:: {}", withdrawer.date);

                self.withdrawers.push(withdrawer);

                return true; 
                
            }

        }

        return false; 
       
    }


    pub fn withdrawer_count(&self) -> usize {

        self.withdrawers.len() 
    }

    pub fn remove_withdrawer(&mut self, withdrawer : FundPoolInvestor) {

        let idx = self.withdrawers.iter().position(|&r| r == withdrawer );
        if idx.is_some() {

            self.withdrawers.remove(idx.unwrap());        
        }
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