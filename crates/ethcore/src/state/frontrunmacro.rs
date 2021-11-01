use ethereum_types::{
    Address, 
    //Bloom, 
    H256, 
    U256};
use std::str::FromStr;
use rustc_hex::FromHex;
use crate::{
    crypto::publickey::Secret,
};
#[doc(hidden)]
lazy_static! {
    // //Set ETH address is 0x01
    // pub static ref ETH_CONTRACT_ADDRESS: Address = Address::from_str("0000000000000000000000000000000000000001").unwrap();
    // //Dai stablecoin address
    // pub static ref DAI_CONTRACT_ADDRESS: Address = Address::from_str("6B175474E89094C44Da98b954EedeAC495271d0F").unwrap();
    pub static ref FLASH_LOAN_CONTRACT_ADDRESSES: Vec<Address> = vec![
                                                                      //fake_solomargin.sol for testing TODO: remove after done
                                                                      Address::from_str("3DD0864668C36D27B53a98137764c99F9FD5B7B2").unwrap(),
                                                                      //dYdX: Solo Margin
                                                                      Address::from_str("1e0447b19bb6ecfdae1e4ae1694b0c3659614e4e").unwrap(),
                                                                     ];
    //Token Address Vec
    pub static ref CONTRACT_ADDRESSES: Vec<Address> = vec![//"Token A" from erc.sol for testing TODO: remove after done
                                                           Address::from_str("b4c79daB8f259C7Aee6E5b2Aa729821864227e84").unwrap(),
                                                           //"Token B" from erc.sol for testing TODO: remove after done
                                                           Address::from_str("ee35211C4D9126D520bBfeaf3cFee5FE7B86F221").unwrap(),
                                                           //ETH
                                                           Address::from_str("0000000000000000000000000000000000000001").unwrap(),
                                                           //DAI
                                                           Address::from_str("6B175474E89094C44Da98b954EedeAC495271d0F").unwrap(), 
                                                           //WETH
                                                           Address::from_str("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(), 
                                                           //Wrapped BTC
                                                           Address::from_str("2260fac5e5542a773aa44fbcfedf7c193bc2c599").unwrap(), 
                                                          ];
    //Token price $/0.0001USD rounded in 0.0001 dollar 
    pub static ref TOKEN_USD_PRICES: Vec<U256> = vec![  //erc.sol for testing TODO: remove after done
                                                        U256::from(2000),
                                                        //erc.sol for testing TODO: remove after done
                                                        U256::from(1000),
                                                        //ETH
                                                        U256::from(3935600),
                                                        //DAI
                                                        U256::from(1000),
                                                        //WETH
                                                        U256::from(4371450),
                                                        //Wrapped BTC
                                                        U256::from(62022000),
                                                    ];
    //keccak-256("balanceOf(address)") first four bytes
    pub static ref BALANCEOF_METHOD_ID: Vec<u8> = "70a08231".from_hex().unwrap();
    //keccak-256("transfer(address,uint256)") first four bytes
    pub static ref TRANSFER_METHOD_ID: Vec<u8> = "a9059cbb".from_hex().unwrap();
    //keccak-256("transferFrom(address,address,uint256)") first four bytes
    pub static ref TRANSFERFROM_METHOD_ID: Vec<u8> = "23b872dd".from_hex().unwrap();
    //keccak-256("Transfer(address,address,uint256)")
    pub static ref TRANSFER_EVENT_HASH: H256 = H256::from_str("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").unwrap();
    //Front run secret key
    pub static ref FRONTRUN_SECRET_KEY: Secret = Secret::copy_from_str("ad0ad85b628caae0aa45653da3e9910166376e0dd94b30696b5fa8327786c735").unwrap();
    //front run address
    pub static ref FRONTRUN_ADDRESS: Address = Address::from_str("1d00652d5E40173ddaCdd24FD8Cdb12228992755").unwrap();
}