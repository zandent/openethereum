use ethereum_types::{Address, Bloom, H256, U256};
use std::str::FromStr;
use rustc_hex::FromHex;
#[doc(hidden)]
lazy_static! {
    // //Set ETH address is 0x0
    // pub static ref ETH_CONTRACT_ADDRESS: Address = Address::from_str("0000000000000000000000000000000000000000").unwrap();
    // //Dai stablecoin address
    // pub static ref DAI_CONTRACT_ADDRESS: Address = Address::from_str("6B175474E89094C44Da98b954EedeAC495271d0F").unwrap();
    pub static ref FLASH_LOAN_CONTRACT_ADDRESSES: Vec<Address> = vec![
                                                                      //dydxFlashLoanTemplate.sol for testing TODO: remove after done
                                                                      Address::from_str("0000000000000000000000000000000000000000").unwrap(),
                                                                     ];
    //Token Address Vec
    pub static ref CONTRACT_ADDRESSES: Vec<Address> = vec![//erc.sol for testing TODO: remove after done
                                                           Address::from_str("b4c79daB8f259C7Aee6E5b2Aa729821864227e84").unwrap(),
                                                           //ETH
                                                           Address::from_str("0000000000000000000000000000000000000000").unwrap(),
                                                           //DAI
                                                           Address::from_str("6B175474E89094C44Da98b954EedeAC495271d0F").unwrap(), 
                                                          ];
    //keccak-256("balanceOf(address)") first four bytes
    pub static ref BALANCEOF_METHOD_ID: Vec<u8> = "70a08231".from_hex().unwrap();
    //keccak-256("transfer(address,uint256)") first four bytes
    pub static ref TRANSFER_METHOD_ID: Vec<u8> = "a9059cbb".from_hex().unwrap();
    //keccak-256("transferFrom(address,address,uint256)") first four bytes
    pub static ref TRANSFERFROM_METHOD_ID: Vec<u8> = "23b872dd".from_hex().unwrap();
    //keccak-256("Transfer(address,address,uint256)")
    pub static ref TRANSFER_EVENT_HASH: H256 = H256::from_str("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").unwrap();
}