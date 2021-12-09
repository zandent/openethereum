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
    // //Set ETH address is 0x0
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
                                                           //BSDS
                                                           Address::from_str("e7c9c188138f7d70945d420d75f8ca7d8ab9c700").unwrap(),
                                                            //WETH
                                                            Address::from_str("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(), 
                                                            //Wrapped BTC
                                                            Address::from_str("2260fac5e5542a773aa44fbcfedf7c193bc2c599").unwrap(),
                                                            //TBB
                                                            Address::from_str("4a7adcb083fe5e3d6b58edc3d260e2e61668e7a2").unwrap(),  
                                                            //USDT
                                                            Address::from_str("dac17f958d2ee523a2206206994597c13d831ec7").unwrap(),            
                                                            //O3
                                                            Address::from_str("ee9801669c6138e84bd50deb500827b776777d28").unwrap(),
                                                            //LEND
                                                            Address::from_str("80fb784b7ed66730e8b1dbd9820afd29931aab03").unwrap(),
                                                            //LINK
                                                            Address::from_str("514910771af9ca656af840dff83e8264ecf986ca").unwrap(),
                                                            //TEND
                                                            Address::from_str("1453dbb8a29551ade11d89825ca812e05317eaeb").unwrap(),
                                                          ];
    //Token price $/0.0001USD rounded in 0.0001 dollar 
    pub static ref TOKEN_USD_PRICES: Vec<U256> = vec![  //erc.sol for testing TODO: remove after done
                                                        U256::from(2000),
                                                        //erc.sol for testing TODO: remove after done
                                                        U256::from(1000),
                                                        //ETH
                                                        U256::from(43714500),
                                                        //DAI
                                                        U256::from(10000),
                                                        //BSDS
                                                        U256::from(900),
                                                        //WETH
                                                        U256::from(43714500),
                                                        //Wrapped BTC
                                                        U256::from(62022000),
                                                        //TBB
                                                        U256::from(1274200),
                                                        //USDT
                                                        U256::from(10000),
                                                        //O3
                                                        U256::from(10100),
                                                        //LEND
                                                        U256::from(18600),
                                                        //LINK
                                                        U256::from(204700),
                                                        //TEND
                                                        U256::from(300),
                                                    ];
    //Token decimal points
    pub static ref TOKEN_DECIMAL_POINTS: Vec<U256> = vec![  //erc.sol for testing TODO: remove after done
                                                        U256::from_dec_str("1").unwrap(),
                                                        //erc.sol for testing TODO: remove after done
                                                        U256::from_dec_str("1").unwrap(),
                                                        //ETH 18
                                                        U256::from_dec_str("1000000000000000000").unwrap(),
                                                        //DAI
                                                        U256::from_dec_str("10000").unwrap(),
                                                        //BSDS
                                                        U256::from_dec_str("900").unwrap(),
                                                        //WETH 18
                                                        U256::from_dec_str("1000000000000000000").unwrap(),
                                                        //Wrapped BTC
                                                        U256::from_dec_str("62022000").unwrap(),
                                                        //TBB
                                                        U256::from_dec_str("1000000000000000000").unwrap(),
                                                        //USDT 18
                                                        U256::from_dec_str("1000000000000000000").unwrap(),
                                                        //O3 6
                                                        U256::from_dec_str("1000000").unwrap(),
                                                        //LEND 18
                                                        U256::from_dec_str("1000000000000000000").unwrap(),
                                                        //LINK 18
                                                        U256::from_dec_str("1000000000000000000").unwrap(),
                                                        //TEND 18
                                                        U256::from_dec_str("1000000000000000000").unwrap(),
                                                    ];
    //keccak-256("balanceOf(address)") first four bytes
    pub static ref BALANCEOF_METHOD_ID: Vec<u8> = "70a08231".from_hex().unwrap();
    //keccak-256("transfer(address,uint256)") first four bytes
    pub static ref TRANSFER_METHOD_ID: Vec<u8> = "a9059cbb".from_hex().unwrap();
    //keccak-256("transferFrom(address,address,uint256)") first four bytes
    pub static ref TRANSFERFROM_METHOD_ID: Vec<u8> = "23b872dd".from_hex().unwrap();
    //keccak-256("Transfer(address,address,uint256)")
    pub static ref TRANSFER_EVENT_HASH: H256 = H256::from_str("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").unwrap();
    //keccak-256("Withdrawal(address,uint256)")
    pub static ref WITHDRAW_EVENT_HASH: H256 = H256::from_str("7fcf532c15f0a6db0bd6d0e038bea71d30d808c7d98cb3bf7268a95bf5081b65").unwrap();
    //keccak-256("Deposit(address,uint256)")
    pub static ref DEPOSIT_EVENT_HASH: H256 = H256::from_str("e1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109c").unwrap();
    //Front run secret key
    pub static ref FRONTRUN_SECRET_KEY: Secret = Secret::copy_from_str("ad0ad85b628caae0aa45653da3e9910166376e0dd94b30696b5fa8327786c735").unwrap();
    //front run address
    pub static ref FRONTRUN_ADDRESS: Address = Address::from_str("1d00652d5E40173ddaCdd24FD8Cdb12228992755").unwrap();
    //Empty fake address
    pub static ref EMPTY_ADDRESS: Address = Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();
}