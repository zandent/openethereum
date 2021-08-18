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
#[doc(hidden)]
pub struct AdversaryAccount {
    // the balance before transaction, the address is any addr found during transaction
    init_balances: RefCell<HashMap<Address, U256>>,
    // final balance after transaction
    final_balances: RefCell<HashMap<Address, U256>>,
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
#[doc(hidden)]
impl AdversaryAccount {
    pub fn new(n: U256, t: SignedTransaction, m_n: U256) -> Self {
        let ret = AdversaryAccount {
            init_balances: Default::default(),
            final_balances: Default::default(),
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
    pub fn lookup_init_balance(&self, addr: Address) -> Option<U256> {
        self.init_balances.borrow_mut().get_mut(&addr).map(|value| value.clone())
    }
    pub fn lookup_final_balance(&self, addr: Address) -> Option<U256> {
        self.final_balances.borrow_mut().get_mut(&addr).map(|value| value.clone())
    }
    pub fn set_init_balance(&self, addr: Address, bal: U256) -> Option<U256> {
        match self.lookup_init_balance(addr) {
            Some(_) => None,
            None => {
                self.init_balances.borrow_mut().insert(addr, bal);
                Some(bal)
                    },
        }
    }
    pub fn set_final_balance(&self, addr: Address, bal: U256) -> Option<U256> {
        self.final_balances.borrow_mut().insert(addr, bal);
        Some(bal)
    }
    pub fn set_balance(&self, addr: Address, bal: U256) -> Option<U256> {
        match self.lookup_init_balance(addr) {
            Some(_) => {
                self.final_balances.borrow_mut().insert(addr, bal);
                Some(bal)
            },
            None => {
                self.init_balances.borrow_mut().insert(addr, bal);
                Some(bal)
            },
        }
    }
    pub fn identify_beneficiary (&self) -> Option<Vec<(Address, U256)>> {
        let mut ret = Vec::new();
        for (a, b) in self.init_balances.borrow_mut().iter() {
            for (c, d) in self.final_balances.borrow_mut().iter() {
                if a==c && b < d {
                    ret.push((Address::from(*a), d.saturating_sub(*b)));
                }
            }
        } 
        match ret.is_empty() {
            true => None,
            false => Some(ret),
        }  
    }
    pub fn identify_victim (&self) -> Option<Vec<(Address, U256)>> {
        let mut ret = Vec::new();
        for (a, b) in self.init_balances.borrow_mut().iter() {
            for (c, d) in self.final_balances.borrow_mut().iter() {
                if a==c && b > d {
                    ret.push((Address::from(*a), b.saturating_sub(*d)));
                }
            }
        } 
        match ret.is_empty() {
            true => None,
            false => Some(ret),
        }  
    }
}