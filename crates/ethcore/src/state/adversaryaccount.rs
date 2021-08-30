use transaction_ext::Transaction;
use types::transaction::{Action, SignedTransaction, TypedTransaction};
use ethereum_types::{Address, Bloom, H256, U256};
use bytes::Bytes;
use parking_lot::{RwLock};
use std::{
    cell::{RefCell, RefMut},
    collections::{HashMap},
    sync::Arc,
};
use ethtrie::{Result as TrieResult, SecTrieDB, TrieDB, TrieFactory};
use std::str::FromStr;
pub use state::erc20macro::*;
/// Direction for each transfer
#[derive(Copy, Clone, Debug)]
pub enum TransferDir {
    From(Address), //The "Address" is the receiver
    To(Address), //The "Address" is the sender
}
/// Accounts who perform transfer in transaction
#[derive(
    Clone,
    Debug,
)]
pub struct AdversaryAccount {
    // account balance trace
    //Address 1: the owner of the tokens
    //Address 2: the contract address of the token
    //TransferDir: the token flow direction of the transfer
    //Vec<U256>: the trace of the toke flows
    balance_traces: RefCell<HashMap<Address, HashMap<Address, Vec<(TransferDir, U256)>>>>,
    // store all transfers in order
    transfer_in_order: RefCell<Vec<(Address, Address, U256, Address)>>,
    //Track all address flash loan potential attack
    flash_loan_information: RefCell<HashMap<Address, IndividualAdversaryAccountHelper>>,
    // potential flash loan transaction
    old_tx: SignedTransaction,
    // Nonce of Adversary account.
    nonce: U256,
    //code to deploy
    //code: TrieResult<Option<Arc<Bytes>>>,
    //data which does function call
    //data: Bytes,
    // Nonce of my account,
    my_nonce: U256,
    // new deploy transaction,
    deploy_tx: Option<SignedTransaction>,
    // new flash loan transaction,
    new_tx: Option<SignedTransaction>,
}
/// The idendity of the address in the transaction with profit or cost in 0.0001 USD unit
#[derive(
    Copy, 
    Clone, 
    Debug
)]
pub enum PotentialIdentity {
    Beneficiary(U256),
    Victim(U256),
    Neither,
    NotDecided,
}
/// single one adversary account flash loan pattern check
#[derive(
    Clone,
    Debug,
)]
pub struct IndividualAdversaryAccountHelper {
    identity: PotentialIdentity,
    //flash_loan_end_positions: RefCell<HashMap<Address, (usize, usize)>>,
}
/// IndividualAdversaryAccountHelper impl
#[doc(hidden)]
impl IndividualAdversaryAccountHelper {
    pub fn new(iden: PotentialIdentity) -> Self {
        IndividualAdversaryAccountHelper{
            identity: iden,
            //flash_loan_end_positions: Default::default(),
        }
    }
}
/// AdversaryAccount impl
#[doc(hidden)]
impl AdversaryAccount {
    pub fn new(n: U256, t: SignedTransaction, m_n: U256) -> Self {
        let ret = AdversaryAccount {
            balance_traces: Default::default(),
            transfer_in_order: Default::default(),
            flash_loan_information: Default::default(),
            old_tx: t.clone(),
            nonce: n,
            //code:,
            //data:,
            my_nonce: m_n,
            deploy_tx: None,
            new_tx: None,            
        };
        ret
    }
    pub fn lookup_balance_trace_from_address(&self, addr: Address) -> Option<HashMap<Address,Vec<(TransferDir, U256)>>> {
        self.balance_traces.borrow_mut().get_mut(&addr).map(|value| value.clone())
    }
    pub fn set_balance(&self, addr: Address, related_addr: Address, bal: U256, token_addr: Address, sender_receiver: bool) -> Option<U256> {
        match self.lookup_balance_trace_from_address(addr) {
            Some(mut map_val) => {
                match map_val.get_mut(&token_addr).map(|value| value.clone()) {
                    Some(val) => {
                        let mut new_val = val.to_vec();
                        if sender_receiver {
                            new_val.push((TransferDir::From(related_addr), bal));
                        }else{
                            new_val.push((TransferDir::To(related_addr), bal));
                        }
                        map_val.insert(token_addr, new_val);
                    },
                    None => {
                        let mut new_val = Vec::new();
                        if sender_receiver {
                            new_val.push((TransferDir::From(related_addr), bal));
                        }else{
                            new_val.push((TransferDir::To(related_addr), bal));
                        }
                        map_val.insert(token_addr, new_val);
                    },
                }
                self.balance_traces.borrow_mut().insert(addr, map_val);
                Some(bal)
            },
            None => {
                let mut new_val = Vec::new();
                if sender_receiver {
                    new_val.push((TransferDir::From(related_addr), bal));
                }else{
                    new_val.push((TransferDir::To(related_addr), bal));
                }                
                let mut inner_map = HashMap::new();
                inner_map.insert(token_addr, new_val);
                self.balance_traces.borrow_mut().insert(addr, inner_map);
                Some(bal)
            },
        }
    }
    pub fn set_token_flow(        
        &self,
        addrfrom: Address, 
        addrto: Address, 
        amt: U256,
        token_addr: Address,
    ) -> Option<U256> {
        match CONTRACT_ADDRESSES.iter().position(|val| *val == token_addr) {
            Some(_) => {
                self.transfer_in_order.borrow_mut().push((addrfrom, addrto, amt, token_addr));
                self.set_balance(addrfrom, addrto, amt, token_addr, true);
                self.set_balance(addrto, addrfrom, amt, token_addr, false)
            },
            None => None,
        }
    }
    pub fn identify_helper (&self) -> Option<Vec<(Address, Vec<(Address, bool, U256)>)>> {
        let mut ret = Vec::new();
        for (a, b) in self.balance_traces.borrow_mut().iter() {
            assert!(!b.is_empty());
            let mut inner_ret = Vec::new();
            for (c, d) in b.iter() {
                assert!(!d.is_empty());
                let mut earn_flag = true;
                let mut benefit = U256::zero();
                for (dir, val) in d.iter() {
                    match dir {
                        TransferDir::From(_) => {
                            if earn_flag {
                                if *val < benefit {
                                    earn_flag = true;
                                    benefit = benefit.saturating_sub(*val);
                                }else{
                                    earn_flag = false;
                                    benefit = val.saturating_sub(benefit);
                                }
                            }else{
                                earn_flag = false;
                                benefit = val.saturating_add(benefit);
                            }
                        },
                        TransferDir::To(_) => {
                            if earn_flag {
                                earn_flag = true;
                                benefit = val.saturating_add(benefit);
                            }else{
                                if *val < benefit {
                                    earn_flag = false;
                                    benefit = benefit.saturating_sub(*val);
                                }else{
                                    earn_flag = true;
                                    benefit = val.saturating_sub(benefit);
                                }
                            }                            
                        },
                    }
                }
                inner_ret.push((*c, earn_flag, benefit));
            }
            ret.push((Address::from(*a), inner_ret.to_vec()));
        } 
        match ret.is_empty() {
            true => None,
            false => Some(ret),
        }  
    }
    pub fn anaylsis_net_profit_in_one_thousandth_usd (&self) {
        match self.identify_helper() {
            Some(ret_vec) => {
                for (addr, values) in ret_vec.iter(){
                    let mut earn_flag = true;
                    let mut benefit = U256::zero();
                    for (a, b, c) in values.iter() {
                        match CONTRACT_ADDRESSES.iter().position(|val| *val == *a) {
                            Some(idx) => {
                                let net_value = c.saturating_mul(TOKEN_USD_PRICES[idx]);
                                if earn_flag {
                                    if *b {
                                        earn_flag = true;
                                        benefit = net_value.saturating_add(benefit);
                                    }else{
                                        if net_value > benefit {
                                            earn_flag = false;
                                            benefit = net_value.saturating_sub(benefit);
                                        }else{
                                            earn_flag = true;
                                            benefit = benefit.saturating_sub(net_value);
                                        }
                                    }
                                }else{
                                    if !*b {
                                        earn_flag = false;
                                        benefit = net_value.saturating_add(benefit);
                                    }else{
                                        if net_value > benefit {
                                            earn_flag = true;
                                            benefit = net_value.saturating_sub(benefit);
                                        }else{
                                            earn_flag = false;
                                            benefit = benefit.saturating_sub(net_value);
                                        }                                        
                                    }
                                }
                            },
                            None => panic!("The token address is not recognizable!"),
                        }                    
                    } 
                    let mut information = self.flash_loan_information.borrow_mut();
                    match information.get_mut(&addr).map(|value| value.clone()) {
                        Some(mut val) => {
                            if benefit == U256::zero() {
                                val.identity = PotentialIdentity::Neither;
                            }else if earn_flag{
                                val.identity = PotentialIdentity::Beneficiary(benefit);
                            }else{
                                val.identity = PotentialIdentity::Victim(benefit);
                            }                            
                            information.insert(*addr, val);
                        }
                        None => {
                            if benefit == U256::zero() {
                                information.insert(*addr, IndividualAdversaryAccountHelper::new(PotentialIdentity::Neither));
                            }else if earn_flag{
                                information.insert(*addr, IndividualAdversaryAccountHelper::new(PotentialIdentity::Beneficiary(benefit)));
                            }else{
                                information.insert(*addr, IndividualAdversaryAccountHelper::new(PotentialIdentity::Victim(benefit)));
                            }
                        }
                    }
                }
            },
            None => {},
        }        
    }
    // DEBUGGING: print out address who gains token
    pub fn identify_beneficiary (&self) -> Option<Vec<(Address, Vec<(Address, U256)>)>> {
        match self.identify_helper() {
            Some(ret_vec) => {
                Some(
                    ret_vec.iter().map(
                        |val| (val.0, 
                                val.1.iter().filter(|inner| inner.1 == true)
                                            .map(|val| (val.0, val.2))
                                            .collect()
                              )
                    )
                    .collect()
                )
            },
            None => None,
        }
    }
    // DEBUGGING: print out address who loses token
    pub fn identify_victim (&self) -> Option<Vec<(Address, Vec<(Address, U256)>)>> {
        match self.identify_helper() {
            Some(ret_vec) => {
                Some(
                    ret_vec.iter().map(
                        |val| (val.0, 
                                val.1.iter().filter(|inner| inner.1 == false)
                                            .map(|val| (val.0, val.2))
                                            .collect()
                              )
                    )
                    .collect()
                )
            },
            None => None,
        } 
    }
    pub fn find_flash_loan_end_positions(&self) -> Option<(Vec<(Address, Address, U256, Address)>, Vec<(Address, Address, U256, Address)>)>{
        let mut flash_loan_start = Vec::new();
        let mut flash_loan_end = Vec::new();
        let transfers = self.transfer_in_order.borrow_mut();
        for (from, to, amt, token) in transfers.iter() {
            match FLASH_LOAN_CONTRACT_ADDRESSES.iter().position(|val| *val == *from) {
                Some(_) => {flash_loan_start.push((*from, *to, *amt, *token));},
                None => (),
            }
            match FLASH_LOAN_CONTRACT_ADDRESSES.iter().position(|val| *val == *to) {
                Some(_) => {
                    if let Some(_) = flash_loan_start.iter().position(|i| i.0==*to && i.2==*amt && i.3==*token){
                        flash_loan_end.push((*from, *to, *amt, *token));
                    }
                },
                None => (),
            }
        }
        if flash_loan_end.is_empty() && flash_loan_end.len() == flash_loan_start.len(){
            None
        }else{
            Some((flash_loan_start, flash_loan_end))
        }
    }
    //If a flash loan attack is detected, return true. Otherwise, false.
    pub fn token_transfer_flash_loan_check (&self) -> bool{
        //TODO: Check the transaction is successfully executed
        //Check the address is beneficiary after all or not. (Pull down current price of each token)
        //Check who is victim
        self.anaylsis_net_profit_in_one_thousandth_usd();
        let mut beneficiary: Vec<Address> = Vec::new();
        let mut victim: Vec<Address> = Vec::new();
        for (addr, result) in self.flash_loan_information.borrow_mut().iter(){
            match result.identity {
                PotentialIdentity::Beneficiary(val) => {
                    println!("Address {:?} gains {:?} in 0.0001 USD unit", *addr, val);
                    beneficiary.push(*addr);
                },
                PotentialIdentity::Victim(val) => {
                    println!("Address {:?} loses {:?} in 0.0001 USD unit", *addr, val);
                    victim.push(*addr);
                }
                _ => (),
            }
        }
        if beneficiary.is_empty() {
            return false;
        }
        //Find some address get amt from flash loan contract address
        //TODO: generally the flash loan receiver is the beneficiary by the end of the transaction. Need to think about corner case
        if let Some((start, _)) = self.find_flash_loan_end_positions() {
            if !start.iter().all(|val| beneficiary.contains(&val.1)) {
                return false;
            }
            //DEBUGGING: print all loan receiver
            for (a, b, c, d) in start.iter() {
                println!("Flash Loan Address {:?} sends {:?} of token address {:?} to Address {:?}", *a, *c, *d, *b);
            }
        }else{
            return false;
        }

        //Check the cost at the beginning of the transaction
        true
    }
    pub fn assemable_deploy_transaction (&self) {

    }
    pub fn assemable_new_transaction (&self) {

    }
    pub fn virtual_run_transactions (&self) {

    }
}