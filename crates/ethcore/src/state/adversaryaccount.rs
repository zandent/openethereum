//use transaction_ext::Transaction;
use types::transaction::{Transaction as RawTransaction, Action, SignedTransaction, TypedTransaction, 
    //UnverifiedTransaction,
};
use ethereum_types::{
    Address, 
    //Bloom, 
    //H256, 
    U256};
//use bytes::Bytes;
use std::{
    cell::{
        RefCell, 
        //RefMut
        },
    collections::{HashMap},
    //sync::Arc,
};
pub use state::frontrunmacro::*;
use executive::contract_address;
use vm::CreateContractAddress;
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
    //old_tx contract address. It is set when init if old_tx is Call. Set after executing if old_tx is Create
    old_tx_contract_address: RefCell<Option<Address>>,
    // Nonce of Adversary account.
    nonce: U256,
    //code to deploy
    //code: Option<Arc<Bytes>>,
    //data which does function call
    //data: Bytes,
    // Nonce of my account,
    my_nonce: U256,
    // new deploy transaction, NOTICE that it may be None if old_tx is deploy tx
    new_deploy_tx: RefCell<Option<SignedTransaction>>,
    // new flash loan transaction, NOTICE that it may be also a deploy transaction if old_tx is deploy tx
    new_tx: RefCell<Option<SignedTransaction>>,
    //temp contract addresses created in this transcation
    temp_contract_addresses: RefCell<Vec<Address>>,
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
        let old_tx_contract_address = match t.tx().action {
            //If it is Create, the contract address is set in transact() function
            //If it is Call, unwrap to get contract address
            Action::Create => None,
            Action::Call(ref address) => Some(*address),
        };
        let ret = AdversaryAccount {
            balance_traces: Default::default(),
            transfer_in_order: Default::default(),
            flash_loan_information: Default::default(),
            old_tx: t.clone(),
            old_tx_contract_address: RefCell::new(old_tx_contract_address),
            nonce: n,
            my_nonce: m_n,
            new_deploy_tx: Default::default(),
            new_tx: Default::default(),   
            temp_contract_addresses: Default::default(),       
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
        match ERC_TOKEN_INFORMATION_MAP.get(&token_addr).map(|value| value.clone()) {
        // match CONTRACT_ADDRESSES.iter().position(|val| *val == token_addr) {
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
                        match ERC_TOKEN_INFORMATION_MAP.get(a).map(|value| value.clone()) {
                            Some((price, decimal)) => {
                                let net_value = c.saturating_mul(price).checked_div(decimal).unwrap();
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
    //For DEBUGGING: print out address who gains token
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
    //For DEBUGGING: print out address who loses token
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
        let mut flash_loan_start_return = Vec::new();
        let mut flash_loan_end_return = Vec::new();
        let transfers = self.transfer_in_order.borrow_mut();
        for (from, to, amt, token) in transfers.iter() {
            match FLASH_LOAN_CONTRACT_ADDRESSES.iter().position(|val| *val == *from) {
                Some(_) => {flash_loan_start.push((*from, *to, *amt, *token));},
                None => (),
            }
            match FLASH_LOAN_CONTRACT_ADDRESSES.iter().position(|val| *val == *to) {
                Some(_) => {
                    if let Some(idx) = flash_loan_start.iter().position(|i| i.0==*to && i.2<=*amt && i.3==*token){
                        flash_loan_start_return.push(flash_loan_start[idx]);
                        flash_loan_end_return.push((*from, *to, *amt, *token));
                    }
                },
                None => (),
            }
        }
        if flash_loan_end_return.is_empty() {
            None
        }else{
            Some((flash_loan_start_return, flash_loan_end_return))
        }
    }
    //If a flash loan attack is detected, return true. Otherwise, false.
    pub fn token_transfer_flash_loan_check (&self, assemable_new: bool) -> bool{
        //TODO: Check the transaction is successfully executed
        //Check the address is beneficiary after all or not. (Pull down current price of each token)
        //Check who is victim
        self.anaylsis_net_profit_in_one_thousandth_usd();
        let mut beneficiary: Vec<Address> = Vec::new();
        let mut victim: Vec<Address> = Vec::new();
        for (addr, result) in self.flash_loan_information.borrow_mut().iter(){
            match result.identity {
                PotentialIdentity::Beneficiary(_) => {
                    //println!("Address {:?} gains {:?} in 0.0001 USD unit", *addr, val);
                    beneficiary.push(*addr);
                },
                PotentialIdentity::Victim(_) => {
                    //println!("Address {:?} loses {:?} in 0.0001 USD unit", *addr, val);
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
            for (a, b, c, d) in start.iter() {
                //DEBUGGING: print all loan receiver
                println!("tx hash: {:?} Flash Loan Address {:?} sends {:?} of token address {:?} to Address {:?}", self.old_tx.hash(), *a, *c, *d, *b);
                let mut data_ptr = self.old_tx_contract_address.borrow_mut();
                *data_ptr = Some(*b);
            }
            for (addr, result) in self.flash_loan_information.borrow_mut().iter(){
                match result.identity {
                    PotentialIdentity::Beneficiary(val) => {
                        println!("Address {:?} gains {:?} in 0.0001 USD unit", *addr, val);
                    },
                    PotentialIdentity::Victim(val) => {
                        println!("Address {:?} loses {:?} in 0.0001 USD unit", *addr, val);
                    }
                    _ => (),
                }
            }
            
        }else{
            return false;
        }
        //Also check either sender address or contract address is beneficiary
        //assert_eq!(*self.old_tx_contract_address.borrow(), None);
        if !(beneficiary.contains(&self.old_tx.sender()) ||  beneficiary.contains(&self.old_tx_contract_address.borrow().unwrap())) {
            println!("sender {:?} and contract address {:?} are both not beneficiary. Front run tx will not be assembled!", self.old_tx.sender(), self.old_tx_contract_address.borrow().unwrap()); 
            return false;
        }
        //flash loan testing
        //return false;
        if assemable_new {
            self.assemable_new_transactions();
        }
        //Check the cost at the beginning of the transaction
        true
    }
    pub fn assemable_new_transactions (&self) {
        match self.old_tx.tx().action {
            Action::Create => {
                *self.new_deploy_tx.borrow_mut() = None;
                *self.new_tx.borrow_mut() = Some(
                                    TypedTransaction::Legacy(RawTransaction {
                                    action: Action::Create,
                                    nonce: self.my_nonce,
                                    gas_price: self.old_tx.tx().gas_price,
                                    gas: self.old_tx.tx().gas,
                                    value: self.old_tx.tx().value,
                                    data: Self::replace_hardcoded_address_in_data(self.old_tx.sender(), *FRONTRUN_ADDRESS, self.old_tx.tx().data.to_vec()),
                                    })
                                    .sign(&FRONTRUN_SECRET_KEY, (*self.old_tx).chain_id())
                                );
            },
            Action::Call(contract_addr) => {
                if let Some((deploy_gas_price, deploy_gas, deploy_value, is_create_action, call_address, deploy_data)) = Self::get_contract_init_data(&(*self.old_tx_contract_address.borrow()).unwrap()){
                *self.new_deploy_tx.borrow_mut() = Some(
                                        TypedTransaction::Legacy(RawTransaction {
                                        action: match is_create_action { 1u8 => Action::Create, _=> Action::Call(call_address),},
                                        nonce: self.my_nonce,
                                        gas_price: deploy_gas_price,
                                        gas: deploy_gas,
                                        value: deploy_value,
                                        data: deploy_data.clone(),
                                        })
                                        .sign(&FRONTRUN_SECRET_KEY, (*self.old_tx).chain_id())
                );
                let new_address= Self::contract_address_calculation(
                    &FRONTRUN_ADDRESS,
                    self.my_nonce,
                    &deploy_data.to_vec(),
                );
                println!("New contract address {:?} is assemabled into front run tx", new_address);
                //Replace contract address and sender address field in data if having.
                let call_data = Self::replace_hardcoded_address_in_data((*self.old_tx_contract_address.borrow()).unwrap(), new_address, self.old_tx.tx().data.to_vec());
                let call_data = Self::replace_hardcoded_address_in_data(self.old_tx.sender(), *FRONTRUN_ADDRESS, call_data);
                *self.new_tx.borrow_mut() = Some(
                                    TypedTransaction::Legacy(RawTransaction {
                                    action: Action::Call(
                                        match contract_addr == (*self.old_tx_contract_address.borrow()).unwrap()
                                            { 
                                                true => new_address, 
                                                _=> contract_addr,
                                            }
                                        ),
                                    nonce: self.my_nonce.saturating_add(U256::one()),
                                    gas_price: self.old_tx.tx().gas_price,
                                    gas: self.old_tx.tx().gas,
                                    value: self.old_tx.tx().value,
                                    data: call_data,
                                    })
                                    .sign(&FRONTRUN_SECRET_KEY, (*self.old_tx).chain_id())
                                );
                }else{
                    println!("No found information for contract address {:?}. Front run tx assembling failed!", (*self.old_tx_contract_address.borrow()).unwrap()); 
                }
            },
        }  
    }
    /// The function is used for the case where some deployed contract is created by opcode 'create' rather than Action::Create
    /// The contract address should be created afte executing the first tx
    pub fn overwrite_new_tx(new_tx_as_input: SignedTransaction, overwrite_contract_address: Address) -> SignedTransaction{
        match new_tx_as_input.tx().action {
            Action::Call(wrong_address) => {
                //Replace contract address field in data if having
                let call_data = Self::replace_hardcoded_address_in_data(wrong_address, overwrite_contract_address, new_tx_as_input.tx().data.to_vec());
                TypedTransaction::Legacy(RawTransaction {
                action: Action::Call(overwrite_contract_address),
                nonce: new_tx_as_input.tx().nonce,
                gas_price: new_tx_as_input.tx().gas_price,
                gas: new_tx_as_input.tx().gas,
                value: new_tx_as_input.tx().value,
                data: call_data,
                })
                .sign(&FRONTRUN_SECRET_KEY, new_tx_as_input.chain_id())
            },
            _ => {panic!("The new tx cannot be create action!");},
        }
    }
    pub fn get_txs(&self) -> Option<(Option<SignedTransaction>, Option<SignedTransaction>)> {
        Some((self.new_deploy_tx.borrow().clone(), self.new_tx.borrow().clone()))
    }
    pub fn set_old_tx_contract_address (&self, addr: Address) -> bool{
        let mut data_ptr = self.old_tx_contract_address.borrow_mut();
        if let Some(_) = *data_ptr {
            false
        }else{
            *data_ptr = Some(addr);
            true
        }
    }
    pub fn store_contract_address (&self, addr: Address) {
        let mut data_ptr = self.temp_contract_addresses.borrow_mut();
        (*data_ptr).push(addr.clone());
    }
    pub fn contract_address_calculation(sender: &Address, nonce: U256, code: &[u8]) -> Address {
        let (new_address, _) = contract_address(
            CreateContractAddress::FromSenderAndNonce,
            sender,
            &nonce,
            code,
        );
        new_address
    }
    pub fn set_contract_init_data(contract: &Address, gas_price: U256, gas: U256, value: U256, data: Vec<u8>, is_create_action: u8, call_address: Address, sender: Address) -> bool{
        if let Ok(db) = sled::open("contract_db") {
            let parsed_data = Self::replace_hardcoded_address_in_data(sender, *FRONTRUN_ADDRESS, data);
            let mut gas_price_bytes: [u8;32] = [0u8;32];
            let mut gas_bytes: [u8;32] = [0u8;32];
            let mut value_bytes: [u8;32] = [0u8;32];
            gas_price.to_little_endian(&mut gas_price_bytes);
            gas.to_little_endian(&mut gas_bytes);
            value.to_little_endian(&mut value_bytes);
            let call_address_bytes: [u8;20] = *call_address.as_fixed_bytes();
            let mut raw_data:Vec<u8> = Vec::new();
            raw_data.extend_from_slice(&gas_price_bytes);
            raw_data.extend_from_slice(&gas_bytes);
            raw_data.extend_from_slice(&value_bytes);
            raw_data.extend_from_slice(&[is_create_action]);
            raw_data.extend_from_slice(&call_address_bytes);
            raw_data.extend(parsed_data);
            if let Ok(_) = db.insert(
                contract.as_bytes(),
                raw_data,
            ){  
                db.flush().expect("Flush all dirty into db");
                true
            }else{
                false
            }
        }else{
            false
        }
    }
    pub fn get_contract_init_data(contract: &Address) -> Option<(U256, U256, U256, u8, Address, Vec<u8>)> { //return gas price, gas, value and data
        if let Ok(db) = sled::open("contract_db") {
            if let Ok(Some(data)) = db.get(contract.as_bytes()) {
                Some((
                    U256::from_little_endian(&data[0..32]),
                    U256::from_little_endian(&data[32..64]),
                    U256::from_little_endian(&data[64..96]),
                    data[96],
                    Address::from_slice(&data[97..117]),
                    data[117..].to_vec(),
                ))
            }else{
                None
            }
        }else{
            None
        }
    }
    //helper function: replace given address with front run addresses in 'data' field
    pub fn replace_hardcoded_address_in_data (address_in: Address, address_out: Address, data: Vec<u8>) -> Vec<u8> {
        let input = address_in.as_bytes().to_vec();
        let output = address_out.as_bytes().to_vec();
        let mut parsed_data = data.clone();
        if data.len() >= 20 {
            for i in 0..data.len()-20 {
                if data[i..i+20] == input[..] {
                    for j in 0..20 {
                        parsed_data[i+j] = output[j];
                    }
                }
            }
        }
        parsed_data
    }
}