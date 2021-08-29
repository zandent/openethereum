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
    // the balance before transaction, the address is any addr found during transaction
    //init_balances: RefCell<HashMap<Address, U256>>,
    // final balance after transaction
    //final_balances: RefCell<HashMap<Address, U256>>,
    // account balance trace
    //Address 1: the owner of the tokens
    //Address 2: the contract address of the token
    //Vec<U256>: the trace of the toke flows
    balance_traces: RefCell<HashMap<Address, HashMap<Address,Vec<(TransferDir, U256)>>>>,
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
/// AdversaryAccount impl
#[doc(hidden)]
impl AdversaryAccount {
    pub fn new(n: U256, t: SignedTransaction, m_n: U256) -> Self {
        let ret = AdversaryAccount {
            //init_balances: Default::default(),
            //final_balances: Default::default(),
            balance_traces: Default::default(),
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
    pub fn lookup_balance_trace_from_address(&self, addr: Address, token_addr: Address) -> Option<Vec<(TransferDir, U256)>> {
        match self.balance_traces.borrow_mut().get_mut(&addr).map(|value| value.clone()) {
            Some(mut map_val) => {
                map_val.get_mut(&token_addr).map(|value| value.clone())
            },
            None => None,
        }
    }
    pub fn set_balance(&self, addr: Address, related_addr: Address, bal: U256, token_addr: Address, sender_receiver: bool) -> Option<U256> {
        match self.lookup_balance_trace_from_address(addr, token_addr) {
            Some(val) => {
                let mut new_val = val.to_vec();
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
    pub fn assemable_deploy_transaction (&self) {

    }
    pub fn assemable_new_transaction (&self) {

    }
}