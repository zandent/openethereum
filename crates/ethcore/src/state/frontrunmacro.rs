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
use std::collections::HashMap;
#[doc(hidden)]
lazy_static! {
    pub static ref ERC_TOKEN_INFORMATION_MAP: HashMap<Address, (U256, U256)> = 
        [
            //"Token A" from erc.sol for testing TODO: remove after done
            (Address::from_str("b4c79daB8f259C7Aee6E5b2Aa729821864227e84").unwrap(), (U256::from(2000), U256::from_dec_str("1").unwrap())),
            //"Token B" from erc.sol for testing TODO: remove after done
            (Address::from_str("ee35211C4D9126D520bBfeaf3cFee5FE7B86F221").unwrap(), (U256::from(1000),U256::from_dec_str("1").unwrap())),
            //ETH 18
            (Address::from_str("0000000000000000000000000000000000000001").unwrap(), (U256::from(40205900),U256::from_dec_str("1000000000000000000").unwrap())),
            //WETH 18
            (Address::from_str("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(), (U256::from(40205900),U256::from_dec_str("1000000000000000000").unwrap())),
            //3crv 18
            (Address::from_str("6c3f90f043a72fa612cbac8115ee7e52bde6e490").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
            //USDP 18
            (Address::from_str("8e870d67f660d95d5be530380d0ec0bd388289e1").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
            //Fei USD 18
            (Address::from_str("956f47f50a910163d8bf957cf5846d573e7f87ca").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
            //Aave interest bearing WETH (aWETH) 18
            (Address::from_str("030ba81f1c18d280636f32af80b9aad02cf0854e").unwrap(), (U256::from(41104000),U256::from_dec_str("1000000000000000000").unwrap())),
            //Aave variable debt bearing USDC 6
            (Address::from_str("619beb58998ed2278e08620f97007e1116d5d25b").unwrap(), (U256::from(0),U256::from_dec_str("1000000").unwrap())),
            //Aave interest bearing USDC aUSDC 6
            (Address::from_str("bcca60bb61934080951369a648fb03df4f96263c").unwrap(), (U256::from(10000),U256::from_dec_str("1000000").unwrap())),
//BNB (BNB) 18
(Address::from_str("B8c77482e45F1F44dE1745F52C74426C631bDD52").unwrap(), (U256::from(5292306),U256::from_dec_str("1000000000000000000").unwrap())),
//Tether USD (USDT) 6
(Address::from_str("dac17f958d2ee523a2206206994597c13d831ec7").unwrap(), (U256::from(10000),U256::from_dec_str("1000000").unwrap())),
//HEX (HEX) 8
(Address::from_str("2b591e99afe9f32eaa6214f7b7629768c40eeb39").unwrap(), (U256::from(2965),U256::from_dec_str("100000000").unwrap())),
//USD Coin (USDC) 6
(Address::from_str("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(), (U256::from(10000),U256::from_dec_str("1000000").unwrap())),
//SHIBA INU (SHIB) 18
(Address::from_str("95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//Matic Token (MATIC) 18
(Address::from_str("7d1afa7b718fb893db30a3abc0cfc608aacfebb0").unwrap(), (U256::from(24342),U256::from_dec_str("1000000000000000000").unwrap())),
//Binance USD (BUSD) 18
(Address::from_str("4fabb145d64652a948d72533023f6e7a623c7c53").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//Crypto.com Coin (CRO) 8
(Address::from_str("a0b73e1ff0b80914ab6fe0444e65848c4c34450b").unwrap(), (U256::from(5280),U256::from_dec_str("100000000").unwrap())),
//Wrapped BTC (WBTC) 8
(Address::from_str("2260fac5e5542a773aa44fbcfedf7c193bc2c599").unwrap(), (U256::from(488430000),U256::from_dec_str("100000000").unwrap())),
//Wrapped UST Token (UST) 18
(Address::from_str("a47c8bf37f92abed4a126bda807a7b7498661acd").unwrap(), (U256::from(10100),U256::from_dec_str("1000000000000000000").unwrap())),
//ChainLink Token (LINK) 18
(Address::from_str("514910771af9ca656af840dff83e8264ecf986ca").unwrap(), (U256::from(194100),U256::from_dec_str("1000000000000000000").unwrap())),
//Dai Stablecoin (DAI) 18
(Address::from_str("6b175474e89094c44da98b954eedeac495271d0f").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//OKB (OKB) 18
(Address::from_str("75231f58b43240c9718dd58b4967c5114342a86c").unwrap(), (U256::from(321700),U256::from_dec_str("1000000000000000000").unwrap())),
//TRON (TRX) 6
(Address::from_str("e1be5d3f34e89de342ee97e6e90d405884da6c67").unwrap(), (U256::from(790),U256::from_dec_str("1000000").unwrap())),
//Uniswap (UNI) 18
(Address::from_str("1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap(), (U256::from(150500),U256::from_dec_str("1000000000000000000").unwrap())),
//stETH (stETH) 18
(Address::from_str("ae7ab96520de3a18e5e111b5eaab095312d7fe84").unwrap(), (U256::from(39974900),U256::from_dec_str("1000000000000000000").unwrap())),
//Wrapped liquid staked Ether 2.0 (wstETH) 18
(Address::from_str("7f39c581f595b53c5cb19bd0b3f8da6c935e2ca0").unwrap(), (U256::from(39974900),U256::from_dec_str("1000000000000000000").unwrap())),
//Compound Ether (cETH) 8
(Address::from_str("4ddc2d193948926d02f9b1fe9e1daa0718270ed5").unwrap(), (U256::from(805900),U256::from_dec_str("100000000").unwrap())),
//VeChain (VEN) 18
(Address::from_str("d850942ef8811f2a866692a623011bde52a462c1").unwrap(), (U256::from(830),U256::from_dec_str("1000000000000000000").unwrap())),
//Wrapped Filecoin (WFIL) 18
(Address::from_str("6e1A19F235bE7ED8E3369eF73b196C07257494DE").unwrap(), (U256::from(352673),U256::from_dec_str("1000000000000000000").unwrap())),
//SAND (SAND) 18
(Address::from_str("3845badAde8e6dFF049820680d1F14bD3903a5d0").unwrap(), (U256::from(51300),U256::from_dec_str("1000000000000000000").unwrap())),
//Magic Internet Money (MIM) 18
(Address::from_str("99d8a9c45b2eca8864373a26d1459e3dff1e17f3").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//Wrapped Decentraland MANA (wMANA) 18
(Address::from_str("fd09cf7cfffa9932e33668311c4777cb9db3c9be").unwrap(), (U256::from(32599),U256::from_dec_str("1000000000000000000").unwrap())),
//Compound Dai (cDAI) 8
(Address::from_str("5d3a536E4D6DbD6114cc1Ead35777bAB948E3643").unwrap(), (U256::from(218),U256::from_dec_str("100000000").unwrap())),
//Theta Token (THETA) 18
(Address::from_str("3883f5e181fccaf8410fa61e12b59bad963fb645").unwrap(), (U256::from(41908),U256::from_dec_str("1000000000000000000").unwrap())),
//Fantom Token (FTM) 18
(Address::from_str("4e15361fd6b4bb609fa63c81a2be19d873717870").unwrap(), (U256::from(14900),U256::from_dec_str("1000000000000000000").unwrap())),
//Graph Token (GRT) 18
(Address::from_str("c944e90c64b2c07662a292be6244bdf05cda44a7").unwrap(), (U256::from(7102),U256::from_dec_str("1000000000000000000").unwrap())),
//Compound USD Coin (cUSDC) 8
(Address::from_str("39aa39c021dfbae8fac545936693ac917d5e7563").unwrap(), (U256::from(225),U256::from_dec_str("100000000").unwrap())),
//Bitfinex LEO Token (LEO) 18
(Address::from_str("2af5d2ad76741191d15dfe7bf6ac92d4bd912ca3").unwrap(), (U256::from(36100),U256::from_dec_str("1000000000000000000").unwrap())),
//Gala (GALA) 8
(Address::from_str("15D4c048F83bd7e37d49eA4C83a07267Ec4203dA").unwrap(), (U256::from(4417),U256::from_dec_str("100000000").unwrap())),
//LoopringCoin V2 (LRC) 18
(Address::from_str("bbbbca6a901c926f240b89eacb641d8aec7aeafd").unwrap(), (U256::from(23600),U256::from_dec_str("1000000000000000000").unwrap())),
//HarmonyOne (ONE) 18
(Address::from_str("799a4202c12ca952cb311598a024c80ed371a41e").unwrap(), (U256::from(2450),U256::from_dec_str("1000000000000000000").unwrap())),
//BitTorrent (BTT) 6
(Address::from_str("e83cccfabd4ed148903bf36d4283ee7c8b3494d1").unwrap(), (U256::from(27),U256::from_dec_str("1000000").unwrap())),
//Quant (QNT) 18
(Address::from_str("4a220e6096b25eadb88358cb44068a3248254675").unwrap(), (U256::from(1836699),U256::from_dec_str("1000000000000000000").unwrap())),
//Amp (AMP) 18
(Address::from_str("ff20817765cb7f73d4bde2e66e067e58d11095c2").unwrap(), (U256::from(484),U256::from_dec_str("1000000000000000000").unwrap())),
//EnjinCoin (ENJ) 18
(Address::from_str("f629cbd94d3791c9250152bd8dfbdf380e2a3b9c").unwrap(), (U256::from(24600),U256::from_dec_str("1000000000000000000").unwrap())),
//Maker (MKR) 18
(Address::from_str("9f8f72aa9304c8b593d555f12ef6589cc3a579a2").unwrap(), (U256::from(24336000),U256::from_dec_str("1000000000000000000").unwrap())),
//Huobi BTC (HBTC) 18
(Address::from_str("0316EB71485b0Ab14103307bf65a021042c6d380").unwrap(), (U256::from(491120000),U256::from_dec_str("1000000000000000000").unwrap())),
//Aave interest bearing CRV (aCRV) 18
(Address::from_str("8dae6cb04688c62d939ed9b68d32bc62e49970b1").unwrap(), (U256::from(48000),U256::from_dec_str("1000000000000000000").unwrap())),
//Spell Token (SPELL) 18
(Address::from_str("090185f2135308bad17527004364ebcc2d37e5f6").unwrap(), (U256::from(222),U256::from_dec_str("1000000000000000000").unwrap())),
//BAT (BAT) 18
(Address::from_str("0d8775f648430679a709e98d2b0cb6250d2887ef").unwrap(), (U256::from(11600),U256::from_dec_str("1000000000000000000").unwrap())),
//KuCoin Token (KCS) 6
(Address::from_str("f34960d9d60be18cc1d5afc1a6f012a723a28811").unwrap(), (U256::from(216916),U256::from_dec_str("1000000").unwrap())),
//Celsius (CEL) 4
(Address::from_str("aaaebe6fe48e54f431b0c390cfaf0b017d09d42d").unwrap(), (U256::from(38500),U256::from_dec_str("10000").unwrap())),
//HuobiToken (HT) 18
(Address::from_str("6f259637dcd74c767781e37bc6133cd6a68aa161").unwrap(), (U256::from(100200),U256::from_dec_str("1000000000000000000").unwrap())),
//Wrapped Celo (wCELO) 18
(Address::from_str("e452e6ea2ddeb012e20db73bf5d3863a3ac8d77a").unwrap(), (U256::from(42676),U256::from_dec_str("1000000000000000000").unwrap())),
//Frax (FRAX) 18
(Address::from_str("853d955acef822db058eb8505911ed77f175b99e").unwrap(), (U256::from(10100),U256::from_dec_str("1000000000000000000").unwrap())),
//chiliZ (CHZ) 18
(Address::from_str("3506424f91fd33084466f402d5d97f05f8e3b4af").unwrap(), (U256::from(2828),U256::from_dec_str("1000000000000000000").unwrap())),
//HoloToken (HOT) 18
(Address::from_str("6c6ee5e31d828de241282b9606c8e98ea48526e2").unwrap(), (U256::from(83),U256::from_dec_str("1000000000000000000").unwrap())),
//TrueUSD (TUSD) 18
(Address::from_str("0000000000085d4780B73119b644AE5ecd22b376").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//Nexo (NEXO) 18
(Address::from_str("b62132e35a6c13ee1ee0f84dc5d40bad8d815206").unwrap(), (U256::from(22200),U256::from_dec_str("1000000000000000000").unwrap())),
//yearn.finance (YFI) 18
(Address::from_str("0bc529c00C6401aEF6D220BE8C6Ea1667F6Ad93e").unwrap(), (U256::from(345110000),U256::from_dec_str("1000000000000000000").unwrap())),
//IoTeX Network (IOTX) 18
(Address::from_str("6fb3e0a217407efff7ca062d46c26e5d60a14d69").unwrap(), (U256::from(1287),U256::from_dec_str("1000000000000000000").unwrap())),
//Compound (COMP) 18
(Address::from_str("c00e94cb662c3520282e6f5717214004a7f26888").unwrap(), (U256::from(1946800),U256::from_dec_str("1000000000000000000").unwrap())),
//SushiToken (SUSHI) 18
(Address::from_str("6b3595068778dd592e39a122f4f5a5cf09c90fe2").unwrap(), (U256::from(58500),U256::from_dec_str("1000000000000000000").unwrap())),
//XinFin XDCE (XDCE) 18
(Address::from_str("41ab1b6fcbb2fa9dced81acbdec13ea6315f2bf2").unwrap(), (U256::from(858),U256::from_dec_str("1000000000000000000").unwrap())),
//Synthetix Network Token (SNX) 18
(Address::from_str("c011a73ee8576fb46f5e1c5751ca3b9fe0af2a6f").unwrap(), (U256::from(53700),U256::from_dec_str("1000000000000000000").unwrap())),
//1INCH Token (1INCH) 18
(Address::from_str("111111111117dc0aa78b770fa6a738034120c302").unwrap(), (U256::from(24600),U256::from_dec_str("1000000000000000000").unwrap())),
//Pax Dollar (USDP) 18
(Address::from_str("8e870d67f660d95d5be530380d0ec0bd388289e1").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//NXM (NXM) 18
(Address::from_str("d7c49cee7e9188cca6ad8ff264c1da2e69d4cf3b").unwrap(), (U256::from(1336500),U256::from_dec_str("1000000000000000000").unwrap())),
//Livepeer Token (LPT) 18
(Address::from_str("58b6a8a3302369daec383334672404ee733ab239").unwrap(), (U256::from(373900),U256::from_dec_str("1000000000000000000").unwrap())),
//WQtum (WQTUM) 18
(Address::from_str("3103df8f05c4d8af16fd22ae63e406b97fec6938").unwrap(), (U256::from(92241),U256::from_dec_str("1000000000000000000").unwrap())),
//Compound USDT (cUSDT) 8
(Address::from_str("f650c3d88d12db855b8bf7d11be6c55a4e07dcc9").unwrap(), (U256::from(217),U256::from_dec_str("100000000").unwrap())),
//WAX Token (WAX) 8
(Address::from_str("39bb259f66e1c59d5abef88375979b4d20d98022").unwrap(), (U256::from(4611),U256::from_dec_str("100000000").unwrap())),
//OMG Network (OMG) 18
(Address::from_str("d26114cd6EE289AccF82350c8d8487fedB8A0C07").unwrap(), (U256::from(61000),U256::from_dec_str("1000000000000000000").unwrap())),
//Gnosis (GNO) 18
(Address::from_str("6810e776880c02933d47db1b9fc05908e5386b96").unwrap(), (U256::from(4537300),U256::from_dec_str("1000000000000000000").unwrap())),
//renBTC (renBTC) 8
(Address::from_str("eb4c2781e4eba804ce9a9803c67d0893436bb27d").unwrap(), (U256::from(498430000),U256::from_dec_str("100000000").unwrap())),
//Ethereum Name Service (ENS) 18
(Address::from_str("c18360217d8f7ab5e7c516566761ea12ce7f9d72").unwrap(), (U256::from(409365),U256::from_dec_str("1000000000000000000").unwrap())),
//pTokens SAFEMOON (pSAFEMOON) 18
(Address::from_str("16631e53c20fd2670027c6d53efe2642929b285c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//Zilliqa (ZIL) 12
(Address::from_str("05f4a42e251f2d52b8ed15e9fedaacfcef1fad27").unwrap(), (U256::from(641),U256::from_dec_str("1000000000000").unwrap())),
//Telcoin (TEL) 2
(Address::from_str("467Bccd9d29f223BcE8043b84E8C8B282827790F").unwrap(), (U256::from(129),U256::from_dec_str("100").unwrap())),
//Bancor (BNT) 18
(Address::from_str("1f573d6fb3f13d689ff844b4ce37794d79a7ff1c").unwrap(), (U256::from(33300),U256::from_dec_str("1000000000000000000").unwrap())),
//Rocket Pool (RPL) 18
(Address::from_str("d33526068d116ce69f19a9ee46f0bd304f21a51f").unwrap(), (U256::from(458100),U256::from_dec_str("1000000000000000000").unwrap())),
//Rocket Pool (RPL) 18
(Address::from_str("b4efd85c19999d84251304bda99e90b92300bd93").unwrap(), (U256::from(458100),U256::from_dec_str("1000000000000000000").unwrap())),
//Illuvium (ILV) 18
(Address::from_str("767fe9edc9e0df98e07454847909b5e959d7ca0e").unwrap(), (U256::from(11055900),U256::from_dec_str("1000000000000000000").unwrap())),
//Wootrade Network (WOO) 18
(Address::from_str("4691937a7508860f876c9c0a2a617e7d9e945d4b").unwrap(), (U256::from(7920),U256::from_dec_str("1000000000000000000").unwrap())),
//ZRX (ZRX) 18
(Address::from_str("e41d2489571d322189246dafa5ebde1f4699f498").unwrap(), (U256::from(7877),U256::from_dec_str("1000000000000000000").unwrap())),
//Dogelon (ELON) 18
(Address::from_str("761d38e5ddf6ccf6cf7c55759d5210750b5d60f3").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//Frax Share (FXS) 18
(Address::from_str("3432b6a60d23ca0dfca7761b7ab56459d9c964d0").unwrap(), (U256::from(175500),U256::from_dec_str("1000000000000000000").unwrap())),
//UMA Voting Token v1 (UMA) 18
(Address::from_str("04Fa0d235C4abf4BcF4787aF4CF447DE572eF828").unwrap(), (U256::from(89900),U256::from_dec_str("1000000000000000000").unwrap())),
//SwissBorg (CHSB) 8
(Address::from_str("ba9d4199fab4f26efe3551d490e3821486f135ba").unwrap(), (U256::from(5961),U256::from_dec_str("100000000").unwrap())),
//IOSToken (IOST) 18
(Address::from_str("fa1a856cfa3409cfa145fa4e20eb270df3eb21ab").unwrap(), (U256::from(302),U256::from_dec_str("1000000000000000000").unwrap())),
//Boba Token (BOBA) 18
(Address::from_str("42bbfa2e77757c645eeaad1655e0911a7553efbc").unwrap(), (U256::from(35100),U256::from_dec_str("1000000000000000000").unwrap())),
//Fei USD (FEI) 18
(Address::from_str("956F47F50A910163D8BF957Cf5846D573E7f87CA").unwrap(), (U256::from(9978),U256::from_dec_str("1000000000000000000").unwrap())),
//dYdX (DYDX) 18
(Address::from_str("92d6c1e31e14520e676a687f0a93788b716beff5").unwrap(), (U256::from(75700),U256::from_dec_str("1000000000000000000").unwrap())),
//XY Oracle (XYO) 18
(Address::from_str("55296f69f40ea6d20e478533c15a6b08b654e758").unwrap(), (U256::from(385),U256::from_dec_str("1000000000000000000").unwrap())),
//Serum (SRM) 6
(Address::from_str("476c5E26a75bd202a9683ffD34359C0CC15be0fF").unwrap(), (U256::from(35500),U256::from_dec_str("1000000").unwrap())),
//Golem Network Token (GLM) 18
(Address::from_str("7DD9c5Cba05E151C895FDe1CF355C9A1D5DA6429").unwrap(), (U256::from(4539),U256::from_dec_str("1000000000000000000").unwrap())),
//Polymath (POLY) 18
(Address::from_str("9992ec3cf6a55b00978cddf2b27bc6882d88d1ec").unwrap(), (U256::from(5050),U256::from_dec_str("1000000000000000000").unwrap())),
//Mask Network (MASK) 18
(Address::from_str("69af81e73a73b40adf4f3d4223cd9b1ece623074").unwrap(), (U256::from(108400),U256::from_dec_str("1000000000000000000").unwrap())),
//Tribe (TRIBE) 18
(Address::from_str("c7283b66eb1eb5fb86327f08e1b5816b0720212b").unwrap(), (U256::from(9210),U256::from_dec_str("1000000000000000000").unwrap())),
//CelerToken (CELR) 18
(Address::from_str("4f9254c83eb525f9fcf346490bbb3ed28a81c667").unwrap(), (U256::from(731),U256::from_dec_str("1000000000000000000").unwrap())),
//Anyswap (ANY) 18
(Address::from_str("f99d58e463a2e07e5692127302c20a191861b4d6").unwrap(), (U256::from(220703),U256::from_dec_str("1000000000000000000").unwrap())),
//Trace (TRAC) 18
(Address::from_str("aa7a9ca87d3694b5755f213b5d04094b8d0f0a6f").unwrap(), (U256::from(10900),U256::from_dec_str("1000000000000000000").unwrap())),
//Function X (FX) 18
(Address::from_str("8c15ef5b4b21951d50e53e4fbda8298ffad25057").unwrap(), (U256::from(9404),U256::from_dec_str("1000000000000000000").unwrap())),
//Fetch (FET) 18
(Address::from_str("aea46A60368A7bD060eec7DF8CBa43b7EF41Ad85").unwrap(), (U256::from(5551),U256::from_dec_str("1000000000000000000").unwrap())),
//Chroma (CHR) 6
(Address::from_str("8A2279d4A90B6fe1C4B30fa660cC9f926797bAA2").unwrap(), (U256::from(6522),U256::from_dec_str("1000000").unwrap())),
//Synapse (SYN) 18
(Address::from_str("0f2d719407fdbeff09d87557abb7232601fd9f29").unwrap(), (U256::from(21400),U256::from_dec_str("1000000000000000000").unwrap())),
//KEEP Token (KEEP) 18
(Address::from_str("85eee30c52b0b379b046fb0f85f4f3dc3009afec").unwrap(), (U256::from(6423),U256::from_dec_str("1000000000000000000").unwrap())),
//Injective Token (INJ) 18
(Address::from_str("e28b3B32B6c345A34Ff64674606124Dd5Aceca30").unwrap(), (U256::from(80300),U256::from_dec_str("1000000000000000000").unwrap())),
//Ocean Token (OCEAN) 18
(Address::from_str("967da4048cD07aB37855c090aAF366e4ce1b9F48").unwrap(), (U256::from(7835),U256::from_dec_str("1000000000000000000").unwrap())),
//Paxos Gold (PAXG) 18
(Address::from_str("45804880De22913dAFE09f4980848ECE6EcbAf78").unwrap(), (U256::from(17943700),U256::from_dec_str("1000000000000000000").unwrap())),
//DENT (DENT) 8
(Address::from_str("3597bfd533a99c9aa083587b074434e61eb0a258").unwrap(), (U256::from(35),U256::from_dec_str("100000000").unwrap())),
//Gemini dollar (GUSD) 2
(Address::from_str("056fd409e1d7a124bd7017459dfea2f387b6d5cd").unwrap(), (U256::from(10000),U256::from_dec_str("100").unwrap())),
//AlphaToken (ALPHA) 18
(Address::from_str("a1faa113cbe53436df28ff0aee54275c13b40975").unwrap(), (U256::from(7125),U256::from_dec_str("1000000000000000000").unwrap())),
//HUSD (HUSD) 8
(Address::from_str("df574c24545e5ffecb9a659c229253d4111d87e1").unwrap(), (U256::from(10000),U256::from_dec_str("100000000").unwrap())),
//Energy Web Token Bridged (EWTB) 18
(Address::from_str("178c820f862b14f316509ec36b13123da19a6054").unwrap(), (U256::from(102780),U256::from_dec_str("1000000000000000000").unwrap())),
//CoinEx Token (CET) 18
(Address::from_str("081f67afa0ccf8c7b17540767bbe95df2ba8d97f").unwrap(), (U256::from(799),U256::from_dec_str("1000000000000000000").unwrap())),
//MEDX TOKEN (MEDX) 8
(Address::from_str("fd1e80508f243e64ce234ea88a5fd2827c71d4b7").unwrap(), (U256::from(3691),U256::from_dec_str("100000000").unwrap())),
//Tether Gold (XAUt) 6
(Address::from_str("68749665ff8d2d112fa859aa293f07a622782f38").unwrap(), (U256::from(17930500),U256::from_dec_str("1000000").unwrap())),
//Swipe (SXP) 18
(Address::from_str("8ce9137d39326ad0cd6491fb5cc0cba0e089b6a9").unwrap(), (U256::from(15400),U256::from_dec_str("1000000000000000000").unwrap())),
//Aragon Network Token (ANT) 18
(Address::from_str("a117000000f279d81a1d3cc75430faa017fa5a2e").unwrap(), (U256::from(75600),U256::from_dec_str("1000000000000000000").unwrap())),
//Pundi X Token (PUNDIX) 18
(Address::from_str("0fd10b9899882a6f2fcb5c371e17e70fdee00c38").unwrap(), (U256::from(11000),U256::from_dec_str("1000000000000000000").unwrap())),
//Rari Governance Token (RGT) 18
(Address::from_str("D291E7a03283640FDc51b121aC401383A46cC623").unwrap(), (U256::from(233384),U256::from_dec_str("1000000000000000000").unwrap())),
//Request (REQ) 18
(Address::from_str("8f8221afbb33998d8584a2b05749ba73c37a938a").unwrap(), (U256::from(3663),U256::from_dec_str("1000000000000000000").unwrap())),
//StatusNetwork (SNT) 18
(Address::from_str("744d70fdbe2ba4cf95131626614a1763df805b9e").unwrap(), (U256::from(724),U256::from_dec_str("1000000000000000000").unwrap())),
//Keep3rV1 (KP3R) 18
(Address::from_str("1ceb5cb57c4d4e2b2433641b95dd330a33185a44").unwrap(), (U256::from(9186700),U256::from_dec_str("1000000000000000000").unwrap())),
//MCO (MCO) 8
(Address::from_str("b63b606ac810a52cca15e44bb630fd42d8d1d83d").unwrap(), (U256::from(167997),U256::from_dec_str("100000000").unwrap())),
//Storj (STORJ) 8
(Address::from_str("b64ef51c888972c908cfacf59b47c1afbc0ab8ac").unwrap(), (U256::from(18300),U256::from_dec_str("100000000").unwrap())),
//Orbs (ORBS) 18
(Address::from_str("ff56cc6b1e6ded347aa0b7676c85ab0b3d08b0fa").unwrap(), (U256::from(878),U256::from_dec_str("1000000000000000000").unwrap())),
//OriginToken (OGN) 18
(Address::from_str("8207c1ffc5b6804f6024322ccf34f29c3541ae26").unwrap(), (U256::from(6228),U256::from_dec_str("1000000000000000000").unwrap())),
//NKN (NKN) 18
(Address::from_str("5cf04716ba20127f1e2297addcf4b5035000c9eb").unwrap(), (U256::from(3515),U256::from_dec_str("1000000000000000000").unwrap())),
//Dusk Network (DUSK) 18
(Address::from_str("940a2db1b7008b6c776d4faaca729d6d4a4aa551").unwrap(), (U256::from(6222),U256::from_dec_str("1000000000000000000").unwrap())),
//UniBright (UBT) 8
(Address::from_str("8400d94a5cb0fa0d041a3788e395285d61c9ee5e").unwrap(), (U256::from(15600),U256::from_dec_str("100000000").unwrap())),
//DODO bird (DODO) 18
(Address::from_str("43Dfc4159D86F3A37A5A4B3D4580b888ad7d4DDd").unwrap(), (U256::from(8635),U256::from_dec_str("1000000000000000000").unwrap())),
//Divi Exchange Token (DIVX) 18
(Address::from_str("13f11c9905a08ca76e3e853be63d4f0944326c72").unwrap(), (U256::from(852),U256::from_dec_str("1000000000000000000").unwrap())),
//BioPassport Coin (BIOT) 9
(Address::from_str("c07A150ECAdF2cc352f5586396e344A6b17625EB").unwrap(), (U256::from(1272),U256::from_dec_str("1000000000").unwrap())),
//Bifrost (BFC) 18
(Address::from_str("0c7D5ae016f806603CB1782bEa29AC69471CAb9c").unwrap(), (U256::from(2037),U256::from_dec_str("1000000000000000000").unwrap())),
//BandToken (BAND) 18
(Address::from_str("ba11d00c5f74255f56a5e366f4f77f5a186d7f55").unwrap(), (U256::from(53200),U256::from_dec_str("1000000000000000000").unwrap())),
//ALICE (ALICE) 6
(Address::from_str("ac51066d7bec65dc4589368da368b212745d63e8").unwrap(), (U256::from(125700),U256::from_dec_str("1000000").unwrap())),
//Token Prometeus Network (PROM) 18
(Address::from_str("fc82bb4ba86045af6f327323a46e80412b91b27d").unwrap(), (U256::from(131300),U256::from_dec_str("1000000000000000000").unwrap())),
//Orchid (OXT) 18
(Address::from_str("4575f41308EC1483f3d399aa9a2826d74Da13Deb").unwrap(), (U256::from(3653),U256::from_dec_str("1000000000000000000").unwrap())),
//BitMax token (BTMX) 18
(Address::from_str("cca0c9c383076649604eE31b20248BC04FdF61cA").unwrap(), (U256::from(3207),U256::from_dec_str("1000000000000000000").unwrap())),
//RLC (RLC) 9
(Address::from_str("607F4C5BB672230e8672085532f7e901544a7375").unwrap(), (U256::from(29800),U256::from_dec_str("1000000000").unwrap())),
//StormX (STMX) 18
(Address::from_str("be9375c6a420d2eeb258962efb95551a5b722803").unwrap(), (U256::from(222),U256::from_dec_str("1000000000000000000").unwrap())),
//Balancer (BAL) 18
(Address::from_str("ba100000625a3754423978a60c9317c58a424e3d").unwrap(), (U256::from(187000),U256::from_dec_str("1000000000000000000").unwrap())),
//XSGD (XSGD) 6
(Address::from_str("70e8de73ce538da2beed35d14187f6959a8eca96").unwrap(), (U256::from(7352),U256::from_dec_str("1000000").unwrap())),
//Numeraire (NMR) 18
(Address::from_str("1776e1f26f98b1a5df9cd347953a26dd3cb46671").unwrap(), (U256::from(330100),U256::from_dec_str("1000000000000000000").unwrap())),
//PowerLedger (POWR) 6
(Address::from_str("595832f8fc6bf59c85c527fec3740a1b7a361269").unwrap(), (U256::from(4522),U256::from_dec_str("1000000").unwrap())),
//Lido DAO Token (LDO) 18
(Address::from_str("5a98fcbea516cf06857215779fd812ca3bef1b32").unwrap(), (U256::from(29600),U256::from_dec_str("1000000000000000000").unwrap())),
//Ankr Eth2 Reward Bearing Certificate (aETHc) 18
(Address::from_str("E95A203B1a91a908F9B9CE46459d101078c2c3cb").unwrap(), (U256::from(35680900),U256::from_dec_str("1000000000000000000").unwrap())),
//SingularityNET Token (AGIX) 8
(Address::from_str("5b7533812759b45c2b44c19e320ba2cd2681b542").unwrap(), (U256::from(1933),U256::from_dec_str("100000000").unwrap())),
//Veritaseum (VERI) 18
(Address::from_str("8f3470A7388c05eE4e7AF3d01D8C722b0FF52374").unwrap(), (U256::from(834200),U256::from_dec_str("1000000000000000000").unwrap())),
//TrueFi (TRU) 8
(Address::from_str("4c19596f5aaff459fa38b0f7ed92f11ae6543784").unwrap(), (U256::from(3230),U256::from_dec_str("100000000").unwrap())),
//ELF (ELF) 18
(Address::from_str("bf2179859fc6d5bee9bf9158632dc51678a4100e").unwrap(), (U256::from(3778),U256::from_dec_str("1000000000000000000").unwrap())),
//Vader (VADER) 18
(Address::from_str("2602278ee1882889b946eb11dc0e810075650983").unwrap(), (U256::from(470),U256::from_dec_str("1000000000000000000").unwrap())),
//Beta Token (BETA) 18
(Address::from_str("be1a001fe942f96eea22ba08783140b9dcc09d28").unwrap(), (U256::from(6425),U256::from_dec_str("1000000000000000000").unwrap())),
//Dawn (DAWN) 18
(Address::from_str("580c8520deda0a441522aeae0f9f7a5f29629afa").unwrap(), (U256::from(23000),U256::from_dec_str("1000000000000000000").unwrap())),
//Aurora DAO (AURA) 18
(Address::from_str("cdcfc0f66c522fd086a1b725ea3c0eeb9f9e8814").unwrap(), (U256::from(2604),U256::from_dec_str("1000000000000000000").unwrap())),
//IceToken (ICE) 18
(Address::from_str("f16e81dce15b08f326220742020379b855b87df9").unwrap(), (U256::from(153100),U256::from_dec_str("1000000000000000000").unwrap())),
//Proton (XPR) 4
(Address::from_str("D7EFB00d12C2c13131FD319336Fdf952525dA2af").unwrap(), (U256::from(182),U256::from_dec_str("10000").unwrap())),
//Uquid Coin (UQC) 18
(Address::from_str("8806926Ab68EB5a7b909DcAf6FdBe5d93271D6e2").unwrap(), (U256::from(147200),U256::from_dec_str("1000000000000000000").unwrap())),
//Crypto20 (C20) 18
(Address::from_str("26e75307fc0c021472feb8f727839531f112f317").unwrap(), (U256::from(44100),U256::from_dec_str("1000000000000000000").unwrap())),
//STPT (STPT) 18
(Address::from_str("de7d85157d9714eadf595045cc12ca4a5f3e2adb").unwrap(), (U256::from(1104),U256::from_dec_str("1000000000000000000").unwrap())),
//Iron Bank EUR (ibEUR) 18
(Address::from_str("96e61422b6a9ba0e068b6c5add4ffabc6a4aae27").unwrap(), (U256::from(11900),U256::from_dec_str("1000000000000000000").unwrap())),
//Metal (MTL) 8
(Address::from_str("F433089366899D83a9f26A773D59ec7eCF30355e").unwrap(), (U256::from(21800),U256::from_dec_str("100000000").unwrap())),
//Kin (KIN) 18
(Address::from_str("818fc6c2ec5986bc6e2cbf00939d90556ab12ce5").unwrap(), (U256::from(1),U256::from_dec_str("1000000000000000000").unwrap())),
//Gitcoin (GTC) 18
(Address::from_str("de30da39c46104798bb5aa3fe8b9e0e1f348163f").unwrap(), (U256::from(95900),U256::from_dec_str("1000000000000000000").unwrap())),
//QuarkChain Token (QKC) 18
(Address::from_str("ea26c4ac16d4a5a106820bc8aee85fd0b7b2b664").unwrap(), (U256::from(204),U256::from_dec_str("1000000000000000000").unwrap())),
//Compound Basic Attention Token (cBAT) 8
(Address::from_str("6c8c6b02e7b2be14d4fa6022dfd6d75921d90e4e").unwrap(), (U256::from(238),U256::from_dec_str("100000000").unwrap())),
//Kyber Network Crystal v2 (KNC) 18
(Address::from_str("deFA4e8a7bcBA345F687a2f1456F5Edd9CE97202").unwrap(), (U256::from(12800),U256::from_dec_str("1000000000000000000").unwrap())),
//FEGtoken (FEG) 9
(Address::from_str("389999216860ab8e0175387a0c90e5c52522c945").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//LCX (LCX) 18
(Address::from_str("037a54aab062628c9bbae1fdb1583c195585fe41").unwrap(), (U256::from(2150),U256::from_dec_str("1000000000000000000").unwrap())),
//Melon Token (MLN) 18
(Address::from_str("ec67005c4e498ec7f55e092bd1d35cbc47c91892").unwrap(), (U256::from(825500),U256::from_dec_str("1000000000000000000").unwrap())),
//KyberNetwork (KNC) 18
(Address::from_str("dd974d5c2e2928dea5f71b9825b8b646686bd200").unwrap(), (U256::from(12900),U256::from_dec_str("1000000000000000000").unwrap())),
//Synth sUSD (sUSD) 18
(Address::from_str("57ab1ec28d129707052df4df418d58a2d46d5f51").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//Reputation (REPv2) 18
(Address::from_str("221657776846890989a759ba2973e427dff5c9bb").unwrap(), (U256::from(170300),U256::from_dec_str("1000000000000000000").unwrap())),
//POA ERC20 on Foundation (POA20) 18
(Address::from_str("6758b7d441a9739b98552b373703d8d3d14f9e62").unwrap(), (U256::from(3955),U256::from_dec_str("1000000000000000000").unwrap())),
//Wrapped NXM (wNXM) 18
(Address::from_str("0d438f3b5175bebc262bf23753c1e53d03432bde").unwrap(), (U256::from(684727),U256::from_dec_str("1000000000000000000").unwrap())),
//MXCToken (MXC) 18
(Address::from_str("5ca381bbfb58f0092df149bd3d243b08b9a8386e").unwrap(), (U256::from(472),U256::from_dec_str("1000000000000000000").unwrap())),
//Adventure Gold (AGLD) 18
(Address::from_str("32353A6C91143bfd6C7d363B546e62a9A2489A20").unwrap(), (U256::from(16145),U256::from_dec_str("1000000000000000000").unwrap())),
//STASIS EURS Token (EURS) 2
(Address::from_str("db25f211ab05b1c97d595516f45794528a807ad8").unwrap(), (U256::from(11399),U256::from_dec_str("100").unwrap())),
//Presearch (PRE) 18
(Address::from_str("EC213F83defB583af3A000B1c0ada660b1902A0F").unwrap(), (U256::from(2813),U256::from_dec_str("1000000000000000000").unwrap())),
//Decentral Games (DG) 18
(Address::from_str("4b520c812e8430659fc9f12f6d0c39026c83588d").unwrap(), (U256::from(3677),U256::from_dec_str("1000000000000000000").unwrap())),
//FunFair (FUN) 8
(Address::from_str("419d0d8bdd9af5e606ae2232ed285aff190e711b").unwrap(), (U256::from(102),U256::from_dec_str("100000000").unwrap())),
//Automata (ATA) 18
(Address::from_str("a2120b9e674d3fc3875f415a7df52e382f141225").unwrap(), (U256::from(6244),U256::from_dec_str("1000000000000000000").unwrap())),
//AIOZ Network (AIOZ) 18
(Address::from_str("626e8036deb333b408be468f951bdb42433cbf18").unwrap(), (U256::from(5053),U256::from_dec_str("1000000000000000000").unwrap())),
//CocosToken (COCOS) 18
(Address::from_str("0c6f5f7d555e7518f6841a79436bd2b1eef03381").unwrap(), (U256::from(24400),U256::from_dec_str("1000000000000000000").unwrap())),
//SpookyToken (BOO) 18
(Address::from_str("55af5865807b196bd0197e0902746f31fbccfa58").unwrap(), (U256::from(150500),U256::from_dec_str("1000000000000000000").unwrap())),
//EthLend (LEND) 18
(Address::from_str("80fB784B7eD66730e8b1DBd9820aFD29931aab03").unwrap(), (U256::from(19500),U256::from_dec_str("1000000000000000000").unwrap())),
//Smooth Love Potion (SLP) 0
(Address::from_str("cc8fa225d80b9c7d42f96e9570156c65d6caaa25").unwrap(), (U256::from(310),U256::from_dec_str("1").unwrap())),
//Compound 0x (cZRX) 8
(Address::from_str("b3319f5d18bc0d84dd1b4825dcde5d5f7266d407").unwrap(), (U256::from(161),U256::from_dec_str("100000000").unwrap())),
//Wrapped Celo USD (wCUSD) 18
(Address::from_str("ad3e3fc59dff318beceaab7d00eb4f68b1ecf195").unwrap(), (U256::from(9952),U256::from_dec_str("1000000000000000000").unwrap())),
//DeversiFi Token (DVF) 18
(Address::from_str("dddddd4301a082e62e84e43f474f044423921918").unwrap(), (U256::from(79500),U256::from_dec_str("1000000000000000000").unwrap())),
//Decentral Games Governance (xDG) 18
(Address::from_str("4f81c790581b240a5c948afd173620ecc8c71c8d").unwrap(), (U256::from(3778),U256::from_dec_str("1000000000000000000").unwrap())),
//CarryToken (CRE) 18
(Address::from_str("115ec79f1de567ec68b7ae7eda501b406626478e").unwrap(), (U256::from(106),U256::from_dec_str("1000000000000000000").unwrap())),
//QANX Token (QANX) 18
(Address::from_str("aaa7a10a8ee237ea61e8ac46c50a8db8bcc1baaa").unwrap(), (U256::from(959),U256::from_dec_str("1000000000000000000").unwrap())),
//TORN Token (TORN) 18
(Address::from_str("77777feddddffc19ff86db637967013e6c6a116c").unwrap(), (U256::from(397700),U256::from_dec_str("1000000000000000000").unwrap())),
//mStable USD (mUSD) 18
(Address::from_str("e2f2a5c287993345a840db3b0845fbc70f5935a5").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//Litentry (LIT) 18
(Address::from_str("b59490ab09a0f526cc7305822ac65f2ab12f9723").unwrap(), (U256::from(29800),U256::from_dec_str("1000000000000000000").unwrap())),
//Nuls (NULS) 8
(Address::from_str("a2791bdf2d5055cda4d46ec17f9f429568275047").unwrap(), (U256::from(9082),U256::from_dec_str("100000000").unwrap())),
//Eden (EDEN) 18
(Address::from_str("1559fa1b8f28238fd5d76d9f434ad86fd20d1559").unwrap(), (U256::from(9465),U256::from_dec_str("1000000000000000000").unwrap())),
//Quickswap (QUICK) 18
(Address::from_str("6c28AeF8977c9B773996d0e8376d2EE379446F2f").unwrap(), (U256::from(2371100),U256::from_dec_str("1000000000000000000").unwrap())),
//Mainframe Token (MFT) 18
(Address::from_str("df2c7238198ad8b389666574f2d8bc411a4b7428").unwrap(), (U256::from(88),U256::from_dec_str("1000000000000000000").unwrap())),
//Ribbon (RBN) 18
(Address::from_str("6123b0049f904d730db3c36a31167d9d4121fa6b").unwrap(), (U256::from(16700),U256::from_dec_str("1000000000000000000").unwrap())),
//Shiden (SDN) 18
(Address::from_str("00e856ee945a49bb73436e719d96910cd9d116a4").unwrap(), (U256::from(14900),U256::from_dec_str("1000000000000000000").unwrap())),
//Ampleforth Governance (FORTH) 18
(Address::from_str("77fba179c79de5b7653f68b5039af940ada60ce0").unwrap(), (U256::from(93000),U256::from_dec_str("1000000000000000000").unwrap())),
//BarnBridge Governance Token (BOND) 18
(Address::from_str("0391D2021f89DC339F60Fff84546EA23E337750f").unwrap(), (U256::from(154500),U256::from_dec_str("1000000000000000000").unwrap())),
//bZx Protocol Token (BZRX) 18
(Address::from_str("56d811088235F11C8920698a204A5010a788f4b3").unwrap(), (U256::from(2153),U256::from_dec_str("1000000000000000000").unwrap())),
//Cortex Coin (CTXC) 18
(Address::from_str("ea11755ae41d889ceec39a63e6ff75a02bc1c00d").unwrap(), (U256::from(4186),U256::from_dec_str("1000000000000000000").unwrap())),
//ParaSwap (PSP) 18
(Address::from_str("cafe001067cdef266afb7eb5a286dcfd277f3de5").unwrap(), (U256::from(3781),U256::from_dec_str("1000000000000000000").unwrap())),
//Tellor Tributes (TRB) 18
(Address::from_str("88df592f8eb5d7bd38bfef7deb0fbc02cf3778a0").unwrap(), (U256::from(328400),U256::from_dec_str("1000000000000000000").unwrap())),
//Bluzelle (BLZ) 18
(Address::from_str("5732046a883704404f284ce41ffadd5b007fd668").unwrap(), (U256::from(2357),U256::from_dec_str("1000000000000000000").unwrap())),
//hoge.finance (HOGE) 9
(Address::from_str("fad45e47083e4607302aa43c65fb3106f1cd7607").unwrap(), (U256::from(2),U256::from_dec_str("1000000000").unwrap())),
//Propy (PRO) 8
(Address::from_str("226bb599a12c826476e3a771454697ea52e9e220").unwrap(), (U256::from(13100),U256::from_dec_str("100000000").unwrap())),
//DIAToken (DIA) 18
(Address::from_str("84cA8bc7997272c7CfB4D0Cd3D55cd942B3c9419").unwrap(), (U256::from(12600),U256::from_dec_str("1000000000000000000").unwrap())),
//FOX (FOX) 18
(Address::from_str("c770eefad204b5180df6a14ee197d99d808ee52d").unwrap(), (U256::from(6189),U256::from_dec_str("1000000000000000000").unwrap())),
//PlatonCoin (PLTC) 18
(Address::from_str("429D83Bb0DCB8cdd5311e34680ADC8B12070a07f").unwrap(), (U256::from(7890),U256::from_dec_str("1000000000000000000").unwrap())),
//Aergo (AERGO) 18
(Address::from_str("91Af0fBB28ABA7E31403Cb457106Ce79397FD4E6").unwrap(), (U256::from(2763),U256::from_dec_str("1000000000000000000").unwrap())),
//Sai Stablecoin v1.0 (SAI) 18
(Address::from_str("89d24a6b4ccb1b6faa2625fe562bdd9a23260359").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//OVR (OVR) 18
(Address::from_str("21bfbda47a0b4b5b1248c767ee49f7caa9b23697").unwrap(), (U256::from(24800),U256::from_dec_str("1000000000000000000").unwrap())),
//GRID (GRID) 12
(Address::from_str("12b19d3e2ccc14da04fae33e63652ce469b3f2fd").unwrap(), (U256::from(18300),U256::from_dec_str("1000000000000").unwrap())),
//Rarible (RARI) 18
(Address::from_str("fca59cd816ab1ead66534d82bc21e7515ce441cf").unwrap(), (U256::from(130900),U256::from_dec_str("1000000000000000000").unwrap())),
//PAID Network (PAID) 18
(Address::from_str("1614f18fc94f47967a3fbe5ffcd46d4e7da3d787").unwrap(), (U256::from(5886),U256::from_dec_str("1000000000000000000").unwrap())),
//Bread (BRD) 18
(Address::from_str("558ec3152e2eb2174905cd19aea4e34a23de9ad6").unwrap(), (U256::from(8114),U256::from_dec_str("1000000000000000000").unwrap())),
//Covalent Query Token (CQT) 18
(Address::from_str("d417144312dbf50465b1c641d016962017ef6240").unwrap(), (U256::from(5470),U256::from_dec_str("1000000000000000000").unwrap())),
//BetProtocolToken (BEPRO) 18
(Address::from_str("cf3c8be2e2c42331da80ef210e9b1b307c03d36a").unwrap(), (U256::from(100),U256::from_dec_str("1000000000000000000").unwrap())),
//Moss Coin (MOC) 18
(Address::from_str("865ec58b06bf6305b886793aa20a2da31d034e68").unwrap(), (U256::from(2436),U256::from_dec_str("1000000000000000000").unwrap())),
//Bytom (BTM) 8
(Address::from_str("cb97e65f07da24d46bcdd078ebebd7c6e6e3d750").unwrap(), (U256::from(383),U256::from_dec_str("100000000").unwrap())),
//EverRise (RISE) 18
(Address::from_str("0cD022ddE27169b20895e0e2B2B8A33B25e63579").unwrap(), (U256::from(10),U256::from_dec_str("1000000000000000000").unwrap())),
//RHOC (RHOC) 8
(Address::from_str("168296bb09e24a88805cb9c33356536b980d3fc5").unwrap(), (U256::from(1026),U256::from_dec_str("100000000").unwrap())),
//BitMartToken (BMC) 18
(Address::from_str("986EE2B944c42D017F52Af21c4c69B84DBeA35d8").unwrap(), (U256::from(3605),U256::from_dec_str("1000000000000000000").unwrap())),
//Refereum (RFR) 4
(Address::from_str("d0929d411954c47438dc1d871dd6081f5c5e149c").unwrap(), (U256::from(130),U256::from_dec_str("10000").unwrap())),
//MANTRA DAO (OM) 18
(Address::from_str("3593d125a4f7849a1b059e64f4517a86dd60c95d").unwrap(), (U256::from(1509),U256::from_dec_str("1000000000000000000").unwrap())),
//BOSAGORA (BOA) 7
(Address::from_str("746dda2ea243400d5a63e0700f190ab79f06489e").unwrap(), (U256::from(2037),U256::from_dec_str("10000000").unwrap())),
//Metronome (MET) 18
(Address::from_str("a3d58c4e56fedcae3a7c43a725aee9a71f0ece4e").unwrap(), (U256::from(50100),U256::from_dec_str("1000000000000000000").unwrap())),
//PolkaFoundry (PKF) 18
(Address::from_str("8b39b70e39aa811b69365398e0aace9bee238aeb").unwrap(), (U256::from(16200),U256::from_dec_str("1000000000000000000").unwrap())),
//DGD (DGD) 9
(Address::from_str("e0b7927c4af23765cb51314a0e0521a9645f0e2a").unwrap(), (U256::from(7595400),U256::from_dec_str("1000000000").unwrap())),
//Parsiq Token (PRQ) 18
(Address::from_str("362bc847A3a9637d3af6624EeC853618a43ed7D2").unwrap(), (U256::from(4306),U256::from_dec_str("1000000000000000000").unwrap())),
//Measurable Data Token (MDT) 18
(Address::from_str("814e0908b12a99fecf5bc101bb5d0b8b5cdf7d26").unwrap(), (U256::from(1001),U256::from_dec_str("1000000000000000000").unwrap())),
//Fusion (FSN) 18
(Address::from_str("d0352a019e9ab9d757776f532377aaebd36fd541").unwrap(), (U256::from(7556),U256::from_dec_str("1000000000000000000").unwrap())),
//OCC (OCC) 18
(Address::from_str("2f109021afe75b949429fe30523ee7c0d5b27207").unwrap(), (U256::from(31300),U256::from_dec_str("1000000000000000000").unwrap())),
//Marlin POND (POND) 18
(Address::from_str("57b946008913b82e4df85f501cbaed910e58d26c").unwrap(), (U256::from(658),U256::from_dec_str("1000000000000000000").unwrap())),
//MATH Token (MATH) 18
(Address::from_str("08d967bb0134f2d07f7cfb6e246680c53927dd30").unwrap(), (U256::from(3624),U256::from_dec_str("1000000000000000000").unwrap())),
//LockTrip (LOC) 18
(Address::from_str("5e3346444010135322268a4630d2ed5f8d09446c").unwrap(), (U256::from(31600),U256::from_dec_str("1000000000000000000").unwrap())),
//Kryll (KRL) 18
(Address::from_str("464ebe77c293e473b48cfe96ddcf88fcf7bfdac0").unwrap(), (U256::from(13700),U256::from_dec_str("1000000000000000000").unwrap())),
//Shyft [ Wrapped ] (SHFT) 18
(Address::from_str("b17c88bda07d28b3838e0c1de6a30eafbcf52d85").unwrap(), (U256::from(3685),U256::from_dec_str("1000000000000000000").unwrap())),
//Adshares (ADS) 11
(Address::from_str("cfcecfe2bd2fed07a9145222e8a7ad9cf1ccd22a").unwrap(), (U256::from(24100),U256::from_dec_str("100000000000").unwrap())),
//AirSwap (AST) 4
(Address::from_str("27054b13b1b798b345b591a4d22e6562d47ea75a").unwrap(), (U256::from(2938),U256::from_dec_str("10000").unwrap())),
//Dock (DOCK) 18
(Address::from_str("e5dada80aa6477e85d09747f2842f7993d0df71c").unwrap(), (U256::from(678),U256::from_dec_str("1000000000000000000").unwrap())),
//Hegic (HEGIC) 18
(Address::from_str("584bC13c7D411c00c01A62e8019472dE68768430").unwrap(), (U256::from(711),U256::from_dec_str("1000000000000000000").unwrap())),
//DEXTools (DEXT) 18
(Address::from_str("fb7b4564402e5500db5bb6d63ae671302777c75a").unwrap(), (U256::from(4624),U256::from_dec_str("1000000000000000000").unwrap())),
//STAKE (STAKE) 18
(Address::from_str("0Ae055097C6d159879521C384F1D2123D1f195e6").unwrap(), (U256::from(144700),U256::from_dec_str("1000000000000000000").unwrap())),
//pTokens BTC (pBTC) 18
(Address::from_str("5228a22e72ccc52d415ecfd199f99d0665e7733b").unwrap(), (U256::from(490380000),U256::from_dec_str("1000000000000000000").unwrap())),
//SENTINEL PROTOCOL (UPP) 18
(Address::from_str("c86d054809623432210c107af2e3f619dcfbf652").unwrap(), (U256::from(1487),U256::from_dec_str("1000000000000000000").unwrap())),
//CoinDash Token (CDT) 18
(Address::from_str("177d39ac676ed1c67a2b268ad7f1e58826e5b0af").unwrap(), (U256::from(712),U256::from_dec_str("1000000000000000000").unwrap())),
//Sentivate (SNTVT) 18
(Address::from_str("7865af71cf0b288b4e7f654f4f7851eb46a2b7f8").unwrap(), (U256::from(189),U256::from_dec_str("1000000000000000000").unwrap())),
//Frontier Token (FRONT) 18
(Address::from_str("f8C3527CC04340b208C854E985240c02F7B7793f").unwrap(), (U256::from(7044),U256::from_dec_str("1000000000000000000").unwrap())),
//QASH (QASH) 6
(Address::from_str("618e75ac90b12c6049ba3b27f5d5f8651b0037f6").unwrap(), (U256::from(661),U256::from_dec_str("1000000").unwrap())),
//BTU Protocol (BTU) 18
(Address::from_str("b683d83a532e2cb7dfa5275eed3698436371cc9f").unwrap(), (U256::from(5524),U256::from_dec_str("1000000000000000000").unwrap())),
//Pinakion (PNK) 18
(Address::from_str("93ed3fbe21207ec2e8f2d3c3de6e058cb73bc04d").unwrap(), (U256::from(817),U256::from_dec_str("1000000000000000000").unwrap())),  


//Gifto (GTO) 5
(Address::from_str("c5bbae50781be1669306b9e001eff57a2957b09d").unwrap(), (U256::from(597),U256::from_dec_str("100000").unwrap())),
//Nectar (NCT) 18
(Address::from_str("9e46a38f5daabe8683e10793b06749eef7d733d1").unwrap(), (U256::from(253),U256::from_dec_str("1000000000000000000").unwrap())),
//NimiqNetwork (NET) 18
(Address::from_str("cfb98637bcae43C13323EAa1731cED2B716962fD").unwrap(), (U256::from(45),U256::from_dec_str("1000000000000000000").unwrap())),
//ERC20 (ERC20) 18
(Address::from_str("c3761eb917cd790b30dad99f6cc5b4ff93c4f9ea").unwrap(), (U256::from(325),U256::from_dec_str("1000000000000000000").unwrap())),
//PolkaBridge (PBR) 18
(Address::from_str("298d492e8c1d909d3f63bc4a36c66c64acb3d695").unwrap(), (U256::from(9484),U256::from_dec_str("1000000000000000000").unwrap())),
//Civilization (CIV) 18
(Address::from_str("37fe0f067fa808ffbdd12891c0858532cfe7361d").unwrap(), (U256::from(1146),U256::from_dec_str("1000000000000000000").unwrap())),
//SelfKey (KEY) 18
(Address::from_str("4cc19356f2d37338b9802aa8e8fc58b0373296e7").unwrap(), (U256::from(121),U256::from_dec_str("1000000000000000000").unwrap())),
//veCRV-DAO yVault (yveCRV-DAO) 18
(Address::from_str("c5bddf9843308380375a611c18b50fb9341f502a").unwrap(), (U256::from(24600),U256::from_dec_str("1000000000000000000").unwrap())),
//Blockchain Monster Coin (BCMC) 18
(Address::from_str("2BA8349123de45E931a8C8264c332E6e9CF593F9").unwrap(), (U256::from(13700),U256::from_dec_str("1000000000000000000").unwrap())),
//Rubic (RBC) 18
(Address::from_str("a4eed63db85311e22df4473f87ccfc3dadcfa3e3").unwrap(), (U256::from(3064),U256::from_dec_str("1000000000000000000").unwrap())),
//NAGA Coin (NGC) 18
(Address::from_str("72dd4b6bd852a3aa172be4d6c5a6dbec588cf131").unwrap(), (U256::from(4211),U256::from_dec_str("1000000000000000000").unwrap())),
//UNIC (UNIC) 18
(Address::from_str("94e0bab2f6ab1f19f4750e42d7349f2740513ad5").unwrap(), (U256::from(1103100),U256::from_dec_str("1000000000000000000").unwrap())),
//Student Coin (STC) 18
(Address::from_str("15b543e986b8c34074dfc9901136d9355a537e7e").unwrap(), (U256::from(61),U256::from_dec_str("1000000000000000000").unwrap())),
//pNetwork Token (PNT) 18
(Address::from_str("89Ab32156e46F46D02ade3FEcbe5Fc4243B9AAeD").unwrap(), (U256::from(9521),U256::from_dec_str("1000000000000000000").unwrap())),
//Fuse Token (FUSE) 18
(Address::from_str("970b9bb2c0444f5e81e9d0efb84c8ccdcdcaf84d").unwrap(), (U256::from(2157),U256::from_dec_str("1000000000000000000").unwrap())),
//BLOCKv (VEE) 18
(Address::from_str("340d2bde5eb28c1eed91b2f790723e3b160613b7").unwrap(), (U256::from(97),U256::from_dec_str("1000000000000000000").unwrap())),
//Guaranteed Entrance Token (GET) 18
(Address::from_str("8a854288a5976036a725879164ca3e91d30c6a1b").unwrap(), (U256::from(20000),U256::from_dec_str("1000000000000000000").unwrap())),
//VesperToken (VSP) 18
(Address::from_str("1b40183efb4dd766f11bda7a7c3ad8982e998421").unwrap(), (U256::from(46100),U256::from_dec_str("1000000000000000000").unwrap())),
//Exeedme (XED) 18
(Address::from_str("ee573a945b01b788b9287ce062a0cfc15be9fd86").unwrap(), (U256::from(4017),U256::from_dec_str("1000000000000000000").unwrap())),
//StackOS (STACK) 18
(Address::from_str("56a86d648c435dc707c8405b78e2ae8eb4e60ba4").unwrap(), (U256::from(1122),U256::from_dec_str("1000000000000000000").unwrap())),
//Stratos Token (STOS) 18
(Address::from_str("08c32b0726c5684024ea6e141c50ade9690bbdcc").unwrap(), (U256::from(13700),U256::from_dec_str("1000000000000000000").unwrap())),
//Quantstamp (QSP) 18
(Address::from_str("99ea4db9ee77acd40b119bd1dc4e33e1c070b80d").unwrap(), (U256::from(419),U256::from_dec_str("1000000000000000000").unwrap())),
//ELYSIA (EL) 18
(Address::from_str("2781246fe707bb15cee3e5ea354e2154a2877b16").unwrap(), (U256::from(109),U256::from_dec_str("1000000000000000000").unwrap())),
//Launchpool token (LPOOL) 18
(Address::from_str("6149c26cd2f7b5ccdb32029af817123f6e37df5b").unwrap(), (U256::from(30700),U256::from_dec_str("1000000000000000000").unwrap())),
//Walton (WTC) 18
(Address::from_str("b7cb1c96db6b22b0d3d9536e0108d062bd488f74").unwrap(), (U256::from(9947),U256::from_dec_str("1000000000000000000").unwrap())),
//MCDEX Token (MCB) 18
(Address::from_str("4e352cF164E64ADCBad318C3a1e222E9EBa4Ce42").unwrap(), (U256::from(156521),U256::from_dec_str("1000000000000000000").unwrap())),
//Reserve (RSV) 18
(Address::from_str("196f4727526eA7FB1e17b2071B3d8eAA38486988").unwrap(), (U256::from(10016),U256::from_dec_str("1000000000000000000").unwrap())),
//UnFederalReserveToken (eRSDL) 18
(Address::from_str("5218E472cFCFE0b64A064F055B43b4cdC9EfD3A6").unwrap(), (U256::from(764),U256::from_dec_str("1000000000000000000").unwrap())),
//Dragon (DRGN) 18
(Address::from_str("419c4db4b9e25d6db2ad9691ccb832c8d9fda05e").unwrap(), (U256::from(775),U256::from_dec_str("1000000000000000000").unwrap())),
//Deri (DERI) 18
(Address::from_str("a487bf43cf3b10dffc97a9a744cbb7036965d3b9").unwrap(), (U256::from(2154),U256::from_dec_str("1000000000000000000").unwrap())),
//Cardstack (CARD) 18
(Address::from_str("954b890704693af242613edef1b603825afcd708").unwrap(), (U256::from(94),U256::from_dec_str("1000000000000000000").unwrap())),
//AVT (AVT) 18
(Address::from_str("0d88ed6e74bbfd96b831231638b66c05571e824f").unwrap(), (U256::from(33800),U256::from_dec_str("1000000000000000000").unwrap())),
//HOPR Token (HOPR) 18
(Address::from_str("f5581dfefd8fb0e4aec526be659cfab1f8c781da").unwrap(), (U256::from(2350),U256::from_dec_str("1000000000000000000").unwrap())),
//CargoX (CXO) 18
(Address::from_str("b6ee9668771a79be7967ee29a63d4184f8097143").unwrap(), (U256::from(1610),U256::from_dec_str("1000000000000000000").unwrap())),
//Switcheo Token (SWTH) 8
(Address::from_str("b4371da53140417cbb3362055374b10d97e420bb").unwrap(), (U256::from(154),U256::from_dec_str("100000000").unwrap())),
//SENTinel (SENT) 8
(Address::from_str("a44e5137293e855b1b7bc7e2c6f8cd796ffcb037").unwrap(), (U256::from(133),U256::from_dec_str("100000000").unwrap())),
//Spice (SFI) 18
(Address::from_str("b753428af26e81097e7fd17f40c88aaa3e04902c").unwrap(), (U256::from(3317100),U256::from_dec_str("1000000000000000000").unwrap())),
//DSLA (DSLA) 18
(Address::from_str("3affcca64c2a6f4e3b6bd9c64cd2c969efd1ecbe").unwrap(), (U256::from(47),U256::from_dec_str("1000000000000000000").unwrap())),
//Route (ROUTE) 18
(Address::from_str("16eccfdbb4ee1a85a33f3a9b21175cd7ae753db4").unwrap(), (U256::from(35600),U256::from_dec_str("1000000000000000000").unwrap())),
//Polkamon (PMON) 18
(Address::from_str("1796ae0b0fa4862485106a0de9b654efe301d0b2").unwrap(), (U256::from(82000),U256::from_dec_str("1000000000000000000").unwrap())),
//Populous (PPT) 8
(Address::from_str("d4fa1460f537bb9085d22c7bccb5dd450ef28e3a").unwrap(), (U256::from(7068),U256::from_dec_str("100000000").unwrap())),
//Blockport (BPT) 18
(Address::from_str("327682779bab2bf4d1337e8974ab9de8275a7ca8").unwrap(), (U256::from(3888),U256::from_dec_str("1000000000000000000").unwrap())),
//LikeCoin (LIKE) 18
(Address::from_str("02f61fd266da6e8b102d4121f5ce7b992640cf98").unwrap(), (U256::from(246),U256::from_dec_str("1000000000000000000").unwrap())),
//Klee Kai (KLEE) 9
(Address::from_str("382f0160c24f5c515a19f155bac14d479433a407").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//THORSwap Token (THOR) 18
(Address::from_str("a5f2211b9b8170f694421f2046281775e8468044").unwrap(), (U256::from(11299),U256::from_dec_str("1000000000000000000").unwrap())),
//0chain (ZCN) 10
(Address::from_str("b9EF770B6A5e12E45983C5D80545258aA38F3B78").unwrap(), (U256::from(5056),U256::from_dec_str("10000000000").unwrap())),
//Smart MFG (MFG) 18
(Address::from_str("6710c63432a2de02954fc0f851db07146a6c0312").unwrap(), (U256::from(787),U256::from_dec_str("1000000000000000000").unwrap())),
//Darwinia Network Native Token (RING) 18
(Address::from_str("9469d013805bffb7d3debe5e7839237e535ec483").unwrap(), (U256::from(472),U256::from_dec_str("1000000000000000000").unwrap())),
//Gelato Network Token (GEL) 18
(Address::from_str("15b7c0c907e4c6b9adaaaabc300c08991d6cea05").unwrap(), (U256::from(23000),U256::from_dec_str("1000000000000000000").unwrap())),
//O3 Swap Token (O3) 18
(Address::from_str("ee9801669c6138e84bd50deb500827b776777d28").unwrap(), (U256::from(6576),U256::from_dec_str("1000000000000000000").unwrap())),

//Ultiledger (ULT) 18
(Address::from_str("e884cc2795b9c45beeac0607da9539fd571ccf85").unwrap(), (U256::from(120),U256::from_dec_str("1000000000000000000").unwrap())),
//Yuan Chain New (YCC) 8
(Address::from_str("37e1160184f7dd29f00b78c050bf13224780b0b0").unwrap(), (U256::from(45),U256::from_dec_str("100000000").unwrap())),
//NUM Token (NUM) 18
(Address::from_str("3496b523e5c00a4b4150d6721320cddb234c3079").unwrap(), (U256::from(9269),U256::from_dec_str("1000000000000000000").unwrap())),
//Internxt (INXT) 8
(Address::from_str("4a8f5f96d5436e43112c2fbc6a9f70da9e4e16d4").unwrap(), (U256::from(200700),U256::from_dec_str("100000000").unwrap())),
//Cindicator (CND) 18
(Address::from_str("d4c435f5b09f855c3317c8524cb1f586e42795fa").unwrap(), (U256::from(115),U256::from_dec_str("1000000000000000000").unwrap())),
//BAX (BAX) 18
(Address::from_str("9a0242b7a33dacbe40edb927834f96eb39f8fbcb").unwrap(), (U256::from(7),U256::from_dec_str("1000000000000000000").unwrap())),
//SAN (SAN) 18
(Address::from_str("7c5a0ce9267ed19b22f8cae653f198e3e8daf098").unwrap(), (U256::from(3439),U256::from_dec_str("1000000000000000000").unwrap())),
//Pendle (PENDLE) 18
(Address::from_str("808507121b80c02388fad14726482e061b8da827").unwrap(), (U256::from(2970),U256::from_dec_str("1000000000000000000").unwrap())),
//ICONOMI (ICN) 18
(Address::from_str("888666CA69E0f178DED6D75b5726Cee99A87D698").unwrap(), (U256::from(2167),U256::from_dec_str("1000000000000000000").unwrap())),
//ZBToken (ZB) 18
(Address::from_str("bd0793332e9fb844a52a205a233ef27a5b34b927").unwrap(), (U256::from(2858),U256::from_dec_str("1000000000000000000").unwrap())),
//ZEON (ZEON) 18
(Address::from_str("e5b826ca2ca02f09c1725e9bd98d9a8874c30532").unwrap(), (U256::from(7),U256::from_dec_str("1000000000000000000").unwrap())),
//BZ (BZ) 18
(Address::from_str("4375e7ad8a01b8ec3ed041399f62d9cd120e0063").unwrap(), (U256::from(1613),U256::from_dec_str("1000000000000000000").unwrap())),
//WPPToken (WPP) 18
(Address::from_str("1955d744F9435522Be508D1Ba60E3c12D0690B6A").unwrap(), (U256::from(66),U256::from_dec_str("1000000000000000000").unwrap())),
//DaTa eXchange Token (DTX) 18
(Address::from_str("765f0c16d1ddc279295c1a7c24b0883f62d33f75").unwrap(), (U256::from(908),U256::from_dec_str("1000000000000000000").unwrap())),
//FOAM Token (FOAM) 18
(Address::from_str("4946fcea7c692606e8908002e55a582af44ac121").unwrap(), (U256::from(566),U256::from_dec_str("1000000000000000000").unwrap())),
//Meta (MTA) 18
(Address::from_str("a3BeD4E1c75D00fa6f4E5E6922DB7261B5E9AcD2").unwrap(), (U256::from(7034),U256::from_dec_str("1000000000000000000").unwrap())),
//Fractal Protocol Token (FCL) 18
(Address::from_str("f4d861575ecc9493420a3f5a14f85b13f0b50eb3").unwrap(), (U256::from(1813),U256::from_dec_str("1000000000000000000").unwrap())),
//TokenClub Token (TCT) 18
(Address::from_str("4824a7b64e3966b0133f4f4ffb1b9d6beb75fff7").unwrap(), (U256::from(343),U256::from_dec_str("1000000000000000000").unwrap())),
//Curate (XCUR) 8
(Address::from_str("E1c7E30C42C24582888C758984f6e382096786bd").unwrap(), (U256::from(23300),U256::from_dec_str("100000000").unwrap())),
//Gro DAO Token (GRO) 18
(Address::from_str("3ec8798b81485a254928b70cda1cf0a2bb0b74d7").unwrap(), (U256::from(49900),U256::from_dec_str("1000000000000000000").unwrap())),
//Shopping.io (SPI) 18
(Address::from_str("9b02dd390a603add5c07f9fd9175b7dabe8d63b7").unwrap(), (U256::from(214700),U256::from_dec_str("1000000000000000000").unwrap())),
//TE-FOOD/TustChain (TONE) 18
(Address::from_str("2Ab6Bb8408ca3199B8Fa6C92d5b455F820Af03c4").unwrap(), (U256::from(328),U256::from_dec_str("1000000000000000000").unwrap())),
//Nerve Network (NVT) 8
(Address::from_str("7b6f71c8b123b38aa8099e0098bec7fbc35b8a13").unwrap(), (U256::from(673),U256::from_dec_str("100000000").unwrap())),
//0xBitcoin Token (0xBTC) 8
(Address::from_str("b6ed7644c69416d67b522e20bc294a9a9b405b31").unwrap(), (U256::from(23200),U256::from_dec_str("100000000").unwrap())),
//Imported GBYTE (GBYTE) 18
(Address::from_str("31f69de127c8a0ff10819c0955490a4ae46fcc2a").unwrap(), (U256::from(221500),U256::from_dec_str("1000000000000000000").unwrap())),
//ArcBlock (ABT) 18
(Address::from_str("b98d4c97425d9908e66e53a6fdf673acca0be986").unwrap(), (U256::from(1708),U256::from_dec_str("1000000000000000000").unwrap())),
//QRL (QRL) 8
(Address::from_str("697beac28b09e122c4332d163985e8a73121b97f").unwrap(), (U256::from(2200),U256::from_dec_str("100000000").unwrap())),
//Lamden Tau (TAU) 18
(Address::from_str("c27a2f05fa577a83ba0fdb4c38443c0718356501").unwrap(), (U256::from(1166),U256::from_dec_str("1000000000000000000").unwrap())),
//InsurAce (INSUR) 18
(Address::from_str("544c42fbb96b39b21df61cf322b5edc285ee7429").unwrap(), (U256::from(9521),U256::from_dec_str("1000000000000000000").unwrap())),
//stakedETH (stETH) 18
(Address::from_str("dfe66b14d37c77f4e9b180ceb433d1b164f0281d").unwrap(), (U256::from(2532400),U256::from_dec_str("1000000000000000000").unwrap())),
//iQeon (IQN) 18
(Address::from_str("0db8d8b76bc361bacbb72e2c491e06085a97ab31").unwrap(), (U256::from(29700),U256::from_dec_str("1000000000000000000").unwrap())),
//Raiden (RDN) 18
(Address::from_str("255aa6df07540cb5d3d297f0d0d4d84cb52bc8e6").unwrap(), (U256::from(3169),U256::from_dec_str("1000000000000000000").unwrap())),
//Dentacoin (Dentacoin) 0
(Address::from_str("08d32b0da63e2C3bcF8019c9c5d849d7a9d791e6").unwrap(), (U256::from(0),U256::from_dec_str("1").unwrap())),
//Amber (AMB) 18
(Address::from_str("4dc3643dbc642b72c158e7f3d2ff232df61cb6ce").unwrap(), (U256::from(298),U256::from_dec_str("1000000000000000000").unwrap())),
//PoolTogether (POOL) 18
(Address::from_str("0cec1a9154ff802e7934fc916ed7ca50bde6844e").unwrap(), (U256::from(51500),U256::from_dec_str("1000000000000000000").unwrap())),
//EligmaToken (ELI) 18
(Address::from_str("c7c03b8a3fc5719066e185ea616e87b88eba44a3").unwrap(), (U256::from(637),U256::from_dec_str("1000000000000000000").unwrap())),
//dHedge DAO Token (DHT) 18
(Address::from_str("ca1207647Ff814039530D7d35df0e1Dd2e91Fa84").unwrap(), (U256::from(7413),U256::from_dec_str("1000000000000000000").unwrap())),
//Nebulas (NAS) 18
(Address::from_str("5d65D971895Edc438f465c17DB6992698a52318D").unwrap(), (U256::from(3301),U256::from_dec_str("1000000000000000000").unwrap())),
//Oraichain Token (ORAI) 18
(Address::from_str("4c11249814f11b9346808179cf06e71ac328c1b5").unwrap(), (U256::from(67600),U256::from_dec_str("1000000000000000000").unwrap())),
//PIKA (PIKA) 18
(Address::from_str("60f5672a271c7e39e787427a18353ba59a4a3578").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//GoBlank Token (BLANK) 18
(Address::from_str("41a3dba3d677e573636ba691a70ff2d606c29666").unwrap(), (U256::from(6661),U256::from_dec_str("1000000000000000000").unwrap())),
//Offshift (XFT) 18
(Address::from_str("abe580e7ee158da464b51ee1a83ac0289622e6be").unwrap(), (U256::from(32100),U256::from_dec_str("1000000000000000000").unwrap())),
//Antimatter.Finance Governance Token (MATTER) 18
(Address::from_str("9b99cca871be05119b2012fd4474731dd653febe").unwrap(), (U256::from(4577),U256::from_dec_str("1000000000000000000").unwrap())),
//Cashaa (CAS) 18
(Address::from_str("e8780b48bdb05f928697a5e8155f672ed91462f7").unwrap(), (U256::from(174),U256::from_dec_str("1000000000000000000").unwrap())),
//Tranche Finance (SLICE) 18
(Address::from_str("0aee8703d34dd9ae107386d3eff22ae75dd616d1").unwrap(), (U256::from(8162),U256::from_dec_str("1000000000000000000").unwrap())),
//Arcona Distribution Contract (ARCONA) 18
(Address::from_str("0f71b8de197a1c84d31de0f1fa7926c365f052b3").unwrap(), (U256::from(9301),U256::from_dec_str("1000000000000000000").unwrap())),
//Morpheus Infrastructure Token (MITx) 18
(Address::from_str("4a527d8fc13c5203ab24ba0944f4cb14658d1db6").unwrap(), (U256::from(320),U256::from_dec_str("1000000000000000000").unwrap())),
//DappRadar (RADAR) 18
(Address::from_str("44709a920fccf795fbc57baa433cc3dd53c44dbe").unwrap(), (U256::from(329),U256::from_dec_str("1000000000000000000").unwrap())),
//Covesting (COV) 18
(Address::from_str("ADA86b1b313D1D5267E3FC0bB303f0A2b66D0Ea7").unwrap(), (U256::from(7274),U256::from_dec_str("1000000000000000000").unwrap())),
//BiFi (BiFi) 18
(Address::from_str("2791bfd60d232150bff86b39b7146c0eaaa2ba81").unwrap(), (U256::from(535),U256::from_dec_str("1000000000000000000").unwrap())),

//ProBit Token (PROB) 18
(Address::from_str("fb559ce67ff522ec0b9ba7f5dc9dc7ef6c139803").unwrap(), (U256::from(3516),U256::from_dec_str("1000000000000000000").unwrap())),
//RAE Token (RAE) 18
(Address::from_str("e5a3229ccb22b6484594973a03a3851dcd948756").unwrap(), (U256::from(20000),U256::from_dec_str("1000000000000000000").unwrap())),
//Jenny Metaverse DAO Token (uJENNY) 18
(Address::from_str("a499648fd0e80fd911972bbeb069e4c20e68bf22").unwrap(), (U256::from(13300),U256::from_dec_str("1000000000000000000").unwrap())),
//Cerby Token (CERBY) 18
(Address::from_str("def1fac7bf08f173d286bbbdcbeeade695129840").unwrap(), (U256::from(5),U256::from_dec_str("1000000000000000000").unwrap())),
//AnRKey X ($ANRX) 18
(Address::from_str("cae72a7a0fd9046cf6b165ca54c9e3a3872109e0").unwrap(), (U256::from(1124),U256::from_dec_str("1000000000000000000").unwrap())),
//Ethereans (OS) 18
(Address::from_str("6100dd79fcaa88420750dcee3f735d168abcb771").unwrap(), (U256::from(235700),U256::from_dec_str("1000000000000000000").unwrap())),
//Atomic Wallet Token (AWC) 8
(Address::from_str("ad22f63404f7305e4713ccbd4f296f34770513f4").unwrap(), (U256::from(11620),U256::from_dec_str("100000000").unwrap())),
//ANGLE (ANGLE) 18
(Address::from_str("31429d1856ad1377a8a0079410b297e1a9e214c2").unwrap(), (U256::from(3357),U256::from_dec_str("1000000000000000000").unwrap())),
//GOVI (GOVI) 18
(Address::from_str("eeaa40b28a2d1b0b08f6f97bb1dd4b75316c6107").unwrap(), (U256::from(12249),U256::from_dec_str("1000000000000000000").unwrap())),
//BTC 2x Flexible Leverage Index (BTC2x-FLI) 18
(Address::from_str("0b498ff89709d3838a063f1dfa463091f9801c2b").unwrap(), (U256::from(453600),U256::from_dec_str("1000000000000000000").unwrap())),
//Lympo Market Token (LMT) 18
(Address::from_str("327673ae6b33bd3d90f0096870059994f30dc8af").unwrap(), (U256::from(1598),U256::from_dec_str("1000000000000000000").unwrap())),
//UnmarshalToken (MARSH) 18
(Address::from_str("5a666c7d92e5fa7edcb6390e4efd6d0cdd69cf37").unwrap(), (U256::from(5605),U256::from_dec_str("1000000000000000000").unwrap())),
//Armor (ARMOR) 18
(Address::from_str("1337def16f9b486faed0293eb623dc8395dfe46a").unwrap(), (U256::from(655),U256::from_dec_str("1000000000000000000").unwrap())),
//UTN-P: Universa Token (UTNP) 18
(Address::from_str("9e3319636e2126e3c0bc9e3134aec5e1508a46c7").unwrap(), (U256::from(37),U256::from_dec_str("1000000000000000000").unwrap())),
//WaBi (WaBi) 18
(Address::from_str("286BDA1413a2Df81731D4930ce2F862a35A609fE").unwrap(), (U256::from(1986),U256::from_dec_str("1000000000000000000").unwrap())),
//Jarvis Reward Token (JRT) 18
(Address::from_str("8a9c67fee641579deba04928c4bc45f66e26343a").unwrap(), (U256::from(548),U256::from_dec_str("1000000000000000000").unwrap())),
//Knoxstertoken (FKX) 18
(Address::from_str("16484d73Ac08d2355F466d448D2b79D2039F6EBB").unwrap(), (U256::from(758),U256::from_dec_str("1000000000000000000").unwrap())),
//Geeq (GEEQ) 18
(Address::from_str("6B9f031D718dDed0d681c20cB754F97b3BB81b78").unwrap(), (U256::from(9984),U256::from_dec_str("1000000000000000000").unwrap())),
//Aurora (AOA) 18
(Address::from_str("9ab165d795019b6d8b3e971dda91071421305e5a").unwrap(), (U256::from(26),U256::from_dec_str("1000000000000000000").unwrap())),
//Genesis Pool (GPOOL) 18
(Address::from_str("797de1dc0b9faf5e25c1f7efe8df9599138fa09d").unwrap(), (U256::from(317),U256::from_dec_str("1000000000000000000").unwrap())),
//OpenANX (OAX) 18
(Address::from_str("701c244b988a513c945973defa05de933b23fe1d").unwrap(), (U256::from(1923),U256::from_dec_str("1000000000000000000").unwrap())),
//Moeda Loyalty Points (MDA) 18
(Address::from_str("51db5ad35c671a87207d88fc11d593ac0c8415bd").unwrap(), (U256::from(5633),U256::from_dec_str("1000000000000000000").unwrap())),
//Salt (SALT) 8
(Address::from_str("4156D3342D5c385a87D264F90653733592000581").unwrap(), (U256::from(1249),U256::from_dec_str("100000000").unwrap())),
//1-UP (1-UP) 18
(Address::from_str("c86817249634ac209bc73fca1712bbd75e37407d").unwrap(), (U256::from(1813),U256::from_dec_str("1000000000000000000").unwrap())),
//KAN (KAN) 18
(Address::from_str("1410434b0346f5be678d0fb554e5c7ab620f8f4a").unwrap(), (U256::from(20),U256::from_dec_str("1000000000000000000").unwrap())),
//Plasma (PPAY) 18
(Address::from_str("054D64b73d3D8A21Af3D764eFd76bCaA774f3Bb2").unwrap(), (U256::from(699),U256::from_dec_str("1000000000000000000").unwrap())),
//Monetha (MTH) 5
(Address::from_str("af4dce16da2877f8c9e00544c93b62ac40631f16").unwrap(), (U256::from(304),U256::from_dec_str("100000").unwrap())),
//Free Coin (FREE) 18
(Address::from_str("2f141ce366a2462f02cea3d12cf93e4dca49e4fd").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//Pluton (PLU) 18
(Address::from_str("D8912C10681D8B21Fd3742244f44658dBA12264E").unwrap(), (U256::from(56700),U256::from_dec_str("1000000000000000000").unwrap())),
//CRPT (CRPT) 18
(Address::from_str("08389495d7456e1951ddf7c3a1314a4bfb646d8b").unwrap(), (U256::from(1233),U256::from_dec_str("1000000000000000000").unwrap())),
//Pinknode Token (PNODE) 18
(Address::from_str("af691508ba57d416f895e32a1616da1024e882d2").unwrap(), (U256::from(1082),U256::from_dec_str("1000000000000000000").unwrap())),
//Strips Token (STRP) 18
(Address::from_str("97872eafd79940c7b24f7bcc1eadb1457347adc9").unwrap(), (U256::from(38900),U256::from_dec_str("1000000000000000000").unwrap())),
//ZTCoin (ZT) 18
(Address::from_str("fe39e6a32acd2af7955cb3d406ba2b55c901f247").unwrap(), (U256::from(203),U256::from_dec_str("1000000000000000000").unwrap())),
//Monolith (TKN) 8
(Address::from_str("aaaf91d9b90df800df4f55c205fd6989c977e73a").unwrap(), (U256::from(2736),U256::from_dec_str("100000000").unwrap())),
//Standard (STND) 18
(Address::from_str("9040e237c3bf18347bb00957dc22167d0f2b999d").unwrap(), (U256::from(3536),U256::from_dec_str("1000000000000000000").unwrap())),
//Tidal Token (TIDAL) 18
(Address::from_str("29cbd0510eec0327992cd6006e63f9fa8e7f33b7").unwrap(), (U256::from(42),U256::from_dec_str("1000000000000000000").unwrap())),
//Jigstack (STAK) 18
(Address::from_str("1f8a626883d7724dbd59ef51cbd4bf1cf2016d13").unwrap(), (U256::from(84),U256::from_dec_str("1000000000000000000").unwrap())),
//Furucombo (COMBO) 18
(Address::from_str("ffffffff2ba8f66d4e51811c5190992176930278").unwrap(), (U256::from(3955),U256::from_dec_str("1000000000000000000").unwrap())),
//LAtoken (LA) 18
(Address::from_str("e50365f5d679cb98a1dd62d6f6e58e59321bcddf").unwrap(), (U256::from(1598),U256::from_dec_str("1000000000000000000").unwrap())),
//Fair Token (FAIR) 18
(Address::from_str("9b20dabcec77f6289113e61893f7beefaeb1990a").unwrap(), (U256::from(136),U256::from_dec_str("1000000000000000000").unwrap())),
//Smart Advertising Transaction Token (SATT) 18
(Address::from_str("df49c9f599a0a9049d97cff34d0c30e468987389").unwrap(), (U256::from(22),U256::from_dec_str("1000000000000000000").unwrap())),
//Digg (DIGG) 9
(Address::from_str("798d1be841a82a273720ce31c822c61a67a601c3").unwrap(), (U256::from(376000000),U256::from_dec_str("1000000000").unwrap())),
//Float Bank (BANK) 18
(Address::from_str("24a6a37576377f63f194caa5f518a60f45b42921").unwrap(), (U256::from(708400),U256::from_dec_str("1000000000000000000").unwrap())),
//Airbloc (ABL) 18
(Address::from_str("f8b358b3397a8ea5464f8cc753645d42e14b79ea").unwrap(), (U256::from(337),U256::from_dec_str("1000000000000000000").unwrap())),
//Unido (UDO) 18
(Address::from_str("ea3983fc6d0fbbc41fb6f6091f68f3e08894dc06").unwrap(), (U256::from(1286),U256::from_dec_str("1000000000000000000").unwrap())),
//Lambda (LAMB) 18
(Address::from_str("8971f9fd7196e5cee2c1032b50f656855af7dd26").unwrap(), (U256::from(60),U256::from_dec_str("1000000000000000000").unwrap())),
//Origin Dollar (OUSD) 18
(Address::from_str("2a8e1e676ec238d8a992307b495b45b3feaa5e86").unwrap(), (U256::from(9956),U256::from_dec_str("1000000000000000000").unwrap())),
//SPANK (SPANK) 18
(Address::from_str("42d6622dece394b54999fbd73d108123806f6a18").unwrap(), (U256::from(133),U256::from_dec_str("1000000000000000000").unwrap())),
//DivergenceProtocol (DIVER) 18
(Address::from_str("fb782396c9b20e564a64896181c7ac8d8979d5f4").unwrap(), (U256::from(1573),U256::from_dec_str("1000000000000000000").unwrap())),
//Public Mint (MINT) 18
(Address::from_str("0cdf9acd87e940837ff21bb40c9fd55f68bba059").unwrap(), (U256::from(1207),U256::from_dec_str("1000000000000000000").unwrap())),

//Float Bank (BANK) 18
(Address::from_str("24a6a37576377f63f194caa5f518a60f45b42921").unwrap(), (U256::from(707600),U256::from_dec_str("1000000000000000000").unwrap())),
//Airbloc (ABL) 18
(Address::from_str("f8b358b3397a8ea5464f8cc753645d42e14b79ea").unwrap(), (U256::from(335),U256::from_dec_str("1000000000000000000").unwrap())),
//Unido (UDO) 18
(Address::from_str("ea3983fc6d0fbbc41fb6f6091f68f3e08894dc06").unwrap(), (U256::from(1286),U256::from_dec_str("1000000000000000000").unwrap())),
//Lambda (LAMB) 18
(Address::from_str("8971f9fd7196e5cee2c1032b50f656855af7dd26").unwrap(), (U256::from(60),U256::from_dec_str("1000000000000000000").unwrap())),
//Origin Dollar (OUSD) 18
(Address::from_str("2a8e1e676ec238d8a992307b495b45b3feaa5e86").unwrap(), (U256::from(9956),U256::from_dec_str("1000000000000000000").unwrap())),
//SPANK (SPANK) 18
(Address::from_str("42d6622dece394b54999fbd73d108123806f6a18").unwrap(), (U256::from(133),U256::from_dec_str("1000000000000000000").unwrap())),
//DivergenceProtocol (DIVER) 18
(Address::from_str("fb782396c9b20e564a64896181c7ac8d8979d5f4").unwrap(), (U256::from(1573),U256::from_dec_str("1000000000000000000").unwrap())),
//Public Mint (MINT) 18
(Address::from_str("0cdf9acd87e940837ff21bb40c9fd55f68bba059").unwrap(), (U256::from(1207),U256::from_dec_str("1000000000000000000").unwrap())),
//PCHAIN (PAI) 18
(Address::from_str("b9bb08ab7e9fa0a1356bd4a39ec0ca267e03b0b3").unwrap(), (U256::from(103),U256::from_dec_str("1000000000000000000").unwrap())),
//SwftCoin (SWFTC) 8
(Address::from_str("0bb217e40f8a5cb79adf04e1aab60e5abd0dfc1e").unwrap(), (U256::from(13),U256::from_dec_str("100000000").unwrap())),
//Wirex Token (WXT) 18
(Address::from_str("a02120696c7b8fe16c09c749e4598819b2b0e915").unwrap(), (U256::from(36),U256::from_dec_str("1000000000000000000").unwrap())),
//Voice Token (VOICE) 18
(Address::from_str("2e2364966267B5D7D2cE6CD9A9B5bD19d9C7C6A9").unwrap(), (U256::from(2316300),U256::from_dec_str("1000000000000000000").unwrap())),
//Rupiah Token (IDRT) 2
(Address::from_str("998FFE1E43fAcffb941dc337dD0468d52bA5b48A").unwrap(), (U256::from(1),U256::from_dec_str("100").unwrap())),
//Compound Wrapped BTC (cWBTC) 8
(Address::from_str("C11b1268C1A384e55C48c2391d8d480264A3A7F4").unwrap(), (U256::from(9811400),U256::from_dec_str("100000000").unwrap())),
//Decentr (DEC) 18
(Address::from_str("30f271C9E86D2B7d00a6376Cd96A1cFBD5F0b9b3").unwrap(), (U256::from(801),U256::from_dec_str("1000000000000000000").unwrap())),
//TrueFlip (TFL) 8
(Address::from_str("a7f976c360ebbed4465c2855684d1aae5271efa9").unwrap(), (U256::from(11800),U256::from_dec_str("100000000").unwrap())),
//VIB (VIB) 18
(Address::from_str("2C974B2d0BA1716E644c1FC59982a89DDD2fF724").unwrap(), (U256::from(429),U256::from_dec_str("1000000000000000000").unwrap())),
//QuadrantProtocol (eQUAD) 18
(Address::from_str("c28e931814725bbeb9e670676fabbcb694fe7df2").unwrap(), (U256::from(141),U256::from_dec_str("1000000000000000000").unwrap())),
//StakeWise (SWISE) 18
(Address::from_str("48c3399719b582dd63eb5aadf12a40b4c3f52fa2").unwrap(), (U256::from(1183),U256::from_dec_str("1000000000000000000").unwrap())),
//Dapp Token (DAPPT) 18
(Address::from_str("96184d9C811Ea0624fC30C80233B1d749B9E485B").unwrap(), (U256::from(52),U256::from_dec_str("1000000000000000000").unwrap())),
//BHPCash (BHPC) 18
(Address::from_str("ee74110fb5a1007b06282e0de5d73a61bf41d9cd").unwrap(), (U256::from(3641),U256::from_dec_str("1000000000000000000").unwrap())),
//Vibe Coin (VIBE) 18
(Address::from_str("e8ff5c9c75deb346acac493c463c8950be03dfba").unwrap(), (U256::from(391),U256::from_dec_str("1000000000000000000").unwrap())),
//Leverj Gluon (L2) 18
(Address::from_str("bbff34e47e559ef680067a6b1c980639eeb64d24").unwrap(), (U256::from(250),U256::from_dec_str("1000000000000000000").unwrap())),
//Falcon (FNT) 6
(Address::from_str("dc5864ede28bd4405aa04d93e05a0531797d9d59").unwrap(), (U256::from(7),U256::from_dec_str("1000000").unwrap())),
//Ixs Token (IXS) 18
(Address::from_str("73d7c860998ca3c01ce8c808f5577d94d545d1b4").unwrap(), (U256::from(1849),U256::from_dec_str("1000000000000000000").unwrap())),
//UREEQA Token (URQA) 18
(Address::from_str("1735db6ab5baa19ea55d0adceed7bcdc008b3136").unwrap(), (U256::from(1733),U256::from_dec_str("1000000000000000000").unwrap())),
//PieDAO DOUGH v2 (DOUGH) 18
(Address::from_str("ad32A8e6220741182940c5aBF610bDE99E737b2D").unwrap(), (U256::from(4552),U256::from_dec_str("1000000000000000000").unwrap())),
//AurusDeFi (AWX) 18
(Address::from_str("a51fc71422a30fa7ffa605b360c3b283501b5bf6").unwrap(), (U256::from(21100),U256::from_dec_str("1000000000000000000").unwrap())),
//NFT INDEX (NFTI) 18
(Address::from_str("e5feeac09d36b18b3fa757e5cf3f8da6b8e27f4c").unwrap(), (U256::from(30077118),U256::from_dec_str("1000000000000000000").unwrap())),
//Enigma (ENG) 8
(Address::from_str("f0ee6b27b759c9893ce4f094b49ad28fd15a23e4").unwrap(), (U256::from(803),U256::from_dec_str("100000000").unwrap())),
//AMLT (AMLT) 18
(Address::from_str("ca0e7269600d353f70b14ad118a49575455c0f2f").unwrap(), (U256::from(196),U256::from_dec_str("1000000000000000000").unwrap())),
//YUKI (YUKI) 8
(Address::from_str("5ab793e36070f0fac928ea15826b0c1bc5365119").unwrap(), (U256::from(5),U256::from_dec_str("100000000").unwrap())),
//Tierion Network Token (TNT) 8
(Address::from_str("08f5a9235b08173b7569f83645d2c7fb55e8ccd8").unwrap(), (U256::from(152),U256::from_dec_str("100000000").unwrap())),
//SpaceChain (SPC) 18
(Address::from_str("8069080a922834460c3a092fb2c1510224dc066b").unwrap(), (U256::from(158),U256::from_dec_str("1000000000000000000").unwrap())),
//TOKPIE (TKP) 18
(Address::from_str("d31695a1d35e489252ce57b129fd4b1b05e6acac").unwrap(), (U256::from(806),U256::from_dec_str("1000000000000000000").unwrap())),
//MATRIX AI Network (MAN) 18
(Address::from_str("e25bcec5d3801ce3a794079bf94adf1b8ccd802d").unwrap(), (U256::from(424),U256::from_dec_str("1000000000000000000").unwrap())),
//Tokenomy (TEN) 18
(Address::from_str("dd16ec0f66e54d453e6756713e533355989040e4").unwrap(), (U256::from(538),U256::from_dec_str("1000000000000000000").unwrap())),
//Coinvest COIN V3 Token (COIN) 18
(Address::from_str("eb547ed1D8A3Ff1461aBAa7F0022FED4836E00A4").unwrap(), (U256::from(1963),U256::from_dec_str("1000000000000000000").unwrap())),
//Yee - A Blockchain-powered &amp; Cloud-based Socia (YEE) 18
(Address::from_str("922105fad8153f516bcfb829f56dc097a0e1d705").unwrap(), (U256::from(20),U256::from_dec_str("1000000000000000000").unwrap())),
//Blockchain Certified Data Token (BCDT) 18
(Address::from_str("acfa209fb73bf3dd5bbfb1101b9bc999c49062a5").unwrap(), (U256::from(1648),U256::from_dec_str("1000000000000000000").unwrap())),
//Everex (EVX) 4
(Address::from_str("f3db5fa2c66b7af3eb0c0b782510816cbe4813b8").unwrap(), (U256::from(2746),U256::from_dec_str("10000").unwrap())),
//TenXPay (PAY) 18
(Address::from_str("B97048628DB6B661D4C2aA833e95Dbe1A905B280").unwrap(), (U256::from(508),U256::from_dec_str("1000000000000000000").unwrap())),
//Pawthereum (PAWTH) 9
(Address::from_str("aecc217a749c2405b5ebc9857a16d58bdc1c367f").unwrap(), (U256::from(85),U256::from_dec_str("1000000000").unwrap())),
//RipioCreditNetwork (RCN) 18
(Address::from_str("f970b8e36e23f7fc3fd752eea86f8be8d83375a6").unwrap(), (U256::from(109),U256::from_dec_str("1000000000000000000").unwrap())),
//Bloom (BLT) 18
(Address::from_str("107c4504cd79c5d2696ea0030a8dd4e92601b82e").unwrap(), (U256::from(1018),U256::from_dec_str("1000000000000000000").unwrap())),
//Insights Network (INSTAR) 18
(Address::from_str("c72fe8e3dd5bef0f9f31f259399f301272ef2a2d").unwrap(), (U256::from(291),U256::from_dec_str("1000000000000000000").unwrap())),
//ChangeNOW (NOW) 8
(Address::from_str("e9a95d175a5f4c9369f3b74222402eb1b837693b").unwrap(), (U256::from(683),U256::from_dec_str("100000000").unwrap())),
//CREDITS (CS) 6
(Address::from_str("46b9ad944d1059450da1163511069c718f699d31").unwrap(), (U256::from(253),U256::from_dec_str("1000000").unwrap())),
//XIO Network (XIO) 18
(Address::from_str("0f7F961648aE6Db43C75663aC7E5414Eb79b5704").unwrap(), (U256::from(1454),U256::from_dec_str("1000000000000000000").unwrap())),
//DOVU (DOV) 18
(Address::from_str("ac3211a5025414af2866ff09c23fc18bc97e79b1").unwrap(), (U256::from(176),U256::from_dec_str("1000000000000000000").unwrap())),

//Hakka Finance (HAKKA) 18
(Address::from_str("0E29e5AbbB5FD88e28b2d355774e73BD47dE3bcd").unwrap(), (U256::from(183),U256::from_dec_str("1000000000000000000").unwrap())),
//Internet Node Token (INT) 6
(Address::from_str("0b76544f6c413a555f309bf76260d1e02377c02a").unwrap(), (U256::from(109),U256::from_dec_str("1000000").unwrap())),
//BIXToken (BIX) 18
(Address::from_str("009c43b42aefac590c719e971020575974122803").unwrap(), (U256::from(442),U256::from_dec_str("1000000000000000000").unwrap())),
//TOP Network (TOP) 18
(Address::from_str("dcd85914b8ae28c1e62f1c488e1d968d5aaffe2b").unwrap(), (U256::from(9),U256::from_dec_str("1000000000000000000").unwrap())),
//Matryx (MTX) 18
(Address::from_str("0af44e2784637218dd1d32a322d44e603a8f0c6a").unwrap(), (U256::from(2102),U256::from_dec_str("1000000000000000000").unwrap())),
//CAPP Token (CAPP) 2
(Address::from_str("11613b1f840bb5A40F8866d857e24DA126B79D73").unwrap(), (U256::from(73),U256::from_dec_str("100").unwrap())),
//Cappasity (CAPP) 2
(Address::from_str("04f2e7221fdb1b52a68169b25793e51478ff0329").unwrap(), (U256::from(73),U256::from_dec_str("100").unwrap())),
//Revain (REV) 6
(Address::from_str("2ef52Ed7De8c5ce03a4eF0efbe9B7450F2D7Edc9").unwrap(), (U256::from(99),U256::from_dec_str("1000000").unwrap())),
//ZMINE Token (ZMN) 18
(Address::from_str("554ffc77f4251a9fb3c0e3590a6a205f8d4e067d").unwrap(), (U256::from(78),U256::from_dec_str("1000000000000000000").unwrap())),
//Hiveterminal Token (HVN) 8
(Address::from_str("C0Eb85285d83217CD7c891702bcbC0FC401E2D9D").unwrap(), (U256::from(132),U256::from_dec_str("100000000").unwrap())),
//AppCoins (APPC) 18
(Address::from_str("1a7a8bd9106f2b8d977e08582dc7d24c723ab0db").unwrap(), (U256::from(400),U256::from_dec_str("1000000000000000000").unwrap())),
//Impermax (IMX) 18
(Address::from_str("7b35ce522cb72e4077baeb96cb923a5529764a00").unwrap(), (U256::from(2016),U256::from_dec_str("1000000000000000000").unwrap())),
//ClinTex (CTI) 18
(Address::from_str("8c18D6a985Ef69744b9d57248a45c0861874f244").unwrap(), (U256::from(558),U256::from_dec_str("1000000000000000000").unwrap())),
//CyberMiles (CMT) 18
(Address::from_str("f85feea2fdd81d51177f6b8f35f0e6734ce45f5f").unwrap(), (U256::from(57),U256::from_dec_str("1000000000000000000").unwrap())),
//indaHash Coin (IDH) 6
(Address::from_str("5136c98a80811c3f46bdda8b5c4555cfd9f812f0").unwrap(), (U256::from(138),U256::from_dec_str("1000000").unwrap())),
//Herocoin (PLAY) 18
(Address::from_str("e477292f1b3268687a29376116b0ed27a9c76170").unwrap(), (U256::from(305),U256::from_dec_str("1000000000000000000").unwrap())),
//Spendcoin (SPND) 18
(Address::from_str("ddd460bbd9f79847ea08681563e8a9696867210c").unwrap(), (U256::from(47),U256::from_dec_str("1000000000000000000").unwrap())),
//ODEM Token (ODEM) 18
(Address::from_str("bf52f2ab39e26e0951d2a02b49b7702abe30406a").unwrap(), (U256::from(201),U256::from_dec_str("1000000000000000000").unwrap())),
//Carbon (CRBN) 18
(Address::from_str("Cdeee767beD58c5325f68500115d4B722b3724EE").unwrap(), (U256::from(1369),U256::from_dec_str("1000000000000000000").unwrap())),
//Float Protocol: FLOAT (FLOAT) 18
(Address::from_str("b05097849bca421a3f51b249ba6cca4af4b97cb9").unwrap(), (U256::from(16299),U256::from_dec_str("1000000000000000000").unwrap())),
//PILLAR (PLR) 18
(Address::from_str("e3818504c1b32bf1557b16c238b2e01fd3149c17").unwrap(), (U256::from(172),U256::from_dec_str("1000000000000000000").unwrap())),
//Genaro X (GNX) 9
(Address::from_str("6ec8a24cabdc339a06a172f8223ea557055adaa5").unwrap(), (U256::from(146),U256::from_dec_str("1000000000").unwrap())),
//GHOST (GHOST) 18
(Address::from_str("4c327471C44B2dacD6E90525f9D629bd2e4f662C").unwrap(), (U256::from(2548),U256::from_dec_str("1000000000000000000").unwrap())),
//NapoleonX (NPX) 2
(Address::from_str("28b5e12cce51f15594b0b91d5b5adaa70f684a02").unwrap(), (U256::from(1708),U256::from_dec_str("100").unwrap())),
//Bundles (BUND) 18
(Address::from_str("8D3E855f3f55109D473735aB76F753218400fe96").unwrap(), (U256::from(504600),U256::from_dec_str("1000000000000000000").unwrap())),
//Woofy (WOOFY) 12
(Address::from_str("d0660cd418a64a1d44e9214ad8e459324d8157f1").unwrap(), (U256::from(346),U256::from_dec_str("1000000000000").unwrap())),
//QunQunCommunities (QUN) 18
(Address::from_str("264dc2dedcdcbb897561a57cba5085ca416fb7b4").unwrap(), (U256::from(63),U256::from_dec_str("1000000000000000000").unwrap())),
//Edgeless (EDG) 0
(Address::from_str("08711d3b02c8758f2fb3ab4e80228418a7f8e39c").unwrap(), (U256::from(368),U256::from_dec_str("1").unwrap())),
//Compound Sai (cSAI) 8
(Address::from_str("f5dce57282a584d2746faf1593d3121fcac444dc").unwrap(), (U256::from(5097),U256::from_dec_str("100000000").unwrap())),
//HPBCoin (HPB) 18
(Address::from_str("38c6a68304cdefb9bec48bbfaaba5c5b47818bb2").unwrap(), (U256::from(1078),U256::from_dec_str("1000000000000000000").unwrap())),
//Block-Chain.com Token (BC) 18
(Address::from_str("2ecb13a8c458c379c4d9a7259e202de03c8f3d19").unwrap(), (U256::from(198),U256::from_dec_str("1000000000000000000").unwrap())),
//CryptalDash (CRD) 18
(Address::from_str("caaa93712bdac37f736c323c93d4d5fdefcc31cc").unwrap(), (U256::from(40),U256::from_dec_str("1000000000000000000").unwrap())),
//VeriSafe (VSF) 18
(Address::from_str("ac9ce326e95f51b5005e9fe1dd8085a01f18450c").unwrap(), (U256::from(5),U256::from_dec_str("1000000000000000000").unwrap())),
//cVToken (cV) 18
(Address::from_str("50bC2Ecc0bfDf5666640048038C1ABA7B7525683").unwrap(), (U256::from(5),U256::from_dec_str("1000000000000000000").unwrap())),
//Egretia (EGT) 18
(Address::from_str("8e1b448ec7adfc7fa35fc2e885678bd323176e34").unwrap(), (U256::from(10),U256::from_dec_str("1000000000000000000").unwrap())),
//Signata (SATA) 18
(Address::from_str("3ebb4a4e91ad83be51f8d596533818b246f4bee1").unwrap(), (U256::from(1945),U256::from_dec_str("1000000000000000000").unwrap())),
//HitchainCoin (HIT) 6
(Address::from_str("7995ab36bb307afa6a683c24a25d90dc1ea83566").unwrap(), (U256::from(1),U256::from_dec_str("1000000").unwrap())),
//ZAP TOKEN (ZAP) 18
(Address::from_str("6781a0f84c7e9e846dcb84a9a5bd49333067b104").unwrap(), (U256::from(151),U256::from_dec_str("1000000000000000000").unwrap())),
//PumaPay (PMA) 18
(Address::from_str("846c66cf71c43f80403b51fe3906b3599d63336f").unwrap(), (U256::from(1),U256::from_dec_str("1000000000000000000").unwrap())),
//nDEX (NDX) 18
(Address::from_str("1966d718a565566e8e202792658d7b5ff4ece469").unwrap(), (U256::from(2),U256::from_dec_str("1000000000000000000").unwrap())),
//Simple Token (ST) 18
(Address::from_str("2c4e8f2d746113d0696ce89b35f0d8bf88e0aeca").unwrap(), (U256::from(53),U256::from_dec_str("1000000000000000000").unwrap())),
//RED MWAT (MWAT) 18
(Address::from_str("6425c6be902d692ae2db752b3c268afadb099d3b").unwrap(), (U256::from(79),U256::from_dec_str("1000000000000000000").unwrap())),
//CUBE (AUTO) 18
(Address::from_str("622dFfCc4e83C64ba959530A5a5580687a57581b").unwrap(), (U256::from(5),U256::from_dec_str("1000000000000000000").unwrap())),
//Public Index Network (PIN) 18
(Address::from_str("c1f976b91217e240885536af8b63bc8b5269a9be").unwrap(), (U256::from(225),U256::from_dec_str("1000000000000000000").unwrap())),
//https://unimex.network/ (UMX) 18
(Address::from_str("10be9a8dae441d276a5027936c3aaded2d82bc15").unwrap(), (U256::from(4719),U256::from_dec_str("1000000000000000000").unwrap())),
//DOS Network Token (DOS) 18
(Address::from_str("0A913beaD80F321E7Ac35285Ee10d9d922659cB7").unwrap(), (U256::from(265),U256::from_dec_str("1000000000000000000").unwrap())),
//Tadpole (TAD) 18
(Address::from_str("9f7229aF0c4b9740e207Ea283b9094983f78ba04").unwrap(), (U256::from(85200),U256::from_dec_str("1000000000000000000").unwrap())),
//Shadows Network (DOWS) 18
(Address::from_str("661ab0ed68000491d98c796146bcf28c20d7c559").unwrap(), (U256::from(1297),U256::from_dec_str("1000000000000000000").unwrap())),
//Quantum (QAU) 8
(Address::from_str("671abbe5ce652491985342e85428eb1b07bc6c64").unwrap(), (U256::from(455),U256::from_dec_str("100000000").unwrap())),
//LGO Token (LGO) 8
(Address::from_str("0a50c93c762fdd6e56d86215c24aaad43ab629aa").unwrap(), (U256::from(208),U256::from_dec_str("100000000").unwrap())),

        ].iter().cloned().collect();
    pub static ref FLASH_LOAN_CONTRACT_ADDRESSES: Vec<Address> = vec![
                                                                      //fake_solomargin.sol for testing TODO: remove after done
                                                                      Address::from_str("3DD0864668C36D27B53a98137764c99F9FD5B7B2").unwrap(),
                                                                      //dYdX: Solo Margin
                                                                      Address::from_str("1e0447b19bb6ecfdae1e4ae1694b0c3659614e4e").unwrap(),
                                                                      //Aave: Lending Pool Core V1
                                                                      Address::from_str("3dfd23A6c5E8BbcFc9581d2E864a68feb6a076d3").unwrap(),
                                                                     ];
    // //Token Address Vec
    // pub static ref CONTRACT_ADDRESSES: Vec<Address> = vec![//"Token A" from erc.sol for testing TODO: remove after done
    //                                                        Address::from_str("b4c79daB8f259C7Aee6E5b2Aa729821864227e84").unwrap(),
    //                                                        //"Token B" from erc.sol for testing TODO: remove after done
    //                                                        Address::from_str("ee35211C4D9126D520bBfeaf3cFee5FE7B86F221").unwrap(),
    //                                                        //ETH
    //                                                        Address::from_str("0000000000000000000000000000000000000001").unwrap(),
    //                                                        //DAI
    //                                                        Address::from_str("6B175474E89094C44Da98b954EedeAC495271d0F").unwrap(), 
    //                                                        //BSDS
    //                                                        Address::from_str("e7c9c188138f7d70945d420d75f8ca7d8ab9c700").unwrap(),
    //                                                         //WETH
    //                                                         Address::from_str("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(), 
    //                                                         //Wrapped BTC
    //                                                         Address::from_str("2260fac5e5542a773aa44fbcfedf7c193bc2c599").unwrap(),
    //                                                         //TBB
    //                                                         Address::from_str("4a7adcb083fe5e3d6b58edc3d260e2e61668e7a2").unwrap(),  
    //                                                         //USDT
    //                                                         Address::from_str("dac17f958d2ee523a2206206994597c13d831ec7").unwrap(),            
    //                                                         //O3
    //                                                         Address::from_str("ee9801669c6138e84bd50deb500827b776777d28").unwrap(),
    //                                                         //LEND
    //                                                         Address::from_str("80fb784b7ed66730e8b1dbd9820afd29931aab03").unwrap(),
    //                                                         //LINK
    //                                                         Address::from_str("514910771af9ca656af840dff83e8264ecf986ca").unwrap(),
    //                                                         //TEND
    //                                                         Address::from_str("1453dbb8a29551ade11d89825ca812e05317eaeb").unwrap(),
    //                                                       ];
    // //Token price $/0.0001USD rounded in 0.0001 dollar 
    // pub static ref TOKEN_USD_PRICES: Vec<U256> = vec![  //erc.sol for testing TODO: remove after done
    //                                                     U256::from(2000),
    //                                                     //erc.sol for testing TODO: remove after done
    //                                                     U256::from(1000),
    //                                                     //ETH
    //                                                     U256::from(43714500),
    //                                                     //DAI
    //                                                     U256::from(10000),
    //                                                     //BSDS
    //                                                     U256::from(900),
    //                                                     //WETH
    //                                                     U256::from(43714500),
    //                                                     //Wrapped BTC
    //                                                     U256::from(62022000),
    //                                                     //TBB
    //                                                     U256::from(1274200),
    //                                                     //USDT
    //                                                     U256::from(10000),
    //                                                     //O3
    //                                                     U256::from(10100),
    //                                                     //LEND
    //                                                     U256::from(18600),
    //                                                     //LINK
    //                                                     U256::from(204700),
    //                                                     //TEND
    //                                                     U256::from(300),
    //                                                 ];
    // //Token decimal points
    // pub static ref TOKEN_DECIMAL_POINTS: Vec<U256> = vec![  //erc.sol for testing TODO: remove after done
    //                                                     U256::from_dec_str("1").unwrap(),
    //                                                     //erc.sol for testing TODO: remove after done
    //                                                     U256::from_dec_str("1").unwrap(),
    //                                                     //ETH 18
    //                                                     U256::from_dec_str("1000000000000000000").unwrap(),
    //                                                     //DAI
    //                                                     U256::from_dec_str("10000").unwrap(),
    //                                                     //BSDS
    //                                                     U256::from_dec_str("900").unwrap(),
    //                                                     //WETH 18
    //                                                     U256::from_dec_str("1000000000000000000").unwrap(),
    //                                                     //Wrapped BTC
    //                                                     U256::from_dec_str("62022000").unwrap(),
    //                                                     //TBB
    //                                                     U256::from_dec_str("1000000000000000000").unwrap(),
    //                                                     //USDT 18
    //                                                     U256::from_dec_str("1000000000000000000").unwrap(),
    //                                                     //O3 6
    //                                                     U256::from_dec_str("1000000").unwrap(),
    //                                                     //LEND 18
    //                                                     U256::from_dec_str("1000000000000000000").unwrap(),
    //                                                     //LINK 18
    //                                                     U256::from_dec_str("1000000000000000000").unwrap(),
    //                                                     //TEND 18
    //                                                     U256::from_dec_str("1000000000000000000").unwrap(),
    //                                                 ];
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
    pub static ref FRONTRUN_ADDRESS_IN_VEC: Vec<u8> = ::rustc_hex::FromHex::from_hex("1d00652d5E40173ddaCdd24FD8Cdb12228992755").unwrap();
    //Empty fake address
    pub static ref EMPTY_ADDRESS: Address = Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();
}