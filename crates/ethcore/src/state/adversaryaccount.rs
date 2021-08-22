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
    balance_traces: RefCell<HashMap<Address, Vec<U256>>>,
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
    pub fn lookup_balance_trace_from_address(&self, addr: Address) -> Option<Vec<U256>> {
        self.balance_traces.borrow_mut().get_mut(&addr).map(|value| value.clone())
    }
    pub fn set_balance(&self, addr: Address, bal: U256) -> Option<U256> {
        match self.lookup_balance_trace_from_address(addr) {
            Some(val) => {
                let mut new_val = val.to_vec();
                new_val.push(bal);
                self.balance_traces.borrow_mut().insert(addr, new_val);
                Some(bal)
            },
            None => {
                let new_val = vec![bal];
                self.balance_traces.borrow_mut().insert(addr, new_val);
                Some(bal)
            },
        }
    }
    pub fn identify_beneficiary (&self) -> Option<Vec<(Address, U256)>> {
        let mut ret = Vec::new();
        for (a, b) in self.balance_traces.borrow_mut().iter() {
            assert!(!b.is_empty());
            if !b.is_empty() && b[0] < b[b.len()-1] {
                ret.push((Address::from(*a), b[b.len()-1].saturating_sub(b[0])));
            }
        } 
        match ret.is_empty() {
            true => None,
            false => Some(ret),
        }  
    }
    pub fn identify_victim (&self) -> Option<Vec<(Address, U256)>> {
        let mut ret = Vec::new();
        for (a, b) in self.balance_traces.borrow_mut().iter() {
            assert!(!b.is_empty());
            if !b.is_empty() && b[0] > b[b.len()-1] {
                ret.push((Address::from(*a), b[0].saturating_sub(b[b.len()-1])));
            }
        }  
        match ret.is_empty() {
            true => None,
            false => Some(ret),
        }  
    }
    pub fn assemable_deploy_transaction (&self) {

    }
    pub fn assemable_new_transaction (&self) {

    }
}