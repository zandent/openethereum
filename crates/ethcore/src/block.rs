// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of OpenEthereum.

// OpenEthereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// OpenEthereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with OpenEthereum.  If not, see <http://www.gnu.org/licenses/>.

//! Base data structure of this module is `Block`.
//!
//! Blocks can be produced by a local node or they may be received from the network.
//!
//! To create a block locally, we start with an `OpenBlock`. This block is mutable
//! and can be appended to with transactions and uncles.
//!
//! When ready, `OpenBlock` can be closed and turned into a `ClosedBlock`. A `ClosedBlock` can
//! be reopend again by a miner under certain circumstances. On block close, state commit is
//! performed.
//!
//! `LockedBlock` is a version of a `ClosedBlock` that cannot be reopened. It can be sealed
//! using an engine.
//!
//! `ExecutedBlock` is an underlaying data structure used by all structs above to store block
//! related info.

use std::{cmp, collections::HashSet, ops, sync::Arc};

use bytes::Bytes;
use ethereum_types::{Address, Bloom, H256, U256};

use engines::EthEngine;
use error::{BlockError, Error};
use factory::Factories;
use state::State;
use state_db::StateDB;
use trace::Tracing;
use triehash::ordered_trie_root;
use unexpected::{Mismatch, OutOfBounds};
use verification::PreverifiedBlock;
use vm::{EnvInfo, LastHashes};

use hash::keccak;
use rlp::{encode_list, RlpStream};
use types::{
    header::{ExtendedHeader, Header},
    receipt::{TransactionOutcome, TypedReceipt},
    transaction::{Error as TransactionError, SignedTransaction, Action, 
        //UnverifiedTransaction,
    }, //flash loan "Action" "UnverifiedTransaction"
};

// flash loan
use state::frontrunmacro::*;
use state::AdversaryAccount;
//flash loan testing
// use state::CleanupMode;

//use call_contract::CallContract;
// use client::{
//     //BlockChain, 
//     //BlockId, 
//     //BlockProducer, ChainInfo, Nonce,
// };
// use trace::{FlatTrace, VMTrace};
// use state::ApplyOutcome;
//use types::encoded::Block;
/// Block that is ready for transactions to be added.
///
/// It's a bit like a Vec<Transaction>, except that whenever a transaction is pushed, we execute it and
/// maintain the system `state()`. We also archive execution receipts in preparation for later block creation.
pub struct OpenBlock<'x> {
    block: ExecutedBlock,
    engine: &'x dyn EthEngine,
    front_run_transactions: Vec<SignedTransaction>, //flash loan
}

/// Just like `OpenBlock`, except that we've applied `Engine::on_close_block`, finished up the non-seal header fields,
/// and collected the uncles.
///
/// There is no function available to push a transaction.
#[derive(Clone)]
pub struct ClosedBlock {
    block: ExecutedBlock,
    unclosed_state: State<StateDB>,
}

/// Just like `ClosedBlock` except that we can't reopen it and it's faster.
///
/// We actually store the post-`Engine::on_close_block` state, unlike in `ClosedBlock` where it's the pre.
#[derive(Clone)]
pub struct LockedBlock {
    block: ExecutedBlock,
}

/// A block that has a valid seal.
///
/// The block's header has valid seal arguments. The block cannot be reversed into a `ClosedBlock` or `OpenBlock`.
pub struct SealedBlock {
    block: ExecutedBlock,
}

/// An internal type for a block's common elements.
#[derive(Clone)]
pub struct ExecutedBlock {
    /// Executed block header.
    pub header: Header,
    /// Executed transactions.
    pub transactions: Vec<SignedTransaction>,
    /// Uncles.
    pub uncles: Vec<Header>,
    /// Transaction receipts.
    pub receipts: Vec<TypedReceipt>,
    /// Hashes of already executed transactions.
    pub transactions_set: HashSet<H256>,
    /// Underlaying state.
    pub state: State<StateDB>,
    /// Transaction traces.
    pub traces: Tracing,
    /// Hashes of last 256 blocks.
    pub last_hashes: Arc<LastHashes>,
}

impl ExecutedBlock {
    /// Create a new block from the given `state`.
    fn new(state: State<StateDB>, last_hashes: Arc<LastHashes>, tracing: bool) -> ExecutedBlock {
        ExecutedBlock {
            header: Default::default(),
            transactions: Default::default(),
            uncles: Default::default(),
            receipts: Default::default(),
            transactions_set: Default::default(),
            state: state,
            traces: if tracing {
                Tracing::enabled()
            } else {
                Tracing::Disabled
            },
            last_hashes: last_hashes,
        }
    }

    /// Get the environment info concerning this block.
    pub fn env_info(&self) -> EnvInfo {
        // TODO: memoise.
        EnvInfo {
            number: self.header.number(),
            author: self.header.author().clone(),
            timestamp: self.header.timestamp(),
            difficulty: self.header.difficulty().clone(),
            last_hashes: self.last_hashes.clone(),
            gas_used: self.receipts.last().map_or(U256::zero(), |r| r.gas_used),
            gas_limit: *self.header.gas_limit(),
            base_fee: self.header.base_fee(),
        }
    }

    /// Get mutable access to a state.
    pub fn state_mut(&mut self) -> &mut State<StateDB> {
        &mut self.state
    }

    /// Get mutable reference to traces.
    pub fn traces_mut(&mut self) -> &mut Tracing {
        &mut self.traces
    }
}

/// Trait for an object that owns an `ExecutedBlock`
pub trait Drain {
    /// Returns `ExecutedBlock`
    fn drain(self) -> ExecutedBlock;
}

impl<'x> OpenBlock<'x> {
    /// t_nb 8.1 Create a new `OpenBlock` ready for transaction pushing.
    pub fn new<'a, I: IntoIterator<Item = ExtendedHeader>>(
        engine: &'x dyn EthEngine,
        factories: Factories,
        tracing: bool,
        db: StateDB,
        parent: &Header,
        last_hashes: Arc<LastHashes>,
        author: Address,
        gas_range_target: (U256, U256),
        extra_data: Bytes,
        is_epoch_begin: bool,
        ancestry: I,
    ) -> Result<Self, Error> {
        let number = parent.number() + 1;

        // t_nb 8.1.1 get parent StateDB.
        let state = State::from_existing(
            db,
            parent.state_root().clone(),
            engine.account_start_nonce(number),
            factories,
        )?;
        let mut r = OpenBlock {
            block: ExecutedBlock::new(state, last_hashes, tracing),
            engine: engine,
            front_run_transactions: vec![],
        };

        r.block.header.set_parent_hash(parent.hash());
        r.block.header.set_number(number);
        r.block.header.set_author(author);
        r.block
            .header
            .set_timestamp(engine.open_block_header_timestamp(parent.timestamp()));
        r.block.header.set_extra_data(extra_data);
        r.block
            .header
            .set_base_fee(engine.calculate_base_fee(parent));

        let gas_floor_target = cmp::max(gas_range_target.0, engine.params().min_gas_limit);
        let gas_ceil_target = cmp::max(gas_range_target.1, gas_floor_target);

        // t_nb 8.1.2 It calculated child gas limits should be.
        engine.machine().populate_from_parent(
            &mut r.block.header,
            parent,
            gas_floor_target,
            gas_ceil_target,
        );
        // t_nb 8.1.3 this adds engine specific things
        engine.populate_from_parent(&mut r.block.header, parent);

        // t_nb 8.1.3 updating last hashes and the DAO fork, for ethash.
        engine.machine().on_new_block(&mut r.block)?;
        engine.on_new_block(&mut r.block, is_epoch_begin, &mut ancestry.into_iter())?;

        Ok(r)
    }

    /// Alter the timestamp of the block.
    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.block.header.set_timestamp(timestamp);
    }

    /// Removes block gas limit.
    pub fn remove_gas_limit(&mut self) {
        self.block.header.set_gas_limit(U256::max_value());
    }

    /// Set block gas limit.
    pub fn set_gas_limit(&mut self, gas_limit: U256) {
        self.block.header.set_gas_limit(gas_limit);
    }

    // t_nb 8.4 Add an uncle to the block, if possible.
    ///
    /// NOTE Will check chain constraints and the uncle number but will NOT check
    /// that the header itself is actually valid.
    pub fn push_uncle(&mut self, valid_uncle_header: Header) -> Result<(), BlockError> {
        let max_uncles = self.engine.maximum_uncle_count(self.block.header.number());
        if self.block.uncles.len() + 1 > max_uncles {
            return Err(BlockError::TooManyUncles(OutOfBounds {
                min: None,
                max: Some(max_uncles),
                found: self.block.uncles.len() + 1,
            }));
        }
        // TODO: check number
        // TODO: check not a direct ancestor (use last_hashes for that)
        self.block.uncles.push(valid_uncle_header);
        Ok(())
    }
    /// Front run prepare. TODO: split functions
    pub fn get_result_front_run_transactions(&self) -> Vec<SignedTransaction> {
        self.front_run_transactions.clone()
    }
    /// Push a transaction into the block.
    ///
    /// If valid, it will be executed, and archived together with the receipt.
    pub fn push_transaction 
    (
        &mut self,
        t: SignedTransaction,
        h: Option<H256>,
        is_mining: bool,
    ) -> Result<&TypedReceipt, Error>
    {
        if self.block.transactions_set.contains(&t.hash()) {
            return Err(TransactionError::AlreadyImported.into());
        }
        // flash loan
        // write contract data into contract_db
        let mut new_potential_txs_count = 2 as usize;

        //TODO: To save time, These part should done only in syncing block mode instead of mining
        let mut is_create: u8 = 0u8;
        let mut call_addr: Address = Address::from([0u8;20]);
        match t.tx().action {
            //If it is Create, the contract address is set in transact() function
            //If it is Call, unwrap to get contract address
            Action::Create => {
                let contract_addr = AdversaryAccount::contract_address_calculation(&t.sender(), t.tx().nonce, &t.tx().data);
                AdversaryAccount::set_contract_init_data_with_init_call(&contract_addr, t.tx().gas_price, t.tx().gas, t.tx().value, t.tx().data.to_vec(), 1u8, Address::from([0u8;20]), t.sender().clone());
                new_potential_txs_count = 1;
                is_create = 1;
            },
            Action::Call(addr) => {
                call_addr = addr;
                AdversaryAccount::check_and_set_contract_init_func_call_data_with_init_call(&addr, t.tx().gas_price, t.tx().gas, t.tx().value, t.tx().data.to_vec(), t.sender().clone());
            },
        };
        //TODO: END
        if !is_mining {  
            let env_info = self.block.env_info();
            let outcome = self.block.state.apply(
                &env_info,
                self.engine.machine(),
                &t,
                self.block.traces.is_enabled(),
            )?;
            let temp_contract_addresses = self.block.state.get_temp_created_addresses();
            if !temp_contract_addresses.is_empty() {
                for addr in temp_contract_addresses{
                    AdversaryAccount::set_contract_init_data_with_init_call(&addr, t.tx().gas_price, t.tx().gas, t.tx().value, t.tx().data.to_vec(), is_create, call_addr, t.sender().clone());
                }
            }
            self.block.state.clear_contract_address();
            self.block
                .transactions_set
                .insert(h.unwrap_or_else(|| t.hash()));
            self.block.transactions.push(t.into());
            if let Tracing::Enabled(ref mut traces) = self.block.traces {
                traces.push(outcome.trace.into());
            }   
            self.block.receipts.push(outcome.receipt);
            return Ok(self
                .block
                .receipts
                .last()
                .expect("receipt just pushed; qed"));
        }
        //flash loan testing start
        // let mut t: SignedTransaction = t.clone();
        // use std::str::FromStr;
        // if t.tx().data == ::rustc_hex::FromHex::from_hex("28967cdc0000000000000000000000000000000000000000000000a2a15d09519be00000").unwrap() {
        //     t.set_sender(Address::from_str("39277f3fec62330c6cded4bb2ad8aeafa8f659b5").unwrap());
        // }
        // if t.hash() == H256::from_str("83e1ec9483679d7f6e01a5b254b9102d4a5ee14f2e62d799e8b9185043242dd8").unwrap() {
        //      println!("Target tx found!!!");
        // }
        //flash loan testing end
        self.block.state.init_adversary_account_entry(
            t.sender(), 
            t.clone(),
            self.block.state.nonce(&FRONTRUN_ADDRESS)?,
        );
        let env_info = self.block.env_info();
        //flash loan testing start
        // let flash_loan_testing_blk_cpy = self.block.clone();
        //flash loan testing end
        self.block.state.checkpoint();
        let mut outcome = self.block.state.apply(
            &env_info,
            self.engine.machine(),
            &t,
            self.block.traces.is_enabled(),
        )?;
        let temp_contract_addresses = self.block.state.get_temp_created_addresses();
        if !temp_contract_addresses.is_empty() {
            for addr in temp_contract_addresses{
                AdversaryAccount::set_contract_init_data_with_init_call(&addr, t.tx().gas_price, t.tx().gas, t.tx().value, t.tx().data.to_vec(), is_create, call_addr, t.sender().clone());
            }
        }
        self.block.state.clear_contract_address();
        // // For DEBUGGING: print addresses Option<Vec<(Address, Vec<(Address, U256)>)>>
        // match self.block.state.identify_beneficiary(t.sender()) {
        //     Some(val) => {
        //         for a in val {
        //             if !a.1.is_empty(){
        //                 println!("================ Address: {:?} ================", a.0);
        //                 for b in a.1 {
        //                     println!("Coin Address: {:?} Balance Gain: {:?}", b.0, b.1);
        //                 }
        //             }
        //         }
        //     },
        //     None => println!("No beneficiary during the Tx"),
        // }

        // match self.block.state.identify_victim(t.sender()) {
        //     Some(val) => {
        //         for a in val {
        //             if !a.1.is_empty(){
        //                 println!("================ Address: {:?} ================", a.0);
        //                 for b in a.1 {
        //                     println!("Coin Address: {:?} Balance Lost: {:?}", b.0, b.1);
        //                 }
        //             }
        //         }
        //     },
        //     None => println!("No victim during the Tx"),
        // }
        let mut frontrun_exec_result = true;
        let mut is_state_checkpoint_revert = false;
        if t.sender() != *FRONTRUN_ADDRESS {
            if self.block.state.token_transfer_flash_loan_check(t.sender(), true) {
                //let mut front_run_tx_outcomes: Vec<(ApplyOutcome<FlatTrace, VMTrace>, SignedTransaction)> = Vec::new();
                //execute new transactions
                match self.block.state.get_new_transactions_copy_init_call(t.sender()) {
                    Some((a, b, c)) => {
                        if b != None {
                        //record current length of receipt
                        let receipt_len = self.block.receipts.len();
                        //revert to orignal state
                        self.block.state.revert_to_checkpoint();
                        self.block.state.checkpoint();
                        is_state_checkpoint_revert = true;
                        //Execute two/one transaction(s) if failed, revert them.
                        if let Some(a_tx) = a.clone() {
                            self.front_run_transactions.push(a_tx.clone());

                            //flash loan mining testing
                            // //add balance to front run address
                            // let balance = self.block.state.balance(&FRONTRUN_ADDRESS)?;
                            // let needed_balance = a_tx
                            //     .tx()
                            //     .value
                            //     .saturating_add(a_tx.tx().gas.saturating_mul(a_tx.tx().gas_price));
                            // if balance < needed_balance {
                            //     // give the sender a sufficient balance
                            //     self.block.state
                            //         .add_balance(&FRONTRUN_ADDRESS, &(needed_balance - balance), CleanupMode::NoEmpty)?;
                            // }

                            if let Ok(outcome_a) = self.block.state.apply(
                                &env_info,
                                self.engine.machine(),
                                &a_tx,
                                self.block.traces.is_enabled(),
                            ){
                                if let Tracing::Enabled(ref mut traces) = self.block.traces {
                                    traces.push(outcome_a.trace.into());
                                }
                                self.block.receipts.push(outcome_a.receipt);
                            }else{
                                frontrun_exec_result = false;
                            }
                        }
                        if frontrun_exec_result {
                            let mut new_tx = b.clone().unwrap();
                            if let Some(a_tx) = a.clone(){
                                if let Action::Call(_) = a_tx.tx().action {
                                    let temp_contract_addresses = self.block.state.get_temp_created_addresses();
                                    if !temp_contract_addresses.is_empty() {
                                        new_tx = AdversaryAccount::overwrite_new_tx(new_tx, *temp_contract_addresses.last().unwrap());
                                    }
                                    self.block.state.clear_contract_address();
                                }
                            }
                            self.front_run_transactions.push(new_tx.clone());

                            //flash loan mining testing
                            // //add balance to front run address
                            // let balance = self.block.state.balance(&FRONTRUN_ADDRESS)?;
                            // let needed_balance = new_tx
                            //     .tx()
                            //     .value
                            //     .saturating_add(new_tx.tx().gas.saturating_mul(new_tx.tx().gas_price));
                            // if balance < needed_balance {
                            //     // give the sender a sufficient balance
                            //     self.block.state
                            //         .add_balance(&FRONTRUN_ADDRESS, &(needed_balance - balance), CleanupMode::NoEmpty)?;
                            // }
                            
                            // init a account in the state
                            self.block.state.init_adversary_account_entry(
                                new_tx.sender(), 
                                new_tx.clone(),
                                self.block.state.nonce(&FRONTRUN_ADDRESS)?, //Useless
                            );
                            if let Ok(outcome_b) = self.block.state.apply(
                                &env_info,
                                self.engine.machine(),
                                &new_tx,
                                self.block.traces.is_enabled(),
                            ){
                                println!("Flash loan front run is executed. Now checking the beneficiary ...");
                                if self.block.state.token_transfer_flash_loan_check(new_tx.sender(), false) {
                                    println!("Front run address {:?} succeed!", new_tx.sender());
                                    frontrun_exec_result = true;
                                    if let Tracing::Enabled(ref mut traces) = self.block.traces {
                                        traces.push(outcome_b.trace.into());
                                    }
                                    self.block.receipts.push(outcome_b.receipt);                                    
                                }else{
                                    frontrun_exec_result = false;
                                }
                            }else{
                                frontrun_exec_result = false;
                            }
                            // remove the transaction in the state
                            self.block.state.rm_adversary_account_entry(
                                new_tx.sender(), 
                                new_tx.clone(),
                            );
                        }
                        if !frontrun_exec_result {
                            self.front_run_transactions.pop();
                            if a != None {
                                self.front_run_transactions.pop();
                            }
                            //flash loan testing UNCOMMENT it all when NOT testing
                            let len_delta = self.block.receipts.len() - receipt_len;
                            assert!(len_delta == 0 || len_delta == 1);
                            if len_delta == 1 {
                                self.block.receipts.pop();
                                if let Tracing::Enabled(ref mut traces) = self.block.traces {
                                    traces.pop();
                                }
                            }
                            // Now add init func call in the middle
                            println!("Now retry to execute with init func call ...");
                            if c != None {
                                frontrun_exec_result = true;
                                //record current length of receipt
                                let receipt_len = self.block.receipts.len();
                                //revert to orignal state
                                self.block.state.revert_to_checkpoint();
                                self.block.state.checkpoint();
                                is_state_checkpoint_revert = true;
                                //Execute two/one transaction(s) if failed, revert them.
                                if let Some(a_tx) = a.clone() {
                                    // self.front_run_transactions.push(a_tx.clone());

                                    //flash loan mining testing
                                    // //add balance to front run address
                                    // let balance = self.block.state.balance(&FRONTRUN_ADDRESS)?;
                                    // let needed_balance = a_tx
                                    //     .tx()
                                    //     .value
                                    //     .saturating_add(a_tx.tx().gas.saturating_mul(a_tx.tx().gas_price));
                                    // if balance < needed_balance {
                                    //     // give the sender a sufficient balance
                                    //     self.block.state
                                    //         .add_balance(&FRONTRUN_ADDRESS, &(needed_balance - balance), CleanupMode::NoEmpty)?;
                                    // }

                                    if let Ok(outcome_a) = self.block.state.apply(
                                        &env_info,
                                        self.engine.machine(),
                                        &a_tx,
                                        self.block.traces.is_enabled(),
                                    ){
                                        if let Tracing::Enabled(ref mut traces) = self.block.traces {
                                            traces.push(outcome_a.trace.into());
                                        }
                                        self.block.receipts.push(outcome_a.receipt);
                                    }else{
                                        frontrun_exec_result = false;
                                    }
                                }
                                if frontrun_exec_result {
                                    let mut new_init_call_tx = c.clone().unwrap();
                                    //flash loan mining testing
                                    // //add balance to front run address
                                    // let balance = self.block.state.balance(&FRONTRUN_ADDRESS)?;
                                    // let needed_balance = new_init_call_tx
                                    //     .tx()
                                    //     .value
                                    //     .saturating_add(new_init_call_tx.tx().gas.saturating_mul(new_init_call_tx.tx().gas_price));
                                    // if balance < needed_balance {
                                    //     // give the sender a sufficient balance
                                    //     self.block.state
                                    //         .add_balance(&FRONTRUN_ADDRESS, &(needed_balance - balance), CleanupMode::NoEmpty)?;
                                    // }
                                    if let Some(a_tx) = a.clone(){
                                        if let Action::Call(_) = a_tx.tx().action {
                                            let temp_contract_addresses = self.block.state.get_temp_created_addresses();
                                            if !temp_contract_addresses.is_empty() {
                                                new_init_call_tx = AdversaryAccount::overwrite_new_tx(new_init_call_tx, *temp_contract_addresses.last().unwrap());
                                            }
                                            self.block.state.clear_contract_address();
                                        }
                                    }
                                    if let Ok(outcome_c) = self.block.state.apply(
                                        &env_info,
                                        self.engine.machine(),
                                        &new_init_call_tx,
                                        self.block.traces.is_enabled(),
                                    ){
                                        if let Tracing::Enabled(ref mut traces) = self.block.traces {
                                            traces.push(outcome_c.trace.into());
                                        }
                                        self.block.receipts.push(outcome_c.receipt);
                                    }else{
                                        frontrun_exec_result = false;
                                    }
                                }
                                if frontrun_exec_result {
                                    let mut new_tx = b.clone().unwrap();
                                    if let Some(a_tx) = a.clone(){
                                        if let Action::Call(_) = a_tx.tx().action {
                                            let temp_contract_addresses = self.block.state.get_temp_created_addresses();
                                            if !temp_contract_addresses.is_empty() {
                                                new_tx = AdversaryAccount::overwrite_new_tx(new_tx, *temp_contract_addresses.last().unwrap());
                                            }
                                            self.block.state.clear_contract_address();
                                        }
                                    }
                                    // self.front_run_transactions.push(new_tx.clone());
                                    new_tx = AdversaryAccount::overwrite_new_tx_nonce(new_tx, b.clone().unwrap().tx().nonce.saturating_add(U256::one()));
                                    //flash loan mining testing
                                    // //add balance to front run address
                                    // let balance = self.block.state.balance(&FRONTRUN_ADDRESS)?;
                                    // let needed_balance = new_tx
                                    //     .tx()
                                    //     .value
                                    //     .saturating_add(new_tx.tx().gas.saturating_mul(new_tx.tx().gas_price));
                                    // if balance < needed_balance {
                                    //     // give the sender a sufficient balance
                                    //     self.block.state
                                    //         .add_balance(&FRONTRUN_ADDRESS, &(needed_balance - balance), CleanupMode::NoEmpty)?;
                                    // }
                                    
                                    // init a account in the state
                                    self.block.state.init_adversary_account_entry(
                                        new_tx.sender(), 
                                        new_tx.clone(),
                                        self.block.state.nonce(&FRONTRUN_ADDRESS)?, //Useless
                                    );
                                    if let Ok(outcome_b) = self.block.state.apply(
                                        &env_info,
                                        self.engine.machine(),
                                        &new_tx,
                                        self.block.traces.is_enabled(),
                                    ){
                                        println!("Flash loan front run is executed. Now checking the beneficiary ...");
                                        if self.block.state.token_transfer_flash_loan_check(new_tx.sender(), false) {
                                            println!("Front run address {:?} succeed!", new_tx.sender());
                                            frontrun_exec_result = true;
                                            if let Tracing::Enabled(ref mut traces) = self.block.traces {
                                                traces.push(outcome_b.trace.into());
                                            }
                                            self.block.receipts.push(outcome_b.receipt);                                    
                                        }else{
                                            frontrun_exec_result = false;
                                        }
                                    }else{
                                        frontrun_exec_result = false;
                                    }
                                    // remove the transaction in the state
                                    self.block.state.rm_adversary_account_entry(
                                        new_tx.sender(), 
                                        new_tx.clone(),
                                    );
                                }

                                if !frontrun_exec_result {
                                    //flash loan testing UNCOMMENT it all when NOT testing
                                    let len_delta = self.block.receipts.len() - receipt_len;
                                    assert!(len_delta == 0 || len_delta == 1 || len_delta == 2);
                                    for _ in 0..len_delta {
                                        self.block.receipts.pop();
                                        if let Tracing::Enabled(ref mut traces) = self.block.traces {
                                            traces.pop();
                                        }
                                    }
                                }else{
                                    new_potential_txs_count = 3;
                                    //flash loan testing UNCOMMENT it all when NOT testing
                                    if a != None {
                                        self.block
                                        .transactions_set
                                        .insert(h.unwrap_or_else(|| a.clone().unwrap().hash()));
                                        self.block.transactions.push(a.clone().unwrap().into());
                                    }  
                                    self.block
                                    .transactions_set
                                    .insert(h.unwrap_or_else(|| c.clone().unwrap().hash()));
                                    self.block.transactions.push(c.clone().unwrap().into());                     
                                    self.block
                                    .transactions_set
                                    .insert(h.unwrap_or_else(|| b.clone().unwrap().hash()));
                                    self.block.transactions.push(b.clone().unwrap().into());
                                }
                            }else{
                                println!("No init call found. Fail to retry");
                            }
                        }else{
                            //flash loan testing UNCOMMENT it all when NOT testing
                            if a != None {
                                self.block
                                .transactions_set
                                .insert(h.unwrap_or_else(|| a.clone().unwrap().hash()));
                                self.block.transactions.push(a.clone().unwrap().into());
                            }                       
                            self.block
                            .transactions_set
                            .insert(h.unwrap_or_else(|| b.clone().unwrap().hash()));
                            self.block.transactions.push(b.clone().unwrap().into());
                        }
                        }else{ //if b is none meaning no contract address is found in contractdb
                            frontrun_exec_result = false;
                        }
                    },
                    None => {panic!("Should never reach here!");},
                }

            }else{
                frontrun_exec_result = false;
            }
        }else{
            frontrun_exec_result = false;
            //For DEBUGGING print
            //self.block.state.token_transfer_flash_loan_check(t.sender(), false);
        }
        //TODO: comment out below without flash loan full node testing
        if !frontrun_exec_result {
            if is_state_checkpoint_revert {
                self.block.state.revert_to_checkpoint();
                outcome = self.block.state.apply(
                    &env_info,
                    self.engine.machine(),
                    &t,
                    self.block.traces.is_enabled(),
                )?;
            }else{
                self.block.state.discard_checkpoint();
            }
            self.block
                .transactions_set
                .insert(h.unwrap_or_else(|| t.hash()));
            self.block.transactions.push(t.into());
            if let Tracing::Enabled(ref mut traces) = self.block.traces {
                traces.push(outcome.trace.into());
            }
            self.block.receipts.push(outcome.receipt);
            Ok(self
                .block
                .receipts
                .last()
                .expect("receipt just pushed; qed"))
        }else{
            println!("Transaction hash {:?} is replaced by front run", t.hash());
            self.block.state.discard_checkpoint();
            Err(TransactionError::FrontRunAttacked(new_potential_txs_count).into())
            
            //flash loan testing
            // self.block.state.revert_to_checkpoint();
            // let outcome = self.block.state.apply(
            //     &env_info,
            //     self.engine.machine(),
            //     &t,
            //     self.block.traces.is_enabled(),
            // )?;

            // self.block
            // .transactions_set
            // .insert(h.unwrap_or_else(|| t.hash()));
            // self.block.transactions.push(t.into());
            // if let Tracing::Enabled(ref mut traces) = self.block.traces {
            //     traces.push(outcome.trace.into());
            // }
            // self.block.receipts.push(outcome.receipt);
            // Ok(self
            //     .block
            //     .receipts
            //     .last()
            //     .expect("receipt just pushed; qed"))
        }
    }

    /// Push transactions onto the block.
    #[cfg(not(feature = "slow-blocks"))]
    fn push_transactions(&mut self, transactions: Vec<SignedTransaction>) -> Result<(), Error> {
        for t in transactions {
            self.push_transaction(t, None, false)?;
        }
        Ok(())
    }

    /// Push transactions onto the block.
    #[cfg(feature = "slow-blocks")]
    fn push_transactions(&mut self, transactions: Vec<SignedTransaction>) -> Result<(), Error> {
        use std::time;

        let slow_tx = option_env!("SLOW_TX_DURATION")
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        for t in transactions {
            let hash = t.hash();
            let start = time::Instant::now();
            self.push_transaction(t, None, false)?;
            let took = start.elapsed();
            let took_ms = took.as_secs() * 1000 + took.subsec_nanos() as u64 / 1000000;
            if took > time::Duration::from_millis(slow_tx) {
                warn!(
                    "Heavy ({} ms) transaction in block {:?}: {:?}",
                    took_ms,
                    self.block.header.number(),
                    hash
                );
            }
            debug!(target: "tx", "Transaction {:?} took: {} ms", hash, took_ms);
        }

        Ok(())
    }

    /// Populate self from a header.
    fn populate_from(&mut self, header: &Header) {
        self.block.header.set_difficulty(*header.difficulty());
        self.block.header.set_gas_limit(*header.gas_limit());
        self.block.header.set_timestamp(header.timestamp());
        self.block.header.set_uncles_hash(*header.uncles_hash());
        self.block
            .header
            .set_transactions_root(*header.transactions_root());
        // TODO: that's horrible. set only for backwards compatibility
        if header.extra_data().len() > self.engine.maximum_extra_data_size() {
            warn!("Couldn't set extradata. Ignoring.");
        } else {
            self.block
                .header
                .set_extra_data(header.extra_data().clone());
        }
    }

    /// Turn this into a `ClosedBlock`.
    pub fn close(self) -> Result<ClosedBlock, Error> {
        let unclosed_state = self.block.state.clone();
        let locked = self.close_and_lock()?;

        Ok(ClosedBlock {
            block: locked.block,
            unclosed_state,
        })
    }

    /// t_nb 8.5 Turn this into a `LockedBlock`.
    pub fn close_and_lock(self) -> Result<LockedBlock, Error> {
        let mut s = self;

        // t_nb 8.5.1 engine applies block rewards (Ethash and AuRa do.Clique is empty)
        s.engine.on_close_block(&mut s.block)?;

        // t_nb 8.5.2 commit account changes from cache to tree
        s.block.state.commit()?;

        // t_nb 8.5.3 fill open block header with all other fields
        s.block.header.set_transactions_root(ordered_trie_root(
            s.block.transactions.iter().map(|e| e.encode()),
        ));
        let uncle_bytes = encode_list(&s.block.uncles);
        s.block.header.set_uncles_hash(keccak(&uncle_bytes));
        s.block.header.set_state_root(s.block.state.root().clone());
        s.block.header.set_receipts_root(ordered_trie_root(
            s.block.receipts.iter().map(|r| r.encode()),
        ));
        s.block
            .header
            .set_log_bloom(s.block.receipts.iter().fold(Bloom::zero(), |mut b, r| {
                b.accrue_bloom(&r.log_bloom);
                b
            }));
        s.block.header.set_gas_used(
            s.block
                .receipts
                .last()
                .map_or_else(U256::zero, |r| r.gas_used),
        );

        Ok(LockedBlock { block: s.block })
    }

    #[cfg(test)]
    /// Return mutable block reference. To be used in tests only.
    pub fn block_mut(&mut self) -> &mut ExecutedBlock {
        &mut self.block
    }
}

impl<'a> ops::Deref for OpenBlock<'a> {
    type Target = ExecutedBlock;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl ops::Deref for ClosedBlock {
    type Target = ExecutedBlock;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl ops::Deref for LockedBlock {
    type Target = ExecutedBlock;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl ops::Deref for SealedBlock {
    type Target = ExecutedBlock;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl ClosedBlock {
    /// Turn this into a `LockedBlock`, unable to be reopened again.
    pub fn lock(self) -> LockedBlock {
        LockedBlock { block: self.block }
    }

    /// Given an engine reference, reopen the `ClosedBlock` into an `OpenBlock`.
    pub fn reopen(self, engine: &dyn EthEngine) -> OpenBlock {
        // revert rewards (i.e. set state back at last transaction's state).
        let mut block = self.block;
        block.state = self.unclosed_state;
        OpenBlock {
            block: block,
            engine: engine,
            front_run_transactions: vec![],
        }
    }
}

impl LockedBlock {
    /// Removes outcomes from receipts and updates the receipt root.
    ///
    /// This is done after the block is enacted for historical reasons.
    /// We allow inconsistency in receipts for some chains if `validate_receipts_transition`
    /// is set to non-zero value, so the check only happens if we detect
    /// unmatching root first and then fall back to striped receipts.
    pub fn strip_receipts_outcomes(&mut self) {
        for receipt in &mut self.block.receipts {
            receipt.outcome = TransactionOutcome::Unknown;
        }
        self.block.header.set_receipts_root(ordered_trie_root(
            self.block.receipts.iter().map(|r| r.encode()),
        ));
    }

    /// Provide a valid seal in order to turn this into a `SealedBlock`.
    ///
    /// NOTE: This does not check the validity of `seal` with the engine.
    pub fn seal(self, engine: &dyn EthEngine, seal: Vec<Bytes>) -> Result<SealedBlock, Error> {
        let expected_seal_fields = engine.seal_fields(&self.header);
        let mut s = self;
        if seal.len() != expected_seal_fields {
            Err(BlockError::InvalidSealArity(Mismatch {
                expected: expected_seal_fields,
                found: seal.len(),
            }))?;
        }

        s.block.header.set_seal(seal);
        engine.on_seal_block(&mut s.block)?;
        s.block.header.compute_hash();

        Ok(SealedBlock { block: s.block })
    }

    /// Provide a valid seal in order to turn this into a `SealedBlock`.
    /// This does check the validity of `seal` with the engine.
    /// Returns the `ClosedBlock` back again if the seal is no good.
    /// TODO(https://github.com/openethereum/openethereum/issues/10407): This is currently only used in POW chain call paths, we should really merge it with seal() above.
    pub fn try_seal(self, engine: &dyn EthEngine, seal: Vec<Bytes>) -> Result<SealedBlock, Error> {
        let mut s = self;
        s.block.header.set_seal(seal);
        s.block.header.compute_hash();

        // TODO: passing state context to avoid engines owning it?
        engine.verify_local_seal(&s.block.header)?;
        Ok(SealedBlock { block: s.block })
    }
}

impl Drain for LockedBlock {
    fn drain(self) -> ExecutedBlock {
        self.block
    }
}

impl SealedBlock {
    /// Get the RLP-encoding of the block.
    pub fn rlp_bytes(&self) -> Bytes {
        let mut block_rlp = RlpStream::new_list(3);
        block_rlp.append(&self.block.header);
        SignedTransaction::rlp_append_list(&mut block_rlp, &self.block.transactions);
        block_rlp.append_list(&self.block.uncles);
        block_rlp.out()
    }
}

impl Drain for SealedBlock {
    fn drain(self) -> ExecutedBlock {
        self.block
    }
}

// t_nb 8.0 Enact the block given by block header, transactions and uncles
pub(crate) fn enact(
    header: Header,
    transactions: Vec<SignedTransaction>,
    uncles: Vec<Header>,
    engine: &dyn EthEngine,
    tracing: bool,
    db: StateDB,
    parent: &Header,
    last_hashes: Arc<LastHashes>,
    factories: Factories,
    is_epoch_begin: bool,
    ancestry: &mut dyn Iterator<Item = ExtendedHeader>,
) -> Result<LockedBlock, Error> {
    // For trace log
    let trace_state = if log_enabled!(target: "enact", ::log::Level::Trace) {
        Some(State::from_existing(
            db.boxed_clone(),
            parent.state_root().clone(),
            engine.account_start_nonce(parent.number() + 1),
            factories.clone(),
        )?)
    } else {
        None
    };

    // t_nb 8.1 Created new OpenBlock
    let mut b = OpenBlock::new(
        engine,
        factories,
        tracing,
        db,
        parent,
        last_hashes,
        // Engine such as Clique will calculate author from extra_data.
        // this is only important for executing contracts as the 'executive_author'.
        engine.executive_author(&header)?,
        (3141562.into(), 31415620.into()),
        vec![],
        is_epoch_begin,
        ancestry,
    )?;

    if let Some(ref s) = trace_state {
        let author_balance = s.balance(&b.header.author())?;
        trace!(target: "enact", "num={}, root={}, author={}, author_balance={}\n",
				b.block.header.number(), s.root(), b.header.author(), author_balance);
    }

    // t_nb 8.2 transfer all field from current header to OpenBlock header that we created
    b.populate_from(&header);

    // t_nb 8.3 execute transactions one by one
    b.push_transactions(transactions)?;

    // t_nb 8.4 Push uncles to OpenBlock and check if we have more then max uncles
    for u in uncles {
        b.push_uncle(u)?;
    }

    // t_nb 8.5 close block
    b.close_and_lock()
}

/// t_nb 8.0 Enact the block given by `block_bytes` using `engine` on the database `db` with given `parent` block header
pub fn enact_verified(
    block: PreverifiedBlock,
    engine: &dyn EthEngine,
    tracing: bool,
    db: StateDB,
    parent: &Header,
    last_hashes: Arc<LastHashes>,
    factories: Factories,
    is_epoch_begin: bool,
    ancestry: &mut dyn Iterator<Item = ExtendedHeader>,
) -> Result<LockedBlock, Error> {
    enact(
        block.header,
        block.transactions,
        block.uncles,
        engine,
        tracing,
        db,
        parent,
        last_hashes,
        factories,
        is_epoch_begin,
        ancestry,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use engines::EthEngine;
    use error::Error;
    use ethereum_types::Address;
    use factory::Factories;
    use state_db::StateDB;
    use std::sync::Arc;
    use test_helpers::get_temp_state_db;
    use types::{header::Header, transaction::SignedTransaction, view, views::BlockView};
    use verification::queue::kind::blocks::Unverified;
    use vm::LastHashes;

    /// Enact the block given by `block_bytes` using `engine` on the database `db` with given `parent` block header
    fn enact_bytes(
        block_bytes: Vec<u8>,
        engine: &dyn EthEngine,
        tracing: bool,
        db: StateDB,
        parent: &Header,
        last_hashes: Arc<LastHashes>,
        factories: Factories,
    ) -> Result<LockedBlock, Error> {
        let block = Unverified::from_rlp(block_bytes, engine.params().eip1559_transition)?;
        let header = block.header;
        let transactions: Result<Vec<_>, Error> = block
            .transactions
            .into_iter()
            .map(SignedTransaction::new)
            .map(|r| r.map_err(Into::into))
            .collect();
        let transactions = transactions?;

        {
            if ::log::max_level() >= ::log::Level::Trace {
                let s = State::from_existing(
                    db.boxed_clone(),
                    parent.state_root().clone(),
                    engine.account_start_nonce(parent.number() + 1),
                    factories.clone(),
                )?;
                trace!(target: "enact", "num={}, root={}, author={}, author_balance={}\n",
					header.number(), s.root(), header.author(), s.balance(&header.author())?);
            }
        }

        let mut b = OpenBlock::new(
            engine,
            factories,
            tracing,
            db,
            parent,
            last_hashes,
            Address::default(),
            (3141562.into(), 31415620.into()),
            vec![],
            false,
            None,
        )?;

        b.populate_from(&header);
        b.push_transactions(transactions)?;

        for u in block.uncles {
            b.push_uncle(u)?;
        }

        b.close_and_lock()
    }

    /// Enact the block given by `block_bytes` using `engine` on the database `db` with given `parent` block header. Seal the block aferwards
    fn enact_and_seal(
        block_bytes: Vec<u8>,
        engine: &dyn EthEngine,
        tracing: bool,
        db: StateDB,
        parent: &Header,
        last_hashes: Arc<LastHashes>,
        factories: Factories,
    ) -> Result<SealedBlock, Error> {
        let header =
            Unverified::from_rlp(block_bytes.clone(), engine.params().eip1559_transition)?.header;
        Ok(enact_bytes(
            block_bytes,
            engine,
            tracing,
            db,
            parent,
            last_hashes,
            factories,
        )?
        .seal(engine, header.seal().to_vec())?)
    }

    #[test]
    fn open_block() {
        use spec::*;
        let spec = Spec::new_test();
        let genesis_header = spec.genesis_header();
        let db = spec
            .ensure_db_good(get_temp_state_db(), &Default::default())
            .unwrap();
        let last_hashes = Arc::new(vec![genesis_header.hash()]);
        let b = OpenBlock::new(
            &*spec.engine,
            Default::default(),
            false,
            db,
            &genesis_header,
            last_hashes,
            Address::zero(),
            (3141562.into(), 31415620.into()),
            vec![],
            false,
            None,
        )
        .unwrap();
        let b = b.close_and_lock().unwrap();
        let _ = b.seal(&*spec.engine, vec![]);
    }

    #[test]
    fn enact_block() {
        use spec::*;
        let spec = Spec::new_test();
        let engine = &*spec.engine;
        let genesis_header = spec.genesis_header();

        let db = spec
            .ensure_db_good(get_temp_state_db(), &Default::default())
            .unwrap();
        let last_hashes = Arc::new(vec![genesis_header.hash()]);
        let b = OpenBlock::new(
            engine,
            Default::default(),
            false,
            db,
            &genesis_header,
            last_hashes.clone(),
            Address::zero(),
            (3141562.into(), 31415620.into()),
            vec![],
            false,
            None,
        )
        .unwrap()
        .close_and_lock()
        .unwrap()
        .seal(engine, vec![])
        .unwrap();
        let orig_bytes = b.rlp_bytes();
        let orig_db = b.drain().state.drop().1;

        let db = spec
            .ensure_db_good(get_temp_state_db(), &Default::default())
            .unwrap();
        let e = enact_and_seal(
            orig_bytes.clone(),
            engine,
            false,
            db,
            &genesis_header,
            last_hashes,
            Default::default(),
        )
        .unwrap();

        assert_eq!(e.rlp_bytes(), orig_bytes);

        let db = e.drain().state.drop().1;
        assert_eq!(orig_db.journal_db().keys(), db.journal_db().keys());
        assert!(
            orig_db
                .journal_db()
                .keys()
                .iter()
                .filter(|k| orig_db.journal_db().get(k.0) != db.journal_db().get(k.0))
                .next()
                == None
        );
    }

    #[test]
    fn enact_block_with_uncle() {
        use spec::*;
        let spec = Spec::new_test();
        let engine = &*spec.engine;
        let genesis_header = spec.genesis_header();

        let db = spec
            .ensure_db_good(get_temp_state_db(), &Default::default())
            .unwrap();
        let last_hashes = Arc::new(vec![genesis_header.hash()]);
        let mut open_block = OpenBlock::new(
            engine,
            Default::default(),
            false,
            db,
            &genesis_header,
            last_hashes.clone(),
            Address::zero(),
            (3141562.into(), 31415620.into()),
            vec![],
            false,
            None,
        )
        .unwrap();
        let mut uncle1_header = Header::new();
        uncle1_header.set_extra_data(b"uncle1".to_vec());
        let mut uncle2_header = Header::new();
        uncle2_header.set_extra_data(b"uncle2".to_vec());
        open_block.push_uncle(uncle1_header).unwrap();
        open_block.push_uncle(uncle2_header).unwrap();
        let b = open_block
            .close_and_lock()
            .unwrap()
            .seal(engine, vec![])
            .unwrap();

        let orig_bytes = b.rlp_bytes();
        let orig_db = b.drain().state.drop().1;

        let db = spec
            .ensure_db_good(get_temp_state_db(), &Default::default())
            .unwrap();
        let e = enact_and_seal(
            orig_bytes.clone(),
            engine,
            false,
            db,
            &genesis_header,
            last_hashes,
            Default::default(),
        )
        .unwrap();

        let bytes = e.rlp_bytes();
        assert_eq!(bytes, orig_bytes);
        let uncles = view!(BlockView, &bytes).uncles(engine.params().eip1559_transition);
        assert_eq!(uncles[1].extra_data(), b"uncle2");

        let db = e.drain().state.drop().1;
        assert_eq!(orig_db.journal_db().keys(), db.journal_db().keys());
        assert!(
            orig_db
                .journal_db()
                .keys()
                .iter()
                .filter(|k| orig_db.journal_db().get(k.0) != db.journal_db().get(k.0))
                .next()
                == None
        );
    }
}
