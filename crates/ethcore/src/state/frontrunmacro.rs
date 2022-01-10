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

//usd-coin 6
(Address::from_str("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(), (U256::from(10100),U256::from_dec_str("1000000").unwrap())),
//multi-collateral-dai 18
(Address::from_str("6b175474e89094c44da98b954eedeac495271d0f").unwrap(), (U256::from(10100),U256::from_dec_str("1000000000000000000").unwrap())),
//weth 18
(Address::from_str("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(), (U256::from(32149100),U256::from_dec_str("1000000000000000000").unwrap())),
//fei-usd 18
(Address::from_str("956F47F50A910163D8BF957Cf5846D573E7f87CA").unwrap(), (U256::from(9978),U256::from_dec_str("1000000000000000000").unwrap())),
//frax-share 18
(Address::from_str("3432b6a60d23ca0dfca7761b7ab56459d9c964d0").unwrap(), (U256::from(382100),U256::from_dec_str("1000000000000000000").unwrap())),
//vader-protocol 18
(Address::from_str("2602278ee1882889b946eb11dc0e810075650983").unwrap(), (U256::from(811),U256::from_dec_str("1000000000000000000").unwrap())),
//tokemak 18
(Address::from_str("2e9d63788249371f1DFC918a52f8d799F4a38C94").unwrap(), (U256::from(586900),U256::from_dec_str("1000000000000000000").unwrap())),
//saitama-inu 9
(Address::from_str("8b3192f5eebd8579568a2ed41e6feb402f93f73f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//strong 18
(Address::from_str("990f341946a3fdb507ae7e52d17851b87168017c").unwrap(), (U256::from(6213000),U256::from_dec_str("1000000000000000000").unwrap())),
//fuse-network 18
(Address::from_str("970b9bb2c0444f5e81e9d0efb84c8ccdcdcaf84d").unwrap(), (U256::from(12800),U256::from_dec_str("1000000000000000000").unwrap())),
//hex 8
(Address::from_str("2b591e99afe9f32eaa6214f7b7629768c40eeb39").unwrap(), (U256::from(2250),U256::from_dec_str("100000000").unwrap())),
//the-sandbox 18
(Address::from_str("3845badAde8e6dFF049820680d1F14bD3903a5d0").unwrap(), (U256::from(49300),U256::from_dec_str("1000000000000000000").unwrap())),
//chainlink 18
(Address::from_str("514910771af9ca656af840dff83e8264ecf986ca").unwrap(), (U256::from(262700),U256::from_dec_str("1000000000000000000").unwrap())),
//synapse-2 18
(Address::from_str("0f2D719407FdBeFF09D87557AbB7232601FD9F29").unwrap(), (U256::from(28800),U256::from_dec_str("1000000000000000000").unwrap())),
//merit-circle 18
(Address::from_str("949d48eca67b17269629c7194f4b727d4ef9e5d6").unwrap(), (U256::from(39700),U256::from_dec_str("1000000000000000000").unwrap())),
//avocado-dao-token 18
(Address::from_str("a41f142b6eb2b164f8164cae0716892ce02f311f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//floki-inu 9
(Address::from_str("43f11c02439e2736800433b4594994Bd43Cd066D").unwrap(), (U256::from(1),U256::from_dec_str("1000000000").unwrap())),
//refi 18
(Address::from_str("A808B22ffd2c472aD1278088F16D4010E6a54D5F").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//wrapped-bitcoin 8
(Address::from_str("2260fac5e5542a773aa44fbcfedf7c193bc2c599").unwrap(), (U256::from(423000000),U256::from_dec_str("100000000").unwrap())),
//dogelon 18
(Address::from_str("761d38e5ddf6ccf6cf7c55759d5210750b5d60f3").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//wootrade 18
(Address::from_str("4691937a7508860f876c9c0a2a617e7d9e945d4b").unwrap(), (U256::from(9618),U256::from_dec_str("1000000000000000000").unwrap())),
//depo 18
(Address::from_str("a5def515cfd373d17830e7c1de1639cb3530a112").unwrap(), (U256::from(1671),U256::from_dec_str("1000000000000000000").unwrap())),
//sushiswap 18
(Address::from_str("6b3595068778dd592e39a122f4f5a5cf09c90fe2").unwrap(), (U256::from(71600),U256::from_dec_str("1000000000000000000").unwrap())),
//star-link 18
(Address::from_str("8e6cd950ad6ba651f6dd608dc70e5886b1aa6b24").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//derace 18
(Address::from_str("9fa69536d1cda4a04cfb50688294de75b505a9ae").unwrap(), (U256::from(24969),U256::from_dec_str("1000000000000000000").unwrap())),
//ufo-gaming 18
(Address::from_str("249e38ea4102d0cf8264d3701f1a0e39c4f2dc3b").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//metisdao 18
(Address::from_str("9E32b13ce7f2E80A01932B42553652E053D6ed8e").unwrap(), (U256::from(1728269),U256::from_dec_str("1000000000000000000").unwrap())),
//ribbon-finance 18
(Address::from_str("6123b0049f904d730db3c36a31167d9d4121fa6b").unwrap(), (U256::from(30000),U256::from_dec_str("1000000000000000000").unwrap())),
//uniswap 18
(Address::from_str("1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap(), (U256::from(159600),U256::from_dec_str("1000000000000000000").unwrap())),
//muse 18
(Address::from_str("b6ca7399b4f9ca56fc27cbff44f4d2e4eef1fc81").unwrap(), (U256::from(462209),U256::from_dec_str("1000000000000000000").unwrap())),
//rai 18
(Address::from_str("03ab458634910aad20ef5f1c8ee96f1d6ac54919").unwrap(), (U256::from(30700),U256::from_dec_str("1000000000000000000").unwrap())),
//yearn-finance 18
(Address::from_str("0bc529c00c6401aef6d220be8c6ea1667f6ad93e").unwrap(), (U256::from(358690000),U256::from_dec_str("1000000000000000000").unwrap())),
//sipher 18
(Address::from_str("9F52c8ecbEe10e00D9faaAc5Ee9Ba0fF6550F511").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//reflexer-ungovernance-token 18
(Address::from_str("6243d8cea23066d098a15582d81a598b4e8391f4").unwrap(), (U256::from(3582939),U256::from_dec_str("1000000000000000000").unwrap())),
//uma 18
(Address::from_str("04Fa0d235C4abf4BcF4787aF4CF447DE572eF828").unwrap(), (U256::from(95600),U256::from_dec_str("1000000000000000000").unwrap())),
//constitutiondao 18
(Address::from_str("7a58c0be72be218b41c608b7fe7c5bb630736c71").unwrap(), (U256::from(1019),U256::from_dec_str("1000000000000000000").unwrap())),
//vlaunch 18
(Address::from_str("51fe2e572e97bfeb1d719809d743ec2675924edc").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//ojamu 18
(Address::from_str("0aa7efe4945db24d95ca6e117bba65ed326e291a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//quant 18
(Address::from_str("4a220e6096b25eadb88358cb44068a3248254675").unwrap(), (U256::from(1803300),U256::from_dec_str("1000000000000000000").unwrap())),
//aragon 18
(Address::from_str("a117000000f279d81a1d3cc75430faa017fa5a2e").unwrap(), (U256::from(93500),U256::from_dec_str("1000000000000000000").unwrap())),
//xmon 18
(Address::from_str("3aaDA3e213aBf8529606924d8D1c55CbDc70Bf74").unwrap(), (U256::from(501920000),U256::from_dec_str("1000000000000000000").unwrap())),
//vesper 18
(Address::from_str("1b40183efb4dd766f11bda7a7c3ad8982e998421").unwrap(), (U256::from(51800),U256::from_dec_str("1000000000000000000").unwrap())),
//highstreet 18
(Address::from_str("71Ab77b7dbB4fa7e017BC15090b2163221420282").unwrap(), (U256::from(93300),U256::from_dec_str("1000000000000000000").unwrap())),
//wilder-world 18
(Address::from_str("2a3bff78b79a009976eea096a51a948a3dc00e34").unwrap(), (U256::from(22982),U256::from_dec_str("1000000000000000000").unwrap())),
//shibnobi 9
(Address::from_str("ab167e816e4d76089119900e941befdfa37d6b32").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//sidus-heroes-sidus-token 18
(Address::from_str("549020a9Cb845220D66d3E9c6D9F9eF61C981102").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//olympus 9
(Address::from_str("64aa3364f17a4d01c6f1751fd97c2bd3d7e7f1d5").unwrap(), (U256::from(2678700),U256::from_dec_str("1000000000").unwrap())),
//brainiac-farm 18
(Address::from_str("39317b8a1ae06c30bb615d88cdc5522781499f1c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//aave 18
(Address::from_str("7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9").unwrap(), (U256::from(2170560),U256::from_dec_str("1000000000000000000").unwrap())),
//impactxp 9
(Address::from_str("b12494c8824fc069757f47d177e666c571cd49ae").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//pax-gold 18
(Address::from_str("45804880de22913dafe09f4980848ece6ecbaf78").unwrap(), (U256::from(18102900),U256::from_dec_str("1000000000000000000").unwrap())),
//lets-go-brandon 18
(Address::from_str("21e783bcf445b515957a10e992ad3c8e9ff51288").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//shiba-inu 18
(Address::from_str("95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//wrapped-luna-token 18
(Address::from_str("d2877702675e6ceb975b4a1dff9fb7baf4c91ea9").unwrap(), (U256::from(698292),U256::from_dec_str("1000000000000000000").unwrap())),
//meta-capital 9
(Address::from_str("bce0665b20164d6cd6d15e70fed1e094ad4a44f0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//unfederalreserve 18
(Address::from_str("5218E472cFCFE0b64A064F055B43b4cdC9EfD3A6").unwrap(), (U256::from(383),U256::from_dec_str("1000000000000000000").unwrap())),
//urus 18
(Address::from_str("6c5fbc90e4d78f70cc5025db005b39b03914fc0c").unwrap(), (U256::from(692800),U256::from_dec_str("1000000000000000000").unwrap())),
//eqifi 18
(Address::from_str("bd3de9a069648c84d27d74d701c9fa3253098b15").unwrap(), (U256::from(3630),U256::from_dec_str("1000000000000000000").unwrap())),
//mute 18
(Address::from_str("a49d7499271ae71cd8ab9ac515e6694c755d400c").unwrap(), (U256::from(16210),U256::from_dec_str("1000000000000000000").unwrap())),
//aggregatedfinance 9
(Address::from_str("0be4447860ddf283884bbaa3702749706750b09e").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//decentraland 18
(Address::from_str("0f5d2fb29fb7d3cfee444a200298f468908cc942").unwrap(), (U256::from(30896),U256::from_dec_str("1000000000000000000").unwrap())),
//shibadoge 9
(Address::from_str("6adb2e268de2aa1abf6578e4a8119b960e02928f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//fantom 18
(Address::from_str("4e15361fd6b4bb609fa63c81a2be19d873717870").unwrap(), (U256::from(26200),U256::from_dec_str("1000000000000000000").unwrap())),
//defi-pulse-index 18
(Address::from_str("1494ca1f11d487c2bbe4543e90080aeba4ba3c2b").unwrap(), (U256::from(2535957),U256::from_dec_str("1000000000000000000").unwrap())),
//mongoose 9
(Address::from_str("a1817B6d8D890F3943b61648992730373B71f156").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//kishu-inu 9
(Address::from_str("A2b4C0Af19cC16a6CfAcCe81F192B024d625817D").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//loopring 18
(Address::from_str("bbbbca6a901c926f240b89eacb641d8aec7aeafd").unwrap(), (U256::from(16900),U256::from_dec_str("1000000000000000000").unwrap())),
//falcon-9 9
(Address::from_str("38a94e92a19e970c144ded0b2dd47278ca11cc1f").unwrap(), (U256::from(147),U256::from_dec_str("1000000000").unwrap())),
//megaweapon 9
(Address::from_str("3063c77c4ef5c1de185321ae2bc5675e17344f7f").unwrap(), (U256::from(25500),U256::from_dec_str("1000000000").unwrap())),
//metafabric 18
(Address::from_str("8c6fa66c21ae3fc435790e451946a9ea82e6e523").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//allianceblock 18
(Address::from_str("00a8b738E453fFd858a7edf03bcCfe20412f0Eb0").unwrap(), (U256::from(5453),U256::from_dec_str("1000000000000000000").unwrap())),
//drops 18
(Address::from_str("6bb61215298f296c55b19ad842d3df69021da2ef").unwrap(), (U256::from(37100),U256::from_dec_str("1000000000000000000").unwrap())),
//rainmaker-games 18
(Address::from_str("71fc1f555a39e0b698653ab0b475488ec3c34d57").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//maker 18
(Address::from_str("9f8f72aa9304c8b593d555f12ef6589cc3a579a2").unwrap(), (U256::from(21323300),U256::from_dec_str("1000000000000000000").unwrap())),
//tenset 18
(Address::from_str("7FF4169a6B5122b664c51c95727d87750eC07c84").unwrap(), (U256::from(26413),U256::from_dec_str("1000000000000000000").unwrap())),
//maple 18
(Address::from_str("33349b282065b0284d756f0577fb39c158f935e6").unwrap(), (U256::from(154800),U256::from_dec_str("1000000000000000000").unwrap())),
//wrapped-ecomi 18
(Address::from_str("04969cd041c0cafb6ac462bd65b536a5bdb3a670").unwrap(), (U256::from(51),U256::from_dec_str("1000000000000000000").unwrap())),
//audius 18
(Address::from_str("18aaa7115705e8be94bffebde57af9bfc265b998").unwrap(), (U256::from(13827),U256::from_dec_str("1000000000000000000").unwrap())),
//gm 9
(Address::from_str("bc7250c8c3eca1dfc1728620af835fca489bfdf3").unwrap(), (U256::from(1),U256::from_dec_str("1000000000").unwrap())),
//kiba-inu 9
(Address::from_str("4b2c54b80b77580dc02a0f6734d3bad733f50900").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//redfox-labs 18
(Address::from_str("a1d6df714f91debf4e0802a542e13067f31b8262").unwrap(), (U256::from(746),U256::from_dec_str("1000000000000000000").unwrap())),
//victoria-vr 18
(Address::from_str("7d5121505149065b562c789a0145ed750e6e8cdd").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//unibright 8
(Address::from_str("8400d94a5cb0fa0d041a3788e395285d61c9ee5e").unwrap(), (U256::from(13799),U256::from_dec_str("100000000").unwrap())),
//keep-network 18
(Address::from_str("85eee30c52b0b379b046fb0f85f4f3dc3009afec").unwrap(), (U256::from(6566),U256::from_dec_str("1000000000000000000").unwrap())),
//polkafoundry 18
(Address::from_str("8b39b70e39aa811b69365398e0aace9bee238aeb").unwrap(), (U256::from(10700),U256::from_dec_str("1000000000000000000").unwrap())),
//eth-2x-flexible-leverage-index 18
(Address::from_str("aa6e8127831c9de45ae56bb1b0d4d4da6e5665bd").unwrap(), (U256::from(1092295),U256::from_dec_str("1000000000000000000").unwrap())),
//radicle 18
(Address::from_str("31c8eacbffdd875c74b94b077895bd78cf1e64a3").unwrap(), (U256::from(90000),U256::from_dec_str("1000000000000000000").unwrap())),
//terrausd 18
(Address::from_str("a47c8bf37f92aBed4A126BDA807A7b7498661acD").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//node-squared 9
(Address::from_str("6110c64219621ce5b02fb8e8e57b54c01b83bf85").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//rubic 18
(Address::from_str("a4eed63db85311e22df4473f87ccfc3dadcfa3e3").unwrap(), (U256::from(2198),U256::from_dec_str("1000000000000000000").unwrap())),
//cerburus 9
(Address::from_str("8a14897ea5f668f36671678593fae44ae23b39fb").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//qanplatform 18
(Address::from_str("aaa7a10a8ee237ea61e8ac46c50a8db8bcc1baaa").unwrap(), (U256::from(1036),U256::from_dec_str("1000000000000000000").unwrap())),
//bully-inu 18
(Address::from_str("55d1d16fb42fce47b899010c996a3a31f6db8fd6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//kylin 18
(Address::from_str("67B6D479c7bB412C54e03dCA8E1Bc6740ce6b99C").unwrap(), (U256::from(1660),U256::from_dec_str("1000000000000000000").unwrap())),
//0chain 10
(Address::from_str("b9ef770b6a5e12e45983c5d80545258aa38f3b78").unwrap(), (U256::from(3768),U256::from_dec_str("10000000000").unwrap())),
//radio-caca 18
(Address::from_str("12BB890508c125661E03b09EC06E404bc9289040").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//polkacity 18
(Address::from_str("aA8330FB2B4D5D07ABFE7A72262752a8505C6B37").unwrap(), (U256::from(5076),U256::from_dec_str("1000000000000000000").unwrap())),
//radix 18
(Address::from_str("6468e79A80C0eaB0F9A2B574c8d5bC374Af59414").unwrap(), (U256::from(2119),U256::from_dec_str("1000000000000000000").unwrap())),
//all-coins-yield-capital 18
(Address::from_str("b56a1f3310578f23120182fb2e58c087efe6e147").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//clifford-inu 18
(Address::from_str("1b9baf2a3edea91ee431f02d449a1044d5726669").unwrap(), (U256::from(1),U256::from_dec_str("1000000000000000000").unwrap())),
//mirrored-google 18
(Address::from_str("59A921Db27Dd6d4d974745B7FfC5c33932653442").unwrap(), (U256::from(28267308),U256::from_dec_str("1000000000000000000").unwrap())),
//stobox-token 18
(Address::from_str("212DD60D4Bf0DA8372fe8116474602d429E5735F").unwrap(), (U256::from(14),U256::from_dec_str("1000000000000000000").unwrap())),
//polkabridge 18
(Address::from_str("298d492e8c1d909d3f63bc4a36c66c64acb3d695").unwrap(), (U256::from(6740),U256::from_dec_str("1000000000000000000").unwrap())),
//dao-maker 18
(Address::from_str("0f51bb10119727a7e5ea3538074fb341f56b09ad").unwrap(), (U256::from(45006),U256::from_dec_str("1000000000000000000").unwrap())),
//wise 18
(Address::from_str("66a0f676479cee1d7373f3dc2e2952778bff5bd6").unwrap(), (U256::from(3654),U256::from_dec_str("1000000000000000000").unwrap())),
//perpetual-protocol 18
(Address::from_str("bc396689893d065f41bc2c6ecbee5e0085233447").unwrap(), (U256::from(87373),U256::from_dec_str("1000000000000000000").unwrap())),
//celsius 4
(Address::from_str("aaaebe6fe48e54f431b0c390cfaf0b017d09d42d").unwrap(), (U256::from(33400),U256::from_dec_str("10000").unwrap())),
//tokenlon-network-token 18
(Address::from_str("0000000000095413afc295d19edeb1ad7b71c952").unwrap(), (U256::from(12800),U256::from_dec_str("1000000000000000000").unwrap())),
//polygon 18
(Address::from_str("7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0").unwrap(), (U256::from(20719),U256::from_dec_str("1000000000000000000").unwrap())),
//k21 18
(Address::from_str("b9d99c33ea2d86ec5ec6b8a4dd816ebba64404af").unwrap(), (U256::from(11437),U256::from_dec_str("1000000000000000000").unwrap())),
//civilization 18
(Address::from_str("37fe0f067fa808ffbdd12891c0858532cfe7361d").unwrap(), (U256::from(1561),U256::from_dec_str("1000000000000000000").unwrap())),
//opulous 18
(Address::from_str("80d55c03180349fff4a229102f62328220a96444").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//kleros 18
(Address::from_str("93ed3fbe21207ec2e8f2d3c3de6e058cb73bc04d").unwrap(), (U256::from(1204),U256::from_dec_str("1000000000000000000").unwrap())),
//ethernity-chain 18
(Address::from_str("bbc2ae13b23d715c30720f079fcd9b4a74093505").unwrap(), (U256::from(87239),U256::from_dec_str("1000000000000000000").unwrap())),
//akita-inu 18
(Address::from_str("3301ee63fb29f863f2333bd4466acb46cd8323e6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//volt-inu 9
(Address::from_str("3f7aff0ef20aa2e646290dfa4e67611b2220c597").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//minds 18
(Address::from_str("b26631c6dda06ad89b93c71400d25692de89c068").unwrap(), (U256::from(22300),U256::from_dec_str("1000000000000000000").unwrap())),
//milc-platform 18
(Address::from_str("9506d37f70eB4C3d79C398d326C871aBBf10521d").unwrap(), (U256::from(3135),U256::from_dec_str("1000000000000000000").unwrap())),
//alpha-brain-capital 18
(Address::from_str("5b4e9a810321e168989802474f689269ec442681").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//ash 18
(Address::from_str("64d91f12ece7362f91a6f8e7940cd55f05060b92").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//glitch 18
(Address::from_str("038a68ff68c393373ec894015816e33ad41bd564").unwrap(), (U256::from(5566),U256::from_dec_str("1000000000000000000").unwrap())),
//cellframe 18
(Address::from_str("26c8afbbfe1ebaca03c2bb082e69d0476bffe099").unwrap(), (U256::from(9281),U256::from_dec_str("1000000000000000000").unwrap())),
//boost-coin 18
(Address::from_str("4e0fca55a6c3a94720ded91153a27f60e26b9aa8").unwrap(), (U256::from(89),U256::from_dec_str("1000000000000000000").unwrap())),
//sidus-heroes 18
(Address::from_str("34Be5b8C30eE4fDe069DC878989686aBE9884470").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//mirrored-microsoft 18
(Address::from_str("41BbEDd7286dAab5910a1f15d12CBda839852BD7").unwrap(), (U256::from(3141157),U256::from_dec_str("1000000000000000000").unwrap())),
//botto 18
(Address::from_str("9dfad1b7102d46b1b197b90095b5c4e9f5845bba").unwrap(), (U256::from(5321),U256::from_dec_str("1000000000000000000").unwrap())),
//revest-finance 18
(Address::from_str("120a3879da835a5af037bb2d1456bebd6b54d4ba").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//mirrored-ishares-silver-trust 18
(Address::from_str("9d1555d8cB3C846Bb4f7D5B1B1080872c3166676").unwrap(), (U256::from(215921),U256::from_dec_str("1000000000000000000").unwrap())),
//lukso 18
(Address::from_str("A8b919680258d369114910511cc87595aec0be6D").unwrap(), (U256::from(154328),U256::from_dec_str("1000000000000000000").unwrap())),
//waxe 8
(Address::from_str("7a2bc711e19ba6aff6ce8246c546e8c4b4944dfd").unwrap(), (U256::from(4123800),U256::from_dec_str("100000000").unwrap())),
//paid-network 18
(Address::from_str("1614F18Fc94f47967A3Fbe5FfcD46d4e7Da3D787").unwrap(), (U256::from(5648),U256::from_dec_str("1000000000000000000").unwrap())),
//blockchainspace 18
(Address::from_str("83e9f223e1edb3486f876ee888d76bfba26c475a").unwrap(), (U256::from(3855),U256::from_dec_str("1000000000000000000").unwrap())),
//skale-network 18
(Address::from_str("00c83aecc790e8a4453e5dd3b0b4b3680501a7a7").unwrap(), (U256::from(1757),U256::from_dec_str("1000000000000000000").unwrap())),
//shiryo-inu 9
(Address::from_str("1e2f15302b90edde696593607b6bd444b64e8f02").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//doge-killer 18
(Address::from_str("27c70cd1946795b66be9d954418546998b546634").unwrap(), (U256::from(11265929),U256::from_dec_str("1000000000000000000").unwrap())),
//bondly 18
(Address::from_str("91dfbee3965baaee32784c2d546b7a0c62f268c9").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//mirror-protocol 18
(Address::from_str("09a3EcAFa817268f77BE1283176B946C4ff2E608").unwrap(), (U256::from(19394),U256::from_dec_str("1000000000000000000").unwrap())),
//groupdao 18
(Address::from_str("16f78145ad0b9af58747e9a97ebd99175378bd3d").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//secret-erc20 6
(Address::from_str("2b89bf8ba858cd2fcee1fada378d5cd6936968be").unwrap(), (U256::from(67300),U256::from_dec_str("1000000").unwrap())),
//mirrored-alibaba 18
(Address::from_str("56aA298a19C93c6801FDde870fA63EF75Cc0aF72").unwrap(), (U256::from(1379425),U256::from_dec_str("1000000000000000000").unwrap())),
//peth18c 18
(Address::from_str("A15690E9205De386Ce849889831C1668c300C1ad").unwrap(), (U256::from(112205),U256::from_dec_str("1000000000000000000").unwrap())),
//ftx-token 18
(Address::from_str("50d1c9771902476076ecfc8b2a83ad6b9355a4c9").unwrap(), (U256::from(359799),U256::from_dec_str("1000000000000000000").unwrap())),
//blank-wallet 18
(Address::from_str("41a3dba3d677e573636ba691a70ff2d606c29666").unwrap(), (U256::from(5493),U256::from_dec_str("1000000000000000000").unwrap())),
//keep3rv1 18
(Address::from_str("1ceb5cb57c4d4e2b2433641b95dd330a33185a44").unwrap(), (U256::from(12142000),U256::from_dec_str("1000000000000000000").unwrap())),
//singularitydao 18
(Address::from_str("993864e43caa7f7f12953ad6feb1d1ca635b875f").unwrap(), (U256::from(12831),U256::from_dec_str("1000000000000000000").unwrap())),
//gas-dao 18
(Address::from_str("6bba316c48b49bd1eac44573c5c871ff02958469").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//bundles 18
(Address::from_str("8D3E855f3f55109D473735aB76F753218400fe96").unwrap(), (U256::from(573300),U256::from_dec_str("1000000000000000000").unwrap())),
//katana-inu 18
(Address::from_str("2e85ae1C47602f7927bCabc2Ff99C40aA222aE15").unwrap(), (U256::from(37),U256::from_dec_str("1000000000000000000").unwrap())),
//metavice-token 18
(Address::from_str("5375fd52707ab7c8d1b088e07169fa74b0999732").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//kitty-inu 9
(Address::from_str("044727e50ff30db57fad06ff4f5846eab5ea52a2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//enjinstarter 18
(Address::from_str("96610186F3ab8d73EBEe1CF950C750f3B1Fb79C2").unwrap(), (U256::from(503),U256::from_dec_str("1000000000000000000").unwrap())),
//umbrella-network 18
(Address::from_str("6fc13eace26590b80cccab1ba5d51890577d83b2").unwrap(), (U256::from(2695),U256::from_dec_str("1000000000000000000").unwrap())),
//lattice-token 8
(Address::from_str("a393473d64d2F9F026B60b6Df7859A689715d092").unwrap(), (U256::from(8270),U256::from_dec_str("100000000").unwrap())),
//request 18
(Address::from_str("8f8221afbb33998d8584a2b05749ba73c37a938a").unwrap(), (U256::from(3057),U256::from_dec_str("1000000000000000000").unwrap())),
//zeroswap 18
(Address::from_str("2eDf094dB69d6Dcd487f1B3dB9febE2eeC0dd4c5").unwrap(), (U256::from(1787),U256::from_dec_str("1000000000000000000").unwrap())),
//life-crypto 18
(Address::from_str("6c936D4AE98E6d2172dB18c16C4b601C99918EE6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//ocean-protocol 18
(Address::from_str("967da4048cd07ab37855c090aaf366e4ce1b9f48").unwrap(), (U256::from(7635),U256::from_dec_str("1000000000000000000").unwrap())),
//cryptocart-v2 18
(Address::from_str("612e1726435fe38dd49a0b35b4065b56f49c8f11").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//dodo 18
(Address::from_str("43dfc4159d86f3a37a5a4b3d4580b888ad7d4ddd").unwrap(), (U256::from(8097),U256::from_dec_str("1000000000000000000").unwrap())),
//babydoge-coin 9
(Address::from_str("AC8E13ecC30Da7Ff04b842f21A62a1fb0f10eBd5").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//digifit 9
(Address::from_str("a420dd089a33d3751e8750f0b3554c72761dc83e").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//jacywaya 9
(Address::from_str("08f2991a6eff2671cf791b82aeae64fbbfdd0633").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//stratos 18
(Address::from_str("08c32b0726C5684024ea6e141C50aDe9690bBdcc").unwrap(), (U256::from(20000),U256::from_dec_str("1000000000000000000").unwrap())),
//gysr 18
(Address::from_str("bea98c05eeae2f3bc8c3565db7551eb738c8ccab").unwrap(), (U256::from(2723),U256::from_dec_str("1000000000000000000").unwrap())),
//metaverse-index 18
(Address::from_str("72e364f2abdc788b7e918bc238b21f109cd634d7").unwrap(), (U256::from(1886533),U256::from_dec_str("1000000000000000000").unwrap())),
//kleekai 9
(Address::from_str("382f0160c24f5c515a19f155bac14d479433a407").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//stake-dao 18
(Address::from_str("73968b9a57c6e53d41345fd57a6e6ae27d6cdb2f").unwrap(), (U256::from(18800),U256::from_dec_str("1000000000000000000").unwrap())),
//storj 8
(Address::from_str("b64ef51c888972c908cfacf59b47c1afbc0ab8ac").unwrap(), (U256::from(15800),U256::from_dec_str("100000000").unwrap())),
//public-mint 18
(Address::from_str("0CDF9acd87E940837ff21BB40c9fd55F68bba059").unwrap(), (U256::from(1512),U256::from_dec_str("1000000000000000000").unwrap())),
//baby-doge-coin 9
(Address::from_str("AC57De9C1A09FeC648E93EB98875B212DB0d460B").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//inuyasha 18
(Address::from_str("5bddbfdc228e1bbdb9ef5ca1dc56b54c4d6d6621").unwrap(), (U256::from(2),U256::from_dec_str("1000000000000000000").unwrap())),
//holo 18
(Address::from_str("6c6ee5e31d828de241282b9606c8e98ea48526e2").unwrap(), (U256::from(64),U256::from_dec_str("1000000000000000000").unwrap())),
//syntropy 18
(Address::from_str("a8c8CfB141A3bB59FEA1E2ea6B79b5ECBCD7b6ca").unwrap(), (U256::from(2624),U256::from_dec_str("1000000000000000000").unwrap())),
//verox 18
(Address::from_str("87DE305311D5788e8da38D19bb427645b09CB4e5").unwrap(), (U256::from(1625677),U256::from_dec_str("1000000000000000000").unwrap())),
//decentr 18
(Address::from_str("30f271c9e86d2b7d00a6376cd96a1cfbd5f0b9b3").unwrap(), (U256::from(614),U256::from_dec_str("1000000000000000000").unwrap())),
//oraichain-token 18
(Address::from_str("4c11249814f11b9346808179cf06e71ac328c1b5").unwrap(), (U256::from(71500),U256::from_dec_str("1000000000000000000").unwrap())),
//harvest-finance 18
(Address::from_str("a0246c9032bC3A600820415aE600c6388619A14D").unwrap(), (U256::from(1544733),U256::from_dec_str("1000000000000000000").unwrap())),
//kirobo 18
(Address::from_str("b1191f691a355b43542bea9b8847bc73e7abb137").unwrap(), (U256::from(3894),U256::from_dec_str("1000000000000000000").unwrap())),
//mask-network 18
(Address::from_str("69af81e73a73b40adf4f3d4223cd9b1ece623074").unwrap(), (U256::from(102100),U256::from_dec_str("1000000000000000000").unwrap())),
//thorchain-erc20 18
(Address::from_str("3155ba85d5f96b2d030a4966af206230e46849cb").unwrap(), (U256::from(66399),U256::from_dec_str("1000000000000000000").unwrap())),
//flux-dao 18
(Address::from_str("3ea8ea4237344c9931214796d9417af1a1180770").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//floki-musk 18
(Address::from_str("67cc621ab2d086a101cff3340df0a065ac75827c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//saffron-finance 18
(Address::from_str("b753428af26e81097e7fd17f40c88aaa3e04902c").unwrap(), (U256::from(2568900),U256::from_dec_str("1000000000000000000").unwrap())),
//mahadao 18
(Address::from_str("b4d930279552397bba2ee473229f89ec245bc365").unwrap(), (U256::from(56000),U256::from_dec_str("1000000000000000000").unwrap())),
//xy-finance 18
(Address::from_str("77777777772cf0455fB38eE0e75f38034dFa50DE").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//signata 18
(Address::from_str("3ebb4A4e91Ad83BE51F8d596533818b246F4bEe1").unwrap(), (U256::from(5666),U256::from_dec_str("1000000000000000000").unwrap())),
//fantasy-world-gold 9
(Address::from_str("7345Ffe6291bc15381A4110831013e8fe9f93253").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//deficliq 18
(Address::from_str("0Def8d8addE14c9eF7c2a986dF3eA4Bd65826767").unwrap(), (U256::from(225),U256::from_dec_str("1000000000000000000").unwrap())),
//mirrored-amazon 18
(Address::from_str("0cae9e4d663793c2a2A0b211c1Cf4bBca2B9cAa7").unwrap(), (U256::from(33093092),U256::from_dec_str("1000000000000000000").unwrap())),
//niifi 15
(Address::from_str("852e5427c86a3b46dd25e5fe027bb15f53c4bcb8").unwrap(), (U256::from(350),U256::from_dec_str("1000000000000000").unwrap())),
//polkafantasy 18
(Address::from_str("948c70Dc6169Bfb10028FdBE96cbC72E9562b2Ac").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//kuma-inu 18
(Address::from_str("48c276e8d03813224bb1e55f953adb6d02fd3e02").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//parsiq 18
(Address::from_str("362bc847A3a9637d3af6624EeC853618a43ed7D2").unwrap(), (U256::from(3912),U256::from_dec_str("1000000000000000000").unwrap())),
//reserve-rights 18
(Address::from_str("8762db106b2c2a0bccb3a80d1ed41273552616e8").unwrap(), (U256::from(257),U256::from_dec_str("1000000000000000000").unwrap())),
//bidao 18
(Address::from_str("25e1474170c4c0aA64fa98123bdc8dB49D7802fa").unwrap(), (U256::from(121),U256::from_dec_str("1000000000000000000").unwrap())),
//naos-finance 18
(Address::from_str("4a615bb7166210cce20e6642a6f8fb5d4d044496").unwrap(), (U256::from(3558),U256::from_dec_str("1000000000000000000").unwrap())),
//cocktail-bar 8
(Address::from_str("22b6c31c2beb8f2d0d5373146eed41ab9ede3caf").unwrap(), (U256::from(2218438),U256::from_dec_str("100000000").unwrap())),
//render-token 18
(Address::from_str("6de037ef9ad2725eb40118bb1702ebb27e4aeb24").unwrap(), (U256::from(37300),U256::from_dec_str("1000000000000000000").unwrap())),
//fnk-wallet 18
(Address::from_str("b5fe099475d3030dde498c3bb6f3854f762a48ad").unwrap(), (U256::from(301),U256::from_dec_str("1000000000000000000").unwrap())),
//b-protocol 18
(Address::from_str("bbbbbbb5aa847a2003fbc6b5c16df0bd1e725f61").unwrap(), (U256::from(70500),U256::from_dec_str("1000000000000000000").unwrap())),
//whale 4
(Address::from_str("9355372396e3F6daF13359B7b607a3374cc638e0").unwrap(), (U256::from(119000),U256::from_dec_str("10000").unwrap())),
//bao-finance 18
(Address::from_str("374cb8c27130e2c9e04f44303f3c8351b9de61c1").unwrap(), (U256::from(2),U256::from_dec_str("1000000000000000000").unwrap())),
//pickle-finance 18
(Address::from_str("429881672B9AE42b8EbA0E26cD9C73711b891Ca5").unwrap(), (U256::from(87219),U256::from_dec_str("1000000000000000000").unwrap())),
//algovest 18
(Address::from_str("94d916873b22c9c1b53695f1c002f78537b9b3b2").unwrap(), (U256::from(15612),U256::from_dec_str("1000000000000000000").unwrap())),
//multi-farm-capital 9
(Address::from_str("b77b6fe3e33ce2a15bae846658fca5da62ab8ac0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//dopex-rdpx 18
(Address::from_str("0ff5a8451a839f5f0bb3562689d9a44089738d11").unwrap(), (U256::from(1241000),U256::from_dec_str("1000000000000000000").unwrap())),
//chain-guardians 18
(Address::from_str("1fe24f25b1cf609b9c4e7e12d802e3640dfa5e43").unwrap(), (U256::from(6887),U256::from_dec_str("1000000000000000000").unwrap())),
//geeq 18
(Address::from_str("6b9f031d718dded0d681c20cb754f97b3bb81b78").unwrap(), (U256::from(10310),U256::from_dec_str("1000000000000000000").unwrap())),
//insurace 18
(Address::from_str("544c42fbb96b39b21df61cf322b5edc285ee7429").unwrap(), (U256::from(10400),U256::from_dec_str("1000000000000000000").unwrap())),
//mirrored-tesla 18
(Address::from_str("21cA39943E91d704678F5D00b6616650F066fD63").unwrap(), (U256::from(10641913),U256::from_dec_str("1000000000000000000").unwrap())),
//golem-network-tokens 18
(Address::from_str("7DD9c5Cba05E151C895FDe1CF355C9A1D5DA6429").unwrap(), (U256::from(4205),U256::from_dec_str("1000000000000000000").unwrap())),
//gemini-dollar 2
(Address::from_str("056Fd409E1d7A124BD7017459dFEa2F387b6d5Cd").unwrap(), (U256::from(10100),U256::from_dec_str("100").unwrap())),
//synthetix-network-token 18
(Address::from_str("c011a73ee8576fb46f5e1c5751ca3b9fe0af2a6f").unwrap(), (U256::from(55199),U256::from_dec_str("1000000000000000000").unwrap())),
//cvault-finance 18
(Address::from_str("62359ed7505efc61ff1d56fef82158ccaffa23d7").unwrap(), (U256::from(68849096),U256::from_dec_str("1000000000000000000").unwrap())),
//wagyuswap 18
(Address::from_str("7FA7dF4996AC59F398476892cfB195eD38543520").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//gencoin-capital 9
(Address::from_str("0b569fa433faa7f01f3ea880193de38044b41de0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//capital-aggregator-token 9
(Address::from_str("3734dc0d241b5ad886fa6bff45ffa67252ac0e89").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//trade-race-manager 6
(Address::from_str("8b3870df408ff4d7c3a26df852d41034eda11d81").unwrap(), (U256::from(17100),U256::from_dec_str("1000000").unwrap())),
//sora 18
(Address::from_str("40FD72257597aA14C7231A7B1aaa29Fce868F677").unwrap(), (U256::from(987211),U256::from_dec_str("1000000000000000000").unwrap())),
//pbtc35a 18
(Address::from_str("A8b12Cc90AbF65191532a12bb5394A714A46d358").unwrap(), (U256::from(885513),U256::from_dec_str("1000000000000000000").unwrap())),
//sarcophagus 18
(Address::from_str("7697b462a7c4ff5f8b55bdbc2f4076c2af9cf51a").unwrap(), (U256::from(9593),U256::from_dec_str("1000000000000000000").unwrap())),
//coldstack 18
(Address::from_str("675bbc7514013e2073db7a919f6e4cbef576de37").unwrap(), (U256::from(15580),U256::from_dec_str("1000000000000000000").unwrap())),
//infinitygaming 9
(Address::from_str("95b4e47025372ded4b73f9b5f0671b94a81445bc").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//chrono-tech 8
(Address::from_str("485d17A6f1B8780392d53D64751824253011A260").unwrap(), (U256::from(2416923),U256::from_dec_str("100000000").unwrap())),
//stackos 18
(Address::from_str("56a86d648c435dc707c8405b78e2ae8eb4e60ba4").unwrap(), (U256::from(762),U256::from_dec_str("1000000000000000000").unwrap())),
//origintrail 18
(Address::from_str("aa7a9ca87d3694b5755f213b5d04094b8d0f0a6f").unwrap(), (U256::from(10200),U256::from_dec_str("1000000000000000000").unwrap())),
//nix-bridge-token 18
(Address::from_str("2e2364966267B5D7D2cE6CD9A9B5bD19d9C7C6A9").unwrap(), (U256::from(3182700),U256::from_dec_str("1000000000000000000").unwrap())),
//rangers-protocol 18
(Address::from_str("0E5C8C387C5EBa2eCbc137aD012aeD5Fe729e251").unwrap(), (U256::from(164981),U256::from_dec_str("1000000000000000000").unwrap())),
//bonded-finance 8
(Address::from_str("5dc02ea99285e17656b8350722694c35154db1e8").unwrap(), (U256::from(191),U256::from_dec_str("100000000").unwrap())),
//naga 18
(Address::from_str("72dd4b6bd852a3aa172be4d6c5a6dbec588cf131").unwrap(), (U256::from(9289),U256::from_dec_str("1000000000000000000").unwrap())),
//exotix 9
(Address::from_str("230bf0637628ef356b63d389e2ec6c77c8853a11").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//antimatter 18
(Address::from_str("9B99CcA871Be05119B2012fd4474731dd653FEBe").unwrap(), (U256::from(5050),U256::from_dec_str("1000000000000000000").unwrap())),
//ryoshis-vision 18
(Address::from_str("777E2ae845272a2F540ebf6a3D03734A5a8f618e").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//zigcoin 18
(Address::from_str("b2617246d0c6c0087f18703d576831899ca94f01").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//singularitynet 8
(Address::from_str("5B7533812759B45C2B44C19e320ba2cD2681b542").unwrap(), (U256::from(1748),U256::from_dec_str("100000000").unwrap())),
//pop 18
(Address::from_str("7fC3eC3574d408F3b59CD88709baCb42575EBF2b").unwrap(), (U256::from(1019),U256::from_dec_str("1000000000000000000").unwrap())),
//uncx 18
(Address::from_str("aDB2437e6F65682B85F814fBc12FeC0508A7B1D0").unwrap(), (U256::from(5192700),U256::from_dec_str("1000000000000000000").unwrap())),
//lossless 18
(Address::from_str("3b9be07d622accaed78f479bc0edabfd6397e320").unwrap(), (U256::from(7927),U256::from_dec_str("1000000000000000000").unwrap())),
//madworld 8
(Address::from_str("31c2415c946928e9FD1Af83cdFA38d3eDBD4326f").unwrap(), (U256::from(1396),U256::from_dec_str("100000000").unwrap())),
//xfai 18
(Address::from_str("4aa41bC1649C9C3177eD16CaaA11482295fC7441").unwrap(), (U256::from(687),U256::from_dec_str("1000000000000000000").unwrap())),
//pooltogether 18
(Address::from_str("0cec1a9154ff802e7934fc916ed7ca50bde6844e").unwrap(), (U256::from(43500),U256::from_dec_str("1000000000000000000").unwrap())),
//pawtocol 18
(Address::from_str("70d2b7c19352bb76e4409858ff5746e500f2b67c").unwrap(), (U256::from(610),U256::from_dec_str("1000000000000000000").unwrap())),
//api3 18
(Address::from_str("0b38210ea11411557c13457D4dA7dC6ea731B88a").unwrap(), (U256::from(38000),U256::from_dec_str("1000000000000000000").unwrap())),
//bone 18
(Address::from_str("5C84bc60a796534bfeC3439Af0E6dB616A966335").unwrap(), (U256::from(131),U256::from_dec_str("1000000000000000000").unwrap())),
//mirrored-united-states-oil-fund 18
(Address::from_str("31c63146a635EB7465e5853020b39713AC356991").unwrap(), (U256::from(583736),U256::from_dec_str("1000000000000000000").unwrap())),
//apy-finance 18
(Address::from_str("95a4492F028aa1fd432Ea71146b433E7B4446611").unwrap(), (U256::from(2264),U256::from_dec_str("1000000000000000000").unwrap())),
//digitalbits 7
(Address::from_str("b9eefc4b0d472a44be93970254df4f4016569d27").unwrap(), (U256::from(3972),U256::from_dec_str("10000000").unwrap())),
//yop 8
(Address::from_str("ae1eaae3f627aaca434127644371b67b18444051").unwrap(), (U256::from(2695),U256::from_dec_str("100000000").unwrap())),
//numbers-protocol 18
(Address::from_str("3496b523e5c00a4b4150d6721320cddb234c3079").unwrap(), (U256::from(8767),U256::from_dec_str("1000000000000000000").unwrap())),
//mononoke-inu 9
(Address::from_str("4da08a1bff50be96bded5c7019227164b49c2bfc").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//occamfi 18
(Address::from_str("2f109021afe75b949429fe30523ee7c0d5b27207").unwrap(), (U256::from(23000),U256::from_dec_str("1000000000000000000").unwrap())),
//sienna-erc20 18
(Address::from_str("9b00e6E8D787b13756eb919786c9745054DB64f9").unwrap(), (U256::from(111115),U256::from_dec_str("1000000000000000000").unwrap())),
//bridge-mutual 18
(Address::from_str("725c263e32c72ddc3a19bea12c5a0479a81ee688").unwrap(), (U256::from(2176),U256::from_dec_str("1000000000000000000").unwrap())),
//sentivate 18
(Address::from_str("7865af71cf0b288b4e7f654f4f7851eb46a2b7f8").unwrap(), (U256::from(151),U256::from_dec_str("1000000000000000000").unwrap())),
//rentible 18
(Address::from_str("2a039b1d9bbdccbb91be28691b730ca893e5e743").unwrap(), (U256::from(10971),U256::from_dec_str("1000000000000000000").unwrap())),
//tokenplace 8
(Address::from_str("4fb721ef3bf99e0f2c193847afa296b9257d3c30").unwrap(), (U256::from(0),U256::from_dec_str("100000000").unwrap())),
//paribus 18
(Address::from_str("d528cf2e081f72908e086f8800977df826b5a483").unwrap(), (U256::from(191),U256::from_dec_str("1000000000000000000").unwrap())),
//captain-inu 18
(Address::from_str("7cca2e1c9b0519f52029467914a15e782bf66971").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//ridotto 18
(Address::from_str("4740735aa98dc8aa232bd049f8f0210458e7fca3").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//everrise 18
(Address::from_str("0cd022dde27169b20895e0e2b2b8a33b25e63579").unwrap(), (U256::from(7),U256::from_dec_str("1000000000000000000").unwrap())),
//b-cube-ai 18
(Address::from_str("93C9175E26F57d2888c7Df8B470C9eeA5C0b0A93").unwrap(), (U256::from(1874),U256::from_dec_str("1000000000000000000").unwrap())),
//alkimi 18
(Address::from_str("3106a0a076BeDAE847652F42ef07FD58589E001f").unwrap(), (U256::from(3183),U256::from_dec_str("1000000000000000000").unwrap())),
//orion-protocol 8
(Address::from_str("0258F474786DdFd37ABCE6df6BBb1Dd5dfC4434a").unwrap(), (U256::from(50626),U256::from_dec_str("100000000").unwrap())),
//pnetwork 18
(Address::from_str("89ab32156e46f46d02ade3fecbe5fc4243b9aaed").unwrap(), (U256::from(8474),U256::from_dec_str("1000000000000000000").unwrap())),
//infinity-token 9
(Address::from_str("7fe4fbad1fee10d6cf8e08198608209a9275944c").unwrap(), (U256::from(10),U256::from_dec_str("1000000000").unwrap())),
//covercompared 18
(Address::from_str("3c03b4ec9477809072ff9cc9292c9b25d4a8e6c6").unwrap(), (U256::from(863),U256::from_dec_str("1000000000000000000").unwrap())),
//lobby 9
(Address::from_str("ac042d9284df95cc6bd35982f6a61e3e7a6f875b").unwrap(), (U256::from(26),U256::from_dec_str("1000000000").unwrap())),
//kattana 18
(Address::from_str("491e136ff7ff03e6ab097e54734697bb5802fc1c").unwrap(), (U256::from(60092),U256::from_dec_str("1000000000000000000").unwrap())),
//meter-governance-mapped-by-meter-io 18
(Address::from_str("Bd2949F67DcdC549c6Ebe98696449Fa79D988A9F").unwrap(), (U256::from(48013),U256::from_dec_str("1000000000000000000").unwrap())),
//everipedia 18
(Address::from_str("579cea1889991f68acc35ff5c3dd0621ff29b0c9").unwrap(), (U256::from(114),U256::from_dec_str("1000000000000000000").unwrap())),
//akropolis 18
(Address::from_str("8ab7404063ec4dbcfd4598215992dc3f8ec853d7").unwrap(), (U256::from(202),U256::from_dec_str("1000000000000000000").unwrap())),
//monstaverse 9
(Address::from_str("ba75fbc4c7a553081f7a137b6e652520db444660").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//mirrored-invesco-qqq-trust 18
(Address::from_str("13B02c8dE71680e71F0820c996E4bE43c2F57d15").unwrap(), (U256::from(3835037),U256::from_dec_str("1000000000000000000").unwrap())),
//gold-fever 18
(Address::from_str("2653891204f463fb2a2f4f412564b19e955166ae").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//ramp 18
(Address::from_str("33D0568941C0C64ff7e0FB4fbA0B11BD37deEd9f").unwrap(), (U256::from(1641),U256::from_dec_str("1000000000000000000").unwrap())),
//alpha 18
(Address::from_str("138c2f1123cf3f82e4596d097c118eac6684940b").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//guzzler 18
(Address::from_str("9f4909cc95fb870bf48c128c1fdbb5f482797632").unwrap(), (U256::from(1),U256::from_dec_str("1000000000000000000").unwrap())),
//bloxmove 18
(Address::from_str("38d9eb07a7b8df7d86f440a4a5c4a4c1a27e1a08").unwrap(), (U256::from(28600),U256::from_dec_str("1000000000000000000").unwrap())),
//meta 18
(Address::from_str("a3BeD4E1c75D00fa6f4E5E6922DB7261B5E9AcD2").unwrap(), (U256::from(6772),U256::from_dec_str("1000000000000000000").unwrap())),
//vidya 18
(Address::from_str("3d3d35bb9bec23b06ca00fe472b50e7a4c692c30").unwrap(), (U256::from(2519),U256::from_dec_str("1000000000000000000").unwrap())),
//card-starter 18
(Address::from_str("3d6f0dea3ac3c607b3998e6ce14b6350721752d9").unwrap(), (U256::from(46800),U256::from_dec_str("1000000000000000000").unwrap())),
//swingby 18
(Address::from_str("8287c7b963b405b7b8d467db9d79eec40625b13a").unwrap(), (U256::from(269),U256::from_dec_str("1000000000000000000").unwrap())),
//marlin 18
(Address::from_str("57b946008913b82e4df85f501cbaed910e58d26c").unwrap(), (U256::from(545),U256::from_dec_str("1000000000000000000").unwrap())),
//don-key 18
(Address::from_str("217ddead61a42369a266f1fb754eb5d3ebadc88a").unwrap(), (U256::from(3030),U256::from_dec_str("1000000000000000000").unwrap())),
//swapdex 7
(Address::from_str("041fdd6637ecfd96af8804278ac12660ac2d12c0").unwrap(), (U256::from(453),U256::from_dec_str("10000000").unwrap())),
//kori-inu 9
(Address::from_str("345dadb10a200f10814ad8523fca0f2d958c3370").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//futureswap 18
(Address::from_str("0e192d382a36de7011f795acc4391cd302003606").unwrap(), (U256::from(36097),U256::from_dec_str("1000000000000000000").unwrap())),
//union-protocol-governance-token 18
(Address::from_str("226f7b842e0f0120b7e194d05432b3fd14773a9d").unwrap(), (U256::from(39),U256::from_dec_str("1000000000000000000").unwrap())),
//bistroo 18
(Address::from_str("6e8908cfa881c9f6f2c64d3436e7b80b1bf0093f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//doont-buy 9
(Address::from_str("4ece5c5cfb9b960a49aae739e15cdb6cfdcc5782").unwrap(), (U256::from(140),U256::from_dec_str("1000000000").unwrap())),
//sphynx-eth 18
(Address::from_str("94dfd4e2210fa5b752c3cd0f381edad9da6640f8").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//domi-online 18
(Address::from_str("45C2F8c9B4c0bDC76200448cc26C48ab6ffef83F").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//deepspace-token 18
(Address::from_str("528b3e98c63ce21c6f680b713918e0f89dfae555").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//ethpad 18
(Address::from_str("8dB1D28Ee0d822367aF8d220C0dc7cB6fe9DC442").unwrap(), (U256::from(746),U256::from_dec_str("1000000000000000000").unwrap())),
//finxflo 18
(Address::from_str("8a40c222996f9f3431f63bf80244c36822060f12").unwrap(), (U256::from(1524),U256::from_dec_str("1000000000000000000").unwrap())),
//monavale 18
(Address::from_str("275f5Ad03be0Fa221B4C6649B8AeE09a42D9412A").unwrap(), (U256::from(5211933),U256::from_dec_str("1000000000000000000").unwrap())),
//c20 18
(Address::from_str("26e75307fc0c021472feb8f727839531f112f317").unwrap(), (U256::from(39200),U256::from_dec_str("1000000000000000000").unwrap())),
//aleph-im 18
(Address::from_str("27702a26126e0b3702af63ee09ac4d1a084ef628").unwrap(), (U256::from(5086),U256::from_dec_str("1000000000000000000").unwrap())),
//oiler-network 18
(Address::from_str("0275e1001e293c46cfe158b3702aade0b99f88a5").unwrap(), (U256::from(5123),U256::from_dec_str("1000000000000000000").unwrap())),
//orion-money 18
(Address::from_str("727f064a78dc734d33eec18d5370aef32ffd46e4").unwrap(), (U256::from(4652),U256::from_dec_str("1000000000000000000").unwrap())),
//global-coin-research 4
(Address::from_str("6307b25a665efc992ec1c1bc403c38f3ddd7c661").unwrap(), (U256::from(21300),U256::from_dec_str("10000").unwrap())),
//unilend 18
(Address::from_str("0202Be363B8a4820f3F4DE7FaF5224fF05943AB1").unwrap(), (U256::from(5696),U256::from_dec_str("1000000000000000000").unwrap())),
//complifi 18
(Address::from_str("752efadc0a7e05ad1bcccda22c141d01a75ef1e4").unwrap(), (U256::from(17282),U256::from_dec_str("1000000000000000000").unwrap())),
//realm 18
(Address::from_str("464fdb8affc9bac185a7393fd4298137866dcfb8").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//robo-inu-finance 9
(Address::from_str("7b32e70e8d73ac87c1b342e063528b2930b15ceb").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//cino-games 18
(Address::from_str("7A2C7928c8CF294E25cA7db8a379278c5b0cFa0F").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//xdai 18
(Address::from_str("0Ae055097C6d159879521C384F1D2123D1f195e6").unwrap(), (U256::from(152900),U256::from_dec_str("1000000000000000000").unwrap())),
//nftrade 18
(Address::from_str("8e0fe2947752be0d5acf73aae77362daf79cb379").unwrap(), (U256::from(7477),U256::from_dec_str("1000000000000000000").unwrap())),
//jigstack 18
(Address::from_str("1f8a626883d7724dbd59ef51cbd4bf1cf2016d13").unwrap(), (U256::from(56),U256::from_dec_str("1000000000000000000").unwrap())),
//opendao 18
(Address::from_str("3b484b82567a09e2588a13d54d032153f0c0aee0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//dinger-token 9
(Address::from_str("9e5bd9d9fad182ff0a93ba8085b664bcab00fa68").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//ren 18
(Address::from_str("408e41876cccdc0f92210600ef50372656052a38").unwrap(), (U256::from(4773),U256::from_dec_str("1000000000000000000").unwrap())),
//tidal-finance 18
(Address::from_str("29cbd0510eec0327992cd6006e63f9fa8e7f33b7").unwrap(), (U256::from(23),U256::from_dec_str("1000000000000000000").unwrap())),
//pinknode 18
(Address::from_str("AF691508BA57d416f895e32a1616dA1024e882D2").unwrap(), (U256::from(1082),U256::from_dec_str("1000000000000000000").unwrap())),
//gamestarter 5
(Address::from_str("D567B5F02b9073aD3a982a099a23Bf019FF11d1c").unwrap(), (U256::from(15700),U256::from_dec_str("100000").unwrap())),
//yieldly 18
(Address::from_str("88cb253d4c8cab8cdf7948a9251db85a13669e23").unwrap(), (U256::from(179),U256::from_dec_str("1000000000000000000").unwrap())),
//cujo-inu 9
(Address::from_str("612c393dace91284dafc23e623aab084fa0ffa64").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//dia 18
(Address::from_str("84ca8bc7997272c7cfb4d0cd3d55cd942b3c9419").unwrap(), (U256::from(11299),U256::from_dec_str("1000000000000000000").unwrap())),
//powerpool 18
(Address::from_str("38e4adb44ef08f22f5b5b76a8f0c2d0dcbe7dca1").unwrap(), (U256::from(12922),U256::from_dec_str("1000000000000000000").unwrap())),
//mantra-dao 18
(Address::from_str("3593d125a4f7849a1b059e64f4517a86dd60c95d").unwrap(), (U256::from(1317),U256::from_dec_str("1000000000000000000").unwrap())),
//abyss 18
(Address::from_str("0e8d6b471e332f140e7d9dbb99e5e3822f728da6").unwrap(), (U256::from(622),U256::from_dec_str("1000000000000000000").unwrap())),
//mimir-token 18
(Address::from_str("71dc40668682a124231301414167e4cf7f55383c").unwrap(), (U256::from(2270),U256::from_dec_str("1000000000000000000").unwrap())),
//tether 6
(Address::from_str("dac17f958d2ee523a2206206994597c13d831ec7").unwrap(), (U256::from(10000),U256::from_dec_str("1000000").unwrap())),
//cia-protocol 9
(Address::from_str("52f4d5ee6c91e01be67ca1f64b11ed0ee370817d").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//coinweb 18
(Address::from_str("505b5eda5e25a67e1c24a2bf1a527ed9eb88bf04").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//ampnet-asset-platform-and-exchange 18
(Address::from_str("bfd815347d024f449886c171f78fa5b8e6790811").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//fear-nfts 18
(Address::from_str("88a9a52f944315d5b4e917b9689e65445c401e83").unwrap(), (U256::from(11000),U256::from_dec_str("1000000000000000000").unwrap())),
//velas 18
(Address::from_str("8c543aed163909142695f2d2acd0d55791a9edb9").unwrap(), (U256::from(3769),U256::from_dec_str("1000000000000000000").unwrap())),
//virtue-poker 18
(Address::from_str("5eeaa2dcb23056f4e8654a349e57ebe5e76b5e6e").unwrap(), (U256::from(1114),U256::from_dec_str("1000000000000000000").unwrap())),
//shopping 18
(Address::from_str("9b02dd390a603add5c07f9fd9175b7dabe8d63b7").unwrap(), (U256::from(174500),U256::from_dec_str("1000000000000000000").unwrap())),
//velaspad 18
(Address::from_str("b8e3bB633F7276cc17735D86154E0ad5ec9928C0").unwrap(), (U256::from(4803),U256::from_dec_str("1000000000000000000").unwrap())),
//uniqly 18
(Address::from_str("3758e00b100876c854636ef8db61988931bb8025").unwrap(), (U256::from(8572),U256::from_dec_str("1000000000000000000").unwrap())),
//pathdao 18
(Address::from_str("2a2550e0a75acec6d811ae3930732f7f3ad67588").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//hellsing-inu 9
(Address::from_str("b087c2180e3134db396977065817aed91fea6ead").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//rio-defi 18
(Address::from_str("af9f549774ecedbd0966c52f250acc548d3f36e5").unwrap(), (U256::from(403),U256::from_dec_str("1000000000000000000").unwrap())),
//ternoa 18
(Address::from_str("03be5c903c727ee2c8c4e9bc0acc860cca4715e2").unwrap(), (U256::from(858),U256::from_dec_str("1000000000000000000").unwrap())),
//nyx-token 9
(Address::from_str("118b552725e1892137740cB4d29390D952709639").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//unlock-protocol 18
(Address::from_str("90de74265a416e1393a450752175aed98fe11517").unwrap(), (U256::from(963800),U256::from_dec_str("1000000000000000000").unwrap())),
//armor 18
(Address::from_str("1337def16f9b486faed0293eb623dc8395dfe46a").unwrap(), (U256::from(531),U256::from_dec_str("1000000000000000000").unwrap())),
//shyft-network 18
(Address::from_str("b17C88bDA07D28B3838E0c1dE6a30eAfBCF52D85").unwrap(), (U256::from(4943),U256::from_dec_str("1000000000000000000").unwrap())),
//dose 18
(Address::from_str("b31ef9e52d94d4120eb44fe1ddfde5b4654a6515").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//goat-token 9
(Address::from_str("74edaf28fc4b9e6a1618d613839daaf6a9d075db").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//freeway-token 18
(Address::from_str("f151980e7a781481709e8195744bf2399fb3cba4").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//gamercoin 18
(Address::from_str("728f30fa2f100742c7949d1961804fa8e0b1387d").unwrap(), (U256::from(617),U256::from_dec_str("1000000000000000000").unwrap())),
//prosper 18
(Address::from_str("8642A849D0dcb7a15a974794668ADcfbe4794B56").unwrap(), (U256::from(10400),U256::from_dec_str("1000000000000000000").unwrap())),
//oin-finance 8
(Address::from_str("9aeB50f542050172359A0e1a25a9933Bc8c01259").unwrap(), (U256::from(1953),U256::from_dec_str("100000000").unwrap())),
//aag-ventures 18
(Address::from_str("5ba19d656b65f1684cfea4af428c23b9f3628f97").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//88mph 18
(Address::from_str("8888801af4d980682e47f1a9036e589479e835c5").unwrap(), (U256::from(324632),U256::from_dec_str("1000000000000000000").unwrap())),
//gerowallet 18
(Address::from_str("3431f91b3a388115f00c5ba9fdb899851d005fb5").unwrap(), (U256::from(549),U256::from_dec_str("1000000000000000000").unwrap())),
//saint-inu 9
(Address::from_str("6fc5af63990aa9e5c5543f5cd8ed148bfa6d9d19").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//zero-tech 18
(Address::from_str("0eC78ED49C2D27b315D462d43B5BAB94d2C79bf8").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//kollect 18
(Address::from_str("1CC30e2EAc975416060Ec6FE682041408420d414").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//hxro 18
(Address::from_str("4bd70556ae3f8a6ec6c4080a0c327b24325438f3").unwrap(), (U256::from(4636),U256::from_dec_str("1000000000000000000").unwrap())),
//crustnetwork 18
(Address::from_str("32a7C02e79c4ea1008dD6564b35F131428673c41").unwrap(), (U256::from(117825),U256::from_dec_str("1000000000000000000").unwrap())),
//basic-attention-token 18
(Address::from_str("0d8775f648430679a709e98d2b0cb6250d2887ef").unwrap(), (U256::from(10900),U256::from_dec_str("1000000000000000000").unwrap())),
//cream-finance 18
(Address::from_str("2ba592f78db6436527729929aaf6c908497cb200").unwrap(), (U256::from(417539),U256::from_dec_str("1000000000000000000").unwrap())),
//graphlinq-protocol 18
(Address::from_str("9f9c8ec3534c3ce16f928381372bfbfbfb9f4d24").unwrap(), (U256::from(197),U256::from_dec_str("1000000000000000000").unwrap())),
//filecoin-standard-hashrate-token 18
(Address::from_str("7346ad4c8cd1886ff6d16072bcea5dfc0bc24ca2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//energy-web-token 18
(Address::from_str("178c820f862b14f316509ec36b13123da19a6054").unwrap(), (U256::from(82410),U256::from_dec_str("1000000000000000000").unwrap())),
//epik-prime 18
(Address::from_str("4da0c48376c277cdbd7fc6fdc6936dee3e4adf75").unwrap(), (U256::from(1721),U256::from_dec_str("1000000000000000000").unwrap())),
//shiba-girlfriend 18
(Address::from_str("505a84a03e382331a1be487b632cf357748b65d6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//gro-dao-token 18
(Address::from_str("3Ec8798B81485A254928B70CDA1cf0A2BB0B74D7").unwrap(), (U256::from(35800),U256::from_dec_str("1000000000000000000").unwrap())),
//pulsepad 18
(Address::from_str("8a74bc8c372bc7f0e9ca3f6ac0df51be15aec47a").unwrap(), (U256::from(1222),U256::from_dec_str("1000000000000000000").unwrap())),
//aladdindao 18
(Address::from_str("b26C4B3Ca601136Daf98593feAeff9E0CA702a8D").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//ok-lets-go 9
(Address::from_str("5dbb9f64cd96e2dbbca58d14863d615b67b42f2e").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//talaria-inu 18
(Address::from_str("6765fdd028be3d7874bc2bb3d7d5ca01c1bf14b2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//fren-token 9
(Address::from_str("37941b3fdb2bd332e667d452a58be01bcacb923e").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//dforce 18
(Address::from_str("431ad2ff6a9c365805ebad47ee021148d6f7dbe0").unwrap(), (U256::from(1158),U256::from_dec_str("1000000000000000000").unwrap())),
//xfund 9
(Address::from_str("892a6f9df0147e5f079b0993f486f9aca3c87881").unwrap(), (U256::from(9078500),U256::from_dec_str("1000000000").unwrap())),
//brokoli-network 18
(Address::from_str("4674a4F24C5f63D53F22490Fb3A08eAAAD739ff8").unwrap(), (U256::from(4913),U256::from_dec_str("1000000000000000000").unwrap())),
//shinchan-token 9
(Address::from_str("baa9af8a83500ac4137c555b9e58ccb3e1f2269d").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//swissborg 8
(Address::from_str("ba9d4199fab4f26efe3551d490e3821486f135ba").unwrap(), (U256::from(5638),U256::from_dec_str("100000000").unwrap())),
//mobi-finance 18
(Address::from_str("b2dbf14d0b47ed3ba02bdb7c954e05a72deb7544").unwrap(), (U256::from(373),U256::from_dec_str("1000000000000000000").unwrap())),
//insured-finance 18
(Address::from_str("159751323a9e0415dd3d6d42a1212fe9f4a0848c").unwrap(), (U256::from(453),U256::from_dec_str("1000000000000000000").unwrap())),
//tenshi-new 18
(Address::from_str("52662717e448be36cb54588499d5a8328bd95292").unwrap(), (U256::from(88),U256::from_dec_str("1000000000000000000").unwrap())),
//frax 18
(Address::from_str("853d955acef822db058eb8505911ed77f175b99e").unwrap(), (U256::from(10100),U256::from_dec_str("1000000000000000000").unwrap())),
//smartkey 8
(Address::from_str("06A01a4d579479Dd5D884EBf61A31727A3d8D442").unwrap(), (U256::from(999),U256::from_dec_str("100000000").unwrap())),
//nord-finance 18
(Address::from_str("6e9730ecffbed43fd876a264c982e254ef05a0de").unwrap(), (U256::from(27799),U256::from_dec_str("1000000000000000000").unwrap())),
//unore 18
(Address::from_str("474021845c4643113458ea4414bdb7fb74a01a77").unwrap(), (U256::from(2808),U256::from_dec_str("1000000000000000000").unwrap())),
//masq 18
(Address::from_str("06f3c323f0238c72bf35011071f2b5b7f43a054c").unwrap(), (U256::from(1822),U256::from_dec_str("1000000000000000000").unwrap())),
//revv 18
(Address::from_str("557b933a7c2c45672b610f8954a3deb39a51a8ca").unwrap(), (U256::from(1317),U256::from_dec_str("1000000000000000000").unwrap())),
//dextf-protocol 18
(Address::from_str("5F64Ab1544D28732F0A24F4713c2C8ec0dA089f0").unwrap(), (U256::from(772),U256::from_dec_str("1000000000000000000").unwrap())),
//vent-finance 18
(Address::from_str("5f0bc16d50f72d10b719dbf6845de2e599eb5624").unwrap(), (U256::from(3108),U256::from_dec_str("1000000000000000000").unwrap())),
//coreto 18
(Address::from_str("9C2dc0c3CC2BADdE84B0025Cf4df1c5aF288D835").unwrap(), (U256::from(172),U256::from_dec_str("1000000000000000000").unwrap())),
//shibnaki 18
(Address::from_str("85122a589fc2a92cbe6c6606e2b6661fedfa67ee").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//gains-farm-v2 18
(Address::from_str("831091dA075665168E01898c6DAC004A867f1e1B").unwrap(), (U256::from(41897929),U256::from_dec_str("1000000000000000000").unwrap())),
//paxos-standard 18
(Address::from_str("8e870d67f660d95d5be530380d0ec0bd388289e1").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//piccolo-inu 9
(Address::from_str("3a1311b8c404629e38f61d566cefefed083b9670").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//dovu 18
(Address::from_str("ac3211a5025414af2866ff09c23fc18bc97e79b1").unwrap(), (U256::from(109),U256::from_dec_str("1000000000000000000").unwrap())),
//mandox 9
(Address::from_str("AFbF03181833aB4E8DEc24D708a2a24c2bAaa4a4").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//revomon 18
(Address::from_str("155040625D7ae3e9caDA9a73E3E44f76D3Ed1409").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//deku-inu 9
(Address::from_str("a1a88cea335edaf30ce90f103f1434a773ea46bd").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//robonomics-network 9
(Address::from_str("7de91b204c1c737bcee6f000aaa6569cf7061cb7").unwrap(), (U256::from(101300),U256::from_dec_str("1000000000").unwrap())),
//foam 18
(Address::from_str("4946fcea7c692606e8908002e55a582af44ac121").unwrap(), (U256::from(491),U256::from_dec_str("1000000000000000000").unwrap())),
//hiko-inu 9
(Address::from_str("1579d058918f339c945802ffac81762e432cd0b8").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//yaxis 18
(Address::from_str("0ada190c81b814548ddc2f6adc4a689ce7c1fe73").unwrap(), (U256::from(11187),U256::from_dec_str("1000000000000000000").unwrap())),
//finance-vote 18
(Address::from_str("45080a6531d671DDFf20DB42f93792a489685e32").unwrap(), (U256::from(102),U256::from_dec_str("1000000000000000000").unwrap())),
//mobiepay 18
(Address::from_str("71ba91dc68c6a206db0a6a92b4b1de3f9271432d").unwrap(), (U256::from(51),U256::from_dec_str("1000000000000000000").unwrap())),
//synapse-network 18
(Address::from_str("6911f552842236bd9e8ea8ddbb3fb414e2c5fa9d").unwrap(), (U256::from(2744),U256::from_dec_str("1000000000000000000").unwrap())),
//shibrwd 18
(Address::from_str("a518c9f3724cced4715e6813858dc2ce9b21ed78").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//susd 18
(Address::from_str("57Ab1ec28D129707052df4dF418D58a2D46d5f51").unwrap(), (U256::from(9865),U256::from_dec_str("1000000000000000000").unwrap())),
//amasa 18
(Address::from_str("65a8fbA02F641a13Bb7B01d5E1129b0521004f52").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//renzec 8
(Address::from_str("1c5db575e2ff833e46a2e9864c22f4b22e0b37c2").unwrap(), (U256::from(1461841),U256::from_dec_str("100000000").unwrap())),
//jupiter 18
(Address::from_str("4B1E80cAC91e2216EEb63e29B957eB91Ae9C2Be8").unwrap(), (U256::from(119),U256::from_dec_str("1000000000000000000").unwrap())),
//cyberfi 18
(Address::from_str("63b4f3e3fa4e438698ce330e365e831f7ccd1ef4").unwrap(), (U256::from(44139),U256::from_dec_str("1000000000000000000").unwrap())),
//bluesparrow-token 9
(Address::from_str("4d67edef87a5ff910954899f4e5a0aaf107afd42").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//lympo-market-token 18
(Address::from_str("327673ae6b33bd3d90f0096870059994f30dc8af").unwrap(), (U256::from(1220),U256::from_dec_str("1000000000000000000").unwrap())),
//cryptomeda 18
(Address::from_str("6286A9e6f7e745A6D884561D88F94542d6715698").unwrap(), (U256::from(122),U256::from_dec_str("1000000000000000000").unwrap())),
//friends-with-benefits-pro 18
(Address::from_str("35bd01fc9d6d5d81ca9e055db88dc49aa2c699a8").unwrap(), (U256::from(573503),U256::from_dec_str("1000000000000000000").unwrap())),
//nucypher 18
(Address::from_str("4fe83213d56308330ec302a8bd641f1d0113a4cc").unwrap(), (U256::from(6441),U256::from_dec_str("1000000000000000000").unwrap())),
//minishib-token 9
(Address::from_str("3c5bda020caa1350a7b4e6e013a2516423c2800f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//pepemon-pepeballs 18
(Address::from_str("4d2ee5dae46c86da2ff521f7657dad98834f97b8").unwrap(), (U256::from(640874),U256::from_dec_str("1000000000000000000").unwrap())),
//pulse-token 18
(Address::from_str("52a047ee205701895ee06a375492490ec9c597ce").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//fortune 9
(Address::from_str("9f009d03e1b7f02065017c90e8e0d5cb378eb015").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//schrodinger 9
(Address::from_str("2c33b28527a63cdf13c0b24ce4cf5bf9c9fb3bc6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//swipe 18
(Address::from_str("8ce9137d39326ad0cd6491fb5cc0cba0e089b6a9").unwrap(), (U256::from(16299),U256::from_dec_str("1000000000000000000").unwrap())),
//superbid 18
(Address::from_str("0563dce613d559a47877ffd1593549fb9d3510d6").unwrap(), (U256::from(8077),U256::from_dec_str("1000000000000000000").unwrap())),
//bright-union 18
(Address::from_str("beab712832112bd7664226db7cd025b153d3af55").unwrap(), (U256::from(891),U256::from_dec_str("1000000000000000000").unwrap())),
//mintyswap 9
(Address::from_str("bbd900e05b4af2124390d206f70bc4e583b1be85").unwrap(), (U256::from(688),U256::from_dec_str("1000000000").unwrap())),
//formation-fi 18
(Address::from_str("21381e026ad6d8266244f2a583b35f9e4413fa2a").unwrap(), (U256::from(505),U256::from_dec_str("1000000000000000000").unwrap())),
//blockchain-cuties-universe 18
(Address::from_str("14da7b27b2e0fedefe0a664118b0c9bc68e2e9af").unwrap(), (U256::from(8050),U256::from_dec_str("1000000000000000000").unwrap())),
//depay 18
(Address::from_str("a0bEd124a09ac2Bd941b10349d8d224fe3c955eb").unwrap(), (U256::from(6889),U256::from_dec_str("1000000000000000000").unwrap())),
//cardstack 18
(Address::from_str("954b890704693af242613edef1b603825afcd708").unwrap(), (U256::from(76),U256::from_dec_str("1000000000000000000").unwrap())),
//darwinia-network 18
(Address::from_str("9469d013805bffb7d3debe5e7839237e535ec483").unwrap(), (U256::from(365),U256::from_dec_str("1000000000000000000").unwrap())),
//tradestars 18
(Address::from_str("734c90044a0ba31b3f2e640c10dc5d3540499bfd").unwrap(), (U256::from(1729),U256::from_dec_str("1000000000000000000").unwrap())),
//blockswap-network 18
(Address::from_str("7d4b1d793239707445305d8d2456d2c735f6b25b").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//kira-network 6
(Address::from_str("16980b3b4a3f9d89e33311b5aa8f80303e5ca4f8").unwrap(), (U256::from(3754),U256::from_dec_str("1000000").unwrap())),
//beholder 18
(Address::from_str("155ff1A85F440EE0A382eA949f24CE4E0b751c65").unwrap(), (U256::from(4828),U256::from_dec_str("1000000000000000000").unwrap())),
//dvision-network 18
(Address::from_str("10633216e7e8281e33c86f02bf8e565a635d9770").unwrap(), (U256::from(9062),U256::from_dec_str("1000000000000000000").unwrap())),
//quid-ika 9
(Address::from_str("9d38f670d15c14716be1f109a4f453e966a2b6d4").unwrap(), (U256::from(185),U256::from_dec_str("1000000000").unwrap())),
//wrapped-nxm 18
(Address::from_str("0d438f3b5175bebc262bf23753c1e53d03432bde").unwrap(), (U256::from(556625),U256::from_dec_str("1000000000000000000").unwrap())),
//unilayer 18
(Address::from_str("0fF6ffcFDa92c53F615a4A75D982f399C989366b").unwrap(), (U256::from(2802),U256::from_dec_str("1000000000000000000").unwrap())),
//morpheus-labs 18
(Address::from_str("4a527d8fc13c5203ab24ba0944f4cb14658d1db6").unwrap(), (U256::from(270),U256::from_dec_str("1000000000000000000").unwrap())),
//sheesha-finance-erc20 18
(Address::from_str("232FB065D9d24c34708eeDbF03724f2e95ABE768").unwrap(), (U256::from(1456037),U256::from_dec_str("1000000000000000000").unwrap())),
//get-protocol 18
(Address::from_str("8a854288a5976036a725879164ca3e91d30c6a1b").unwrap(), (U256::from(17200),U256::from_dec_str("1000000000000000000").unwrap())),
//chain 18
(Address::from_str("41C37A4683d6a05adB31c39D71348A8403B13Ca9").unwrap(), (U256::from(1054000),U256::from_dec_str("1000000000000000000").unwrap())),
//ethereummax 18
(Address::from_str("15874d65e649880c2614e7a480cb7c9a55787ff6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//wrapped-gen-0-cryptokitties 18
(Address::from_str("a10740ff9ff6852eac84cdcff9184e1d6d27c057").unwrap(), (U256::from(3256800),U256::from_dec_str("1000000000000000000").unwrap())),
//tixl-new 18
(Address::from_str("8eEF5a82E6Aa222a60F009ac18c24EE12dBf4b41").unwrap(), (U256::from(973),U256::from_dec_str("1000000000000000000").unwrap())),
//pawthereum 9
(Address::from_str("aecc217a749c2405b5ebc9857a16d58bdc1c367f").unwrap(), (U256::from(57),U256::from_dec_str("1000000000").unwrap())),
//myneighboralice 6
(Address::from_str("AC51066d7bEC65Dc4589368da368b212745d63E8").unwrap(), (U256::from(115399),U256::from_dec_str("1000000").unwrap())),
//centaurify 18
(Address::from_str("08ba718f288c3b12b01146816bef9fa03cc635bc").unwrap(), (U256::from(152),U256::from_dec_str("1000000000000000000").unwrap())),
//shira-inu 9
(Address::from_str("04a5198063e45d84b1999516d3228167146417a6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//sovryn 18
(Address::from_str("bdab72602e9ad40fc6a6852caf43258113b8f7a5").unwrap(), (U256::from(74832),U256::from_dec_str("1000000000000000000").unwrap())),
//charged-particles 18
(Address::from_str("02d3a27ac3f55d5d91fb0f52759842696a864217").unwrap(), (U256::from(7735),U256::from_dec_str("1000000000000000000").unwrap())),
//ksm-starter 18
(Address::from_str("Bc17729fDf562723f0267F79FF25aDE441056d87").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//gamecredits 18
(Address::from_str("63f88a2298a5c4aee3c216aa6d926b184a4b2437").unwrap(), (U256::from(1351),U256::from_dec_str("1000000000000000000").unwrap())),
//metronome 18
(Address::from_str("a3d58c4e56fedcae3a7c43a725aee9a71f0ece4e").unwrap(), (U256::from(38600),U256::from_dec_str("1000000000000000000").unwrap())),
//bird-money 18
(Address::from_str("70401dfd142a16dc7031c56e862fc88cb9537ce0").unwrap(), (U256::from(753015),U256::from_dec_str("1000000000000000000").unwrap())),
//astroelon 9
(Address::from_str("97b65710d03e12775189f0d113202cc1443b0aa2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//btse 8
(Address::from_str("666d875c600aa06ac1cf15641361dec3b00432ef").unwrap(), (U256::from(65300),U256::from_dec_str("100000000").unwrap())),
//paralink-network 18
(Address::from_str("3a8d5BC8A8948b68DfC0Ce9C14aC4150e083518c").unwrap(), (U256::from(149),U256::from_dec_str("1000000000000000000").unwrap())),
//kunoichix 9
(Address::from_str("0b5ECBb411d8FE829e5eAc253EE1F2Dc05D8d1Ae").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//ureeqa 18
(Address::from_str("1735Db6AB5BAa19eA55d0AdcEeD7bcDc008B3136").unwrap(), (U256::from(1397),U256::from_dec_str("1000000000000000000").unwrap())),
//benchmark-protocol 9
(Address::from_str("67c597624b17b16fb77959217360b7cd18284253").unwrap(), (U256::from(14872),U256::from_dec_str("1000000000").unwrap())),
//ovr 18
(Address::from_str("21bfbda47a0b4b5b1248c767ee49f7caa9b23697").unwrap(), (U256::from(24100),U256::from_dec_str("1000000000000000000").unwrap())),
//gamezone 18
(Address::from_str("b6adb74efb5801160ff749b1985fd3bd5000e938").unwrap(), (U256::from(3862),U256::from_dec_str("1000000000000000000").unwrap())),
//defi-yield-protocol 18
(Address::from_str("961C8c0B1aaD0c0b10a51FeF6a867E3091BCef17").unwrap(), (U256::from(4566),U256::from_dec_str("1000000000000000000").unwrap())),
//dev-protocol 18
(Address::from_str("5caf454ba92e6f2c929df14667ee360ed9fd5b26").unwrap(), (U256::from(15800),U256::from_dec_str("1000000000000000000").unwrap())),
//matrix-samurai-rbxs 18
(Address::from_str("a9639160481b625ba43677be753e0a70bf58c647").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//dxdao 18
(Address::from_str("a1d65e8fb6e87b60feccbc582f7f97804b725521").unwrap(), (U256::from(5428640),U256::from_dec_str("1000000000000000000").unwrap())),
//umbria-network 18
(Address::from_str("a4bbe66f151b22b167127c770016b15ff97dd35c").unwrap(), (U256::from(20742),U256::from_dec_str("1000000000000000000").unwrap())),
//bankless-dao 18
(Address::from_str("2d94aa3e47d9d5024503ca8491fce9a2fb4da198").unwrap(), (U256::from(733),U256::from_dec_str("1000000000000000000").unwrap())),
//arcona 18
(Address::from_str("0f71b8de197a1c84d31de0f1fa7926c365f052b3").unwrap(), (U256::from(7181),U256::from_dec_str("1000000000000000000").unwrap())),
//grey-token 9
(Address::from_str("9b2D81A1AE36E8e66A0875053429816f0B6b829E").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//moonie-nft 18
(Address::from_str("A6F7645ed967FAF708A614a2fcA8D4790138586f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//plasma-finance 18
(Address::from_str("054d64b73d3d8a21af3d764efd76bcaa774f3bb2").unwrap(), (U256::from(644),U256::from_dec_str("1000000000000000000").unwrap())),
//crowns 18
(Address::from_str("ac0104cca91d167873b8601d2e71eb3d4d8c33e0").unwrap(), (U256::from(64500),U256::from_dec_str("1000000000000000000").unwrap())),
//archangel-token 9
(Address::from_str("36e43065e977bc72cb86dbd8405fae7057cdc7fd").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//unimex-network 18
(Address::from_str("10be9a8dae441d276a5027936c3aaded2d82bc15").unwrap(), (U256::from(3176),U256::from_dec_str("1000000000000000000").unwrap())),
//rlc 9
(Address::from_str("607f4c5bb672230e8672085532f7e901544a7375").unwrap(), (U256::from(28700),U256::from_dec_str("1000000000").unwrap())),
//launchpool 18
(Address::from_str("6149c26cd2f7b5ccdb32029af817123f6e37df5b").unwrap(), (U256::from(25900),U256::from_dec_str("1000000000000000000").unwrap())),
//olyseum 18
(Address::from_str("6595b8fd9c920c81500dca94e53cdc712513fb1f").unwrap(), (U256::from(52),U256::from_dec_str("1000000000000000000").unwrap())),
//balancer 18
(Address::from_str("ba100000625a3754423978a60c9317c58a424e3D").unwrap(), (U256::from(179500),U256::from_dec_str("1000000000000000000").unwrap())),
//polytrade 18
(Address::from_str("6e5970DBd6fc7eb1f29C6D2eDF2bC4c36124C0C1").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//alpaca-city 18
(Address::from_str("7cA4408137eb639570F8E647d9bD7B7E8717514A").unwrap(), (U256::from(2518),U256::from_dec_str("1000000000000000000").unwrap())),
//dos-network 18
(Address::from_str("0A913beaD80F321E7Ac35285Ee10d9d922659cB7").unwrap(), (U256::from(162),U256::from_dec_str("1000000000000000000").unwrap())),
//tontoken 18
(Address::from_str("6a6c2ada3ce053561c2fbc3ee211f23d9b8c520a").unwrap(), (U256::from(334),U256::from_dec_str("1000000000000000000").unwrap())),
//koinos 8
(Address::from_str("66d28cb58487a7609877550e1a34691810a6b9fc").unwrap(), (U256::from(6863),U256::from_dec_str("100000000").unwrap())),
//nitro-league 18
(Address::from_str("0335A7610D817aeCA1bEBbEfbd392ecC2eD587B8").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//adx-net 18
(Address::from_str("ade00c28244d5ce17d72e40330b1c318cd12b7c3").unwrap(), (U256::from(5116),U256::from_dec_str("1000000000000000000").unwrap())),
//yeager-inu 9
(Address::from_str("8966f05d78f5c6ede8e964df705847fe2b6045b1").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//shiba-viking 18
(Address::from_str("040a856f2c59bb49166210a54a55d0b2599b46d8").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//maps 6
(Address::from_str("2b915b505c017abb1547aa5ab355fbe69865cc6d").unwrap(), (U256::from(2501),U256::from_dec_str("1000000").unwrap())),
//continuum-world 18
(Address::from_str("b19dd661f076998e3b0456935092a233e12c2280").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//azuki 18
(Address::from_str("910524678C0B1B23FFB9285a81f99C29C11CBaEd").unwrap(), (U256::from(289),U256::from_dec_str("1000000000000000000").unwrap())),
//index-cooperative 18
(Address::from_str("0954906da0Bf32d5479e25f46056d22f08464cab").unwrap(), (U256::from(131052),U256::from_dec_str("1000000000000000000").unwrap())),
//spice-dao 18
(Address::from_str("9b6db7597a74602a5a806e33408e7e2dafa58193").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//poolz-finance 18
(Address::from_str("69a95185ee2a045cdc4bcd1b1df10710395e4e23").unwrap(), (U256::from(58800),U256::from_dec_str("1000000000000000000").unwrap())),
//crypto-com-coin 8
(Address::from_str("a0b73e1ff0b80914ab6fe0444e65848c4c34450b").unwrap(), (U256::from(4721),U256::from_dec_str("100000000").unwrap())),
//darwinia-commitment-token 18
(Address::from_str("9f284e1337a815fe77d2ff4ae46544645b20c5ff").unwrap(), (U256::from(487600),U256::from_dec_str("1000000000000000000").unwrap())),
//multi-chain-capital 9
(Address::from_str("1a7981d87e3b6a95c1516eb820e223fe979896b3").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//lithium 18
(Address::from_str("188e817b02e635d482ae4d81e25dda98a97c4a42").unwrap(), (U256::from(112),U256::from_dec_str("1000000000000000000").unwrap())),
//voxel-x-network 18
(Address::from_str("16CC8367055aE7e9157DBcB9d86Fd6CE82522b31").unwrap(), (U256::from(995),U256::from_dec_str("1000000000000000000").unwrap())),
//evedo 18
(Address::from_str("5aaefe84e0fb3dd1f0fcff6fa7468124986b91bd").unwrap(), (U256::from(937),U256::from_dec_str("1000000000000000000").unwrap())),
//kiki 18
(Address::from_str("369b77bbeeee50e6ea206dcf41ee670c47360055").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//hyprr 18
(Address::from_str("12f649a9e821f90bb143089a6e56846945892ffb").unwrap(), (U256::from(42),U256::from_dec_str("1000000000000000000").unwrap())),
//raze-network 18
(Address::from_str("5Eaa69B29f99C84Fe5dE8200340b4e9b4Ab38EaC").unwrap(), (U256::from(814),U256::from_dec_str("1000000000000000000").unwrap())),
//uniris 18
(Address::from_str("8a3d77e9d6968b780564936d15B09805827C21fa").unwrap(), (U256::from(1957),U256::from_dec_str("1000000000000000000").unwrap())),
//steth 18
(Address::from_str("ae7ab96520de3a18e5e111b5eaab095312d7fe84").unwrap(), (U256::from(32179400),U256::from_dec_str("1000000000000000000").unwrap())),
//block-duelers 18
(Address::from_str("7bce667ef12023dc5f8577d015a2f09d99a5ef58").unwrap(), (U256::from(190100),U256::from_dec_str("1000000000000000000").unwrap())),
//e1337 4
(Address::from_str("35872fea6a4843facbcdbce99e3b69596a3680b8").unwrap(), (U256::from(8583),U256::from_dec_str("10000").unwrap())),
//triall 18
(Address::from_str("58f9102bf53cf186682bd9a281d3cd3c616eec41").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//versoview 18
(Address::from_str("755be920943eA95e39eE2DC437b268917B580D6e").unwrap(), (U256::from(1273),U256::from_dec_str("1000000000000000000").unwrap())),
//theos 18
(Address::from_str("9e10f61749c4952c320412a6b26901605ff6da1d").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//polkamon 18
(Address::from_str("1796ae0b0fa4862485106a0de9b654eFE301D0b2").unwrap(), (U256::from(53400),U256::from_dec_str("1000000000000000000").unwrap())),
//defil 18
(Address::from_str("09ce2b746c32528b7d864a1e3979bd97d2f095ab").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//naraka-token 9
(Address::from_str("8e3fe7cdf4ebb605bbbac3a43d76ea757f7f06e2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//fomoeth 9
(Address::from_str("8a65b987d9813f0a97446eda0de918b2573ae406").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//less-network 18
(Address::from_str("62786eeacc9246b4018e0146cb7a3efeacd9459d").unwrap(), (U256::from(191),U256::from_dec_str("1000000000000000000").unwrap())),
//degate 18
(Address::from_str("53c8395465a84955c95159814461466053dedede").unwrap(), (U256::from(2185),U256::from_dec_str("1000000000000000000").unwrap())),
//peanut 18
(Address::from_str("89bd2e7e388fab44ae88bef4e1ad12b4f1e0911c").unwrap(), (U256::from(2275),U256::from_dec_str("1000000000000000000").unwrap())),
//wgmi 18
(Address::from_str("20f6a313cb250062331fe70b9567e3ee5f01888b").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//metavpad 18
(Address::from_str("62858686119135cc00C4A3102b436a0eB314D402").unwrap(), (U256::from(3323),U256::from_dec_str("1000000000000000000").unwrap())),
//doubledice-token 18
(Address::from_str("4e08f03079c5cd3083ea331ec61bcc87538b7665").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//shadows 18
(Address::from_str("661ab0ed68000491d98c796146bcf28c20d7c559").unwrap(), (U256::from(974),U256::from_dec_str("1000000000000000000").unwrap())),
//iagon 18
(Address::from_str("40eb746dee876ac1e78697b7ca85142d178a1fc8").unwrap(), (U256::from(366),U256::from_dec_str("1000000000000000000").unwrap())),
//ix-swap 18
(Address::from_str("73d7c860998ca3c01ce8c808f5577d94d545d1b4").unwrap(), (U256::from(1499),U256::from_dec_str("1000000000000000000").unwrap())),
//bluzelle 18
(Address::from_str("5732046a883704404f284ce41ffadd5b007fd668").unwrap(), (U256::from(1903),U256::from_dec_str("1000000000000000000").unwrap())),
//cat-token 18
(Address::from_str("56015bbe3c01fe05bc30a8a9a9fd9a88917e7db3").unwrap(), (U256::from(2827),U256::from_dec_str("1000000000000000000").unwrap())),
//bunnyverse 18
(Address::from_str("072987d5b36ad8d45552aed98879a7101ccdd749").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//hord 18
(Address::from_str("43a96962254855f16b925556f9e97be436a43448").unwrap(), (U256::from(1097),U256::from_dec_str("1000000000000000000").unwrap())),
//mxc 18
(Address::from_str("5ca381bbfb58f0092df149bd3d243b08b9a8386e").unwrap(), (U256::from(625),U256::from_dec_str("1000000000000000000").unwrap())),
//sekuritance 18
(Address::from_str("887168120cb89fb06f3e74dc4af20d67df0977f6").unwrap(), (U256::from(80),U256::from_dec_str("1000000000000000000").unwrap())),
//nil-coin 8
(Address::from_str("0eb638648207d00b9025684d13b1cb53806debe4").unwrap(), (U256::from(0),U256::from_dec_str("100000000").unwrap())),
//locgame 18
(Address::from_str("60eb57d085c59932d5faa6c6026268a4386927d0").unwrap(), (U256::from(1082),U256::from_dec_str("1000000000000000000").unwrap())),
//unitrade 18
(Address::from_str("6f87d756daf0503d08eb8993686c7fc01dc44fb1").unwrap(), (U256::from(854),U256::from_dec_str("1000000000000000000").unwrap())),
//official-crypto-cowboy-token 18
(Address::from_str("95a1796437bad6502d1c1cce165cd76e522409a9").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//origin-protocol 18
(Address::from_str("8207c1ffc5b6804f6024322ccf34f29c3541ae26").unwrap(), (U256::from(5581),U256::from_dec_str("1000000000000000000").unwrap())),
//dexkit 18
(Address::from_str("7866e48c74cbfb8183cd1a929cd9b95a7a5cb4f4").unwrap(), (U256::from(9708),U256::from_dec_str("1000000000000000000").unwrap())),
//sator 9
(Address::from_str("3EF389f264e07fFF3106A3926F2a166d1393086F").unwrap(), (U256::from(1318),U256::from_dec_str("1000000000").unwrap())),
//node-runners 18
(Address::from_str("739763a258640919981F9bA610AE65492455bE53").unwrap(), (U256::from(320500),U256::from_dec_str("1000000000000000000").unwrap())),
//defiville 18
(Address::from_str("20a68f9e34076b2dc15ce726d7eebb83b694702d").unwrap(), (U256::from(10000),U256::from_dec_str("1000000000000000000").unwrap())),
//nftlootbox 18
(Address::from_str("7b3D36Eb606f873A75A6aB68f8c999848B04F935").unwrap(), (U256::from(580400),U256::from_dec_str("1000000000000000000").unwrap())),
//spike-inu 9
(Address::from_str("0f3debf94483beecbfd20167c946a61ea62d000f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//dego-finance 18
(Address::from_str("88ef27e69108b2633f8e1c184cc37940a075cc02").unwrap(), (U256::from(53118),U256::from_dec_str("1000000000000000000").unwrap())),
//phala-network 18
(Address::from_str("6c5bA91642F10282b576d91922Ae6448C9d52f4E").unwrap(), (U256::from(3741),U256::from_dec_str("1000000000000000000").unwrap())),
//blockzerolabs 18
(Address::from_str("0f7F961648aE6Db43C75663aC7E5414Eb79b5704").unwrap(), (U256::from(1025),U256::from_dec_str("1000000000000000000").unwrap())),
//ore-network 18
(Address::from_str("4f640F2529ee0cF119A2881485845FA8e61A782A").unwrap(), (U256::from(724),U256::from_dec_str("1000000000000000000").unwrap())),
//beyond-finance 18
(Address::from_str("4bb3205bf648b7f59ef90dee0f1b62f6116bc7ca").unwrap(), (U256::from(4902),U256::from_dec_str("1000000000000000000").unwrap())),
//huobi-token 18
(Address::from_str("6f259637dcd74c767781e37bc6133cd6a68aa161").unwrap(), (U256::from(90200),U256::from_dec_str("1000000000000000000").unwrap())),
//compound 18
(Address::from_str("c00e94cb662c3520282e6f5717214004a7f26888").unwrap(), (U256::from(1975200),U256::from_dec_str("1000000000000000000").unwrap())),
//polyswarm 18
(Address::from_str("9e46a38f5daabe8683e10793b06749eef7d733d1").unwrap(), (U256::from(245),U256::from_dec_str("1000000000000000000").unwrap())),
//numeraire 18
(Address::from_str("1776e1F26f98b1A5dF9cD347953a26dd3Cb46671").unwrap(), (U256::from(282200),U256::from_dec_str("1000000000000000000").unwrap())),
//rupiah-token 2
(Address::from_str("998FFE1E43fAcffb941dc337dD0468d52bA5b48A").unwrap(), (U256::from(1),U256::from_dec_str("100").unwrap())),
//daostack 18
(Address::from_str("543ff227f64aa17ea132bf9886cab5db55dcaddf").unwrap(), (U256::from(357),U256::from_dec_str("1000000000000000000").unwrap())),
//airswap 4
(Address::from_str("27054b13b1b798b345b591a4d22e6562d47ea75a").unwrap(), (U256::from(2337),U256::from_dec_str("10000").unwrap())),
//chihiro-inu 9
(Address::from_str("35156b404c3f9bdaf45ab65ba315419bcde3775c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//ptokens-btc 18
(Address::from_str("5228a22e72ccc52d415ecfd199f99d0665e7733b").unwrap(), (U256::from(396080000),U256::from_dec_str("1000000000000000000").unwrap())),
//sparkpoint 18
(Address::from_str("0488401c3f535193fa8df029d9ffe615a06e74e6").unwrap(), (U256::from(11),U256::from_dec_str("1000000000000000000").unwrap())),
//argoapp 18
(Address::from_str("28cca76f6e8ec81e4550ecd761f899110b060e97").unwrap(), (U256::from(1680),U256::from_dec_str("1000000000000000000").unwrap())),
//swapp-protocol 18
(Address::from_str("8cb924583681cbfe487a62140a994a49f833c244").unwrap(), (U256::from(82),U256::from_dec_str("1000000000000000000").unwrap())),
//dotmoovs 18
(Address::from_str("24ec2ca132abf8f6f8a6e24a1b97943e31f256a7").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//blackdragon 18
(Address::from_str("4Efe8665e564bF454cCF5C90Ee16817F7485d5Cf").unwrap(), (U256::from(109500),U256::from_dec_str("1000000000000000000").unwrap())),
//neoworld-cash 18
(Address::from_str("4b94c8567763654101f690cf4d54957206383b75").unwrap(), (U256::from(1),U256::from_dec_str("1000000000000000000").unwrap())),
//gnosis-gno 18
(Address::from_str("6810e776880c02933d47db1b9fc05908e5386b96").unwrap(), (U256::from(4531100),U256::from_dec_str("1000000000000000000").unwrap())),
//measurable-data-token 18
(Address::from_str("814e0908b12a99fecf5bc101bb5d0b8b5cdf7d26").unwrap(), (U256::from(934),U256::from_dec_str("1000000000000000000").unwrap())),
//kounotori 9
(Address::from_str("616ef40d55c0d2c506f4d6873bda8090b79bf8fc").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//mettalex 18
(Address::from_str("2e1e15c44ffe4df6a0cb7371cd00d5028e571d14").unwrap(), (U256::from(10116),U256::from_dec_str("1000000000000000000").unwrap())),
//ridge 9
(Address::from_str("64609A845Ad463d07ee51e91a88D1461C3Dc3165").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//digital-fitness 18
(Address::from_str("84cffa78b2fbbeec8c37391d2b12a04d2030845e").unwrap(), (U256::from(315),U256::from_dec_str("1000000000000000000").unwrap())),
//the-citadel 9
(Address::from_str("849ba2278cdae7fa7006c0661fea1c35d5af3336").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//the-rare-antiquities-token 18
(Address::from_str("6460b9954a05714a1a8d36bac6d8bc9b657352d7").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//kaiba-inu 9
(Address::from_str("8bb048845ee0d75be8e07954b2e1e5b51b64b442").unwrap(), (U256::from(300),U256::from_dec_str("1000000000").unwrap())),
//tower-token 18
(Address::from_str("1c9922314ed1415c95b9fd453c3818fd41867d0b").unwrap(), (U256::from(449),U256::from_dec_str("1000000000000000000").unwrap())),
//bitant 18
(Address::from_str("15Ee120fD69BEc86C1d38502299af7366a41D1a6").unwrap(), (U256::from(9),U256::from_dec_str("1000000000000000000").unwrap())),
//ally-direct-token 18
(Address::from_str("9d561d63375672ABd02119b9Bc4FB90EB9E307Ca").unwrap(), (U256::from(51),U256::from_dec_str("1000000000000000000").unwrap())),
//efforce 18
(Address::from_str("34950ff2b487d9e5282c5ab342d08a2f712eb79f").unwrap(), (U256::from(2118),U256::from_dec_str("1000000000000000000").unwrap())),
//oxen 9
(Address::from_str("d1e2d5085b39b80c9948aeb1b9aa83af6756bcc5").unwrap(), (U256::from(5135),U256::from_dec_str("1000000000").unwrap())),
//unistake 18
(Address::from_str("9ed8e7c9604790f7ec589f99b94361d8aab64e5e").unwrap(), (U256::from(158),U256::from_dec_str("1000000000000000000").unwrap())),
//decentralized-nations 18
(Address::from_str("15f0eedf9ce24fc4b6826e590a8292ce5524a1da").unwrap(), (U256::from(19294),U256::from_dec_str("1000000000000000000").unwrap())),
//safeswap-governance-token 18
(Address::from_str("2ecc48ba346a73d7d55aa5a46b5e314d9daa6161").unwrap(), (U256::from(247),U256::from_dec_str("1000000000000000000").unwrap())),
//zap 18
(Address::from_str("6781a0f84c7e9e846dcb84a9a5bd49333067b104").unwrap(), (U256::from(151),U256::from_dec_str("1000000000000000000").unwrap())),
//bzx-protocol 18
(Address::from_str("56d811088235F11C8920698a204A5010a788f4b3").unwrap(), (U256::from(2612),U256::from_dec_str("1000000000000000000").unwrap())),
//digicol 18
(Address::from_str("63B8b7d4A3EFD0735c4BFFBD95B332a55e4eB851").unwrap(), (U256::from(156),U256::from_dec_str("1000000000000000000").unwrap())),
//kawakami-inu 18
(Address::from_str("546aed37d202d607f45cbd2b8c0cad0d25fbe339").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//winry-inu 9
(Address::from_str("1a87077c4f834884691b8ba4fc808d2ec93a9f30").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//lunr-token 4
(Address::from_str("A87135285Ae208e22068AcDBFf64B11Ec73EAa5A").unwrap(), (U256::from(13300),U256::from_dec_str("10000").unwrap())),
//sportx 18
(Address::from_str("99fe3b1391503a1bc1788051347a1324bff41452").unwrap(), (U256::from(4813),U256::from_dec_str("1000000000000000000").unwrap())),
//degen-index 18
(Address::from_str("126c121f99e1e211df2e5f8de2d96fa36647c855").unwrap(), (U256::from(44052),U256::from_dec_str("1000000000000000000").unwrap())),
//redpanda 9
(Address::from_str("514cdb9cd8a2fb2bdcf7a3b8ddd098caf466e548").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//axl-inu 18
(Address::from_str("25b24b3c47918b7962b3e49c4f468367f73cc0e0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//santiment 18
(Address::from_str("7c5a0ce9267ed19b22f8cae653f198e3e8daf098").unwrap(), (U256::from(2916),U256::from_dec_str("1000000000000000000").unwrap())),
//nftify 18
(Address::from_str("aCbd826394189Cf2623C6DF98a18b41fC8fFC16D").unwrap(), (U256::from(509),U256::from_dec_str("1000000000000000000").unwrap())),
//oxygen 6
(Address::from_str("965697b4ef02f0de01384d0d4f9f782b1670c163").unwrap(), (U256::from(6075),U256::from_dec_str("1000000").unwrap())),
//keyfi 18
(Address::from_str("b8647e90c0645152fccf4d9abb6b59eb4aa99052").unwrap(), (U256::from(7065),U256::from_dec_str("1000000000000000000").unwrap())),
//serum 6
(Address::from_str("476c5e26a75bd202a9683ffd34359c0cc15be0ff").unwrap(), (U256::from(31800),U256::from_dec_str("1000000").unwrap())),
//allbridge 18
(Address::from_str("a11bd36801d8fa4448f0ac4ea7a62e3634ce8c7c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//wrapped-virgin-gen-0-cryptokitties 18
(Address::from_str("25c7b64a93eb1261e130ec21a3e9918caa38b611").unwrap(), (U256::from(3833899),U256::from_dec_str("1000000000000000000").unwrap())),
//hanzo-inu 9
(Address::from_str("239dc02a28a0774738463e06245544a72745d5c5").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//south-african-tether 18
(Address::from_str("48f07301e9e29c3c38a80ae8d9ae771f224f1054").unwrap(), (U256::from(603),U256::from_dec_str("1000000000000000000").unwrap())),
//spheroid-universe 18
(Address::from_str("a0cf46eb152656c7090e769916eb44a138aaa406").unwrap(), (U256::from(719),U256::from_dec_str("1000000000000000000").unwrap())),
//4thpillar-technologies 18
(Address::from_str("4730fb1463a6f1f44aeb45f6c5c422427f37f4d0").unwrap(), (U256::from(69),U256::from_dec_str("1000000000000000000").unwrap())),
//zuz-protocol 18
(Address::from_str("202f1877e1db1120ca3e9a98c5d505e7f035c249").unwrap(), (U256::from(3216),U256::from_dec_str("1000000000000000000").unwrap())),
//metadoge-token 18
(Address::from_str("8530b66ca3ddf50e0447eae8ad7ea7d5e62762ed").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//disbalancer 18
(Address::from_str("7fbec0bb6a7152e77c30d005b5d49cbc08a602c3").unwrap(), (U256::from(6221),U256::from_dec_str("1000000000000000000").unwrap())),
//plgnet 18
(Address::from_str("47da5456bc2e1ce391b645ce80f2e97192e4976a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//magickdao 9
(Address::from_str("6b578f63a40173d85215cc01d6d79e553e8c993c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//vaiot 18
(Address::from_str("9f801c1f02af03cc240546dadef8e56cd46ea2e9").unwrap(), (U256::from(2098),U256::from_dec_str("1000000000000000000").unwrap())),
//saja 9
(Address::from_str("698c6ac9ca5f16cabc5a636d3a619329c0958cba").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//standard-protocol 18
(Address::from_str("9040e237C3bF18347bb00957Dc22167D0f2b999d").unwrap(), (U256::from(3247),U256::from_dec_str("1000000000000000000").unwrap())),
//buying-com 2
(Address::from_str("396ec402b42066864c406d1ac3bc86b575003ed8").unwrap(), (U256::from(0),U256::from_dec_str("100").unwrap())),
//dfyn-network 18
(Address::from_str("9695e0114e12c0d3a3636fab5a18e6b737529023").unwrap(), (U256::from(1911),U256::from_dec_str("1000000000000000000").unwrap())),
//shoefy 18
(Address::from_str("0fD67B4ceb9b607Ef206904eC73459c4880132c9").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//rickmortydoxx 9
(Address::from_str("5d29011d843b0b1760c43e10d66f302174bccd1a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//uncl 18
(Address::from_str("2f4eb47a1b1f4488c71fc10e39a4aa56af33dd49").unwrap(), (U256::from(379561),U256::from_dec_str("1000000000000000000").unwrap())),
//nexo 18
(Address::from_str("b62132e35a6c13ee1ee0f84dc5d40bad8d815206").unwrap(), (U256::from(21900),U256::from_dec_str("1000000000000000000").unwrap())),
//hegic 18
(Address::from_str("584bC13c7D411c00c01A62e8019472dE68768430").unwrap(), (U256::from(592),U256::from_dec_str("1000000000000000000").unwrap())),
//dao-vc 18
(Address::from_str("284b59cf2539544559c6efa11e2795e06d535345").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//floki-gold 9
(Address::from_str("9f9fd5872beb21392f286afc6eb3a0f8154384fc").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//koka-inu 18
(Address::from_str("ac5bf342763248702f4fbd6dc068381a609543a2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//new-year-resolution 18
(Address::from_str("3eCF9840DEB8e3c395E1941Fc39ceB662BF5A1Dd").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//router-protocol 18
(Address::from_str("16eccfdbb4ee1a85a33f3a9b21175cd7ae753db4").unwrap(), (U256::from(26800),U256::from_dec_str("1000000000000000000").unwrap())),
//crypto-perx 18
(Address::from_str("c6e145421fd494b26dcf2bfeb1b02b7c5721978f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//konomi-network 18
(Address::from_str("850aAB69f0e0171A9a49dB8BE3E71351c8247Df4").unwrap(), (U256::from(2596),U256::from_dec_str("1000000000000000000").unwrap())),
//goku 9
(Address::from_str("a64dfe8d86963151e6496bee513e366f6e42ed79").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//crust-shadow 18
(Address::from_str("2620638EDA99F9e7E902Ea24a285456EE9438861").unwrap(), (U256::from(381),U256::from_dec_str("1000000000000000000").unwrap())),
//gamesta 18
(Address::from_str("55cd00764E85AA3B6b34130C983fFf9eB458250c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//xaya 8
(Address::from_str("6DC02164d75651758aC74435806093E421b64605").unwrap(), (U256::from(0),U256::from_dec_str("100000000").unwrap())),
//workquest 18
(Address::from_str("06677dc4fe12d3ba3c7ccfd0df8cd45e4d4095bf").unwrap(), (U256::from(329),U256::from_dec_str("1000000000000000000").unwrap())),
//ethereum-wrapped-filecoin 18
(Address::from_str("4b7ee45f30767f36f06f79b32bf1fca6f726deda").unwrap(), (U256::from(321200),U256::from_dec_str("1000000000000000000").unwrap())),
//mixmarvel 18
(Address::from_str("5d285f735998f36631f678ff41fb56a10a4d0429").unwrap(), (U256::from(132),U256::from_dec_str("1000000000000000000").unwrap())),
//pussy-financial 18
(Address::from_str("9196e18bc349b1f64bc08784eae259525329a1ad").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//a2dao 18
(Address::from_str("8052327F1BAF94A9DC8B26b9100f211eE3774f54").unwrap(), (U256::from(10200),U256::from_dec_str("1000000000000000000").unwrap())),
//totemfi 18
(Address::from_str("6ff1bfa14a57594a5874b37ff6ac5efbd9f9599a").unwrap(), (U256::from(1616),U256::from_dec_str("1000000000000000000").unwrap())),
//daoventures 18
(Address::from_str("77dcE26c03a9B833fc2D7C31C22Da4f42e9d9582").unwrap(), (U256::from(573),U256::from_dec_str("1000000000000000000").unwrap())),
//bonfi 18
(Address::from_str("1DE5e000C41C8d35b9f1f4985C23988f05831057").unwrap(), (U256::from(21),U256::from_dec_str("1000000000000000000").unwrap())),
//impactx 9
(Address::from_str("5af6ad286c8ed6633284f2f135c4716057d52669").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//energi 18
(Address::from_str("1416946162b1c2c871a73b07e932d2fb6c932069").unwrap(), (U256::from(10800),U256::from_dec_str("1000000000000000000").unwrap())),
//dark-energy-crystals 3
(Address::from_str("9393fdc77090f31c7db989390d43f454b1a6e7f3").unwrap(), (U256::from(0),U256::from_dec_str("1000").unwrap())),
//panda-inu 9
(Address::from_str("aa0bd7A009b189EAeab81dfA5e899CB137E0Fc3f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//defichain 8
(Address::from_str("8fc8f8269ebca376d046ce292dc7eac40c8d358a").unwrap(), (U256::from(0),U256::from_dec_str("100000000").unwrap())),
//lcx 18
(Address::from_str("037a54aab062628c9bbae1fdb1583c195585fe41").unwrap(), (U256::from(1580),U256::from_dec_str("1000000000000000000").unwrap())),
//truepnl 18
(Address::from_str("9fc8f0ca1668e87294941b7f627e9c15ea06b459").unwrap(), (U256::from(1021),U256::from_dec_str("1000000000000000000").unwrap())),
//spiderdao 18
(Address::from_str("bcd4b7de6fde81025f74426d43165a5b0d790fdd").unwrap(), (U256::from(85),U256::from_dec_str("1000000000000000000").unwrap())),
//portion 18
(Address::from_str("6D0F5149c502faf215C89ab306ec3E50b15e2892").unwrap(), (U256::from(108),U256::from_dec_str("1000000000000000000").unwrap())),
//ampleforth-governance-token 18
(Address::from_str("77fba179c79de5b7653f68b5039af940ada60ce0").unwrap(), (U256::from(81900),U256::from_dec_str("1000000000000000000").unwrap())),
//wagmi-game 18
(Address::from_str("1e987df68cc13d271e621ec82e050a1bbd62c180").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//0xbtc 8
(Address::from_str("b6ed7644c69416d67b522e20bc294a9a9b405b31").unwrap(), (U256::from(16600),U256::from_dec_str("100000000").unwrap())),
//unmarshal-token 18
(Address::from_str("5a666c7d92E5fA7Edcb6390E4efD6d0CDd69cF37").unwrap(), (U256::from(3988),U256::from_dec_str("1000000000000000000").unwrap())),
//capital-dao-protocol 18
(Address::from_str("3c48ca59bf2699e51d4974d4b6d284ae52076e5e").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//tokenize-xchange 8
(Address::from_str("667102BD3413bFEaa3Dffb48fa8288819E480a88").unwrap(), (U256::from(75977),U256::from_dec_str("100000000").unwrap())),
//tosdis 18
(Address::from_str("220b71671b649c03714da9c621285943f3cbcdc6").unwrap(), (U256::from(189700),U256::from_dec_str("1000000000000000000").unwrap())),
//minter-network 18
(Address::from_str("cafe34bae6f1b23a6b575303edcc0578d2188131").unwrap(), (U256::from(28),U256::from_dec_str("1000000000000000000").unwrap())),
//bigshortbets 18
(Address::from_str("131157c6760f78f7ddf877c0019eba175ba4b6f6").unwrap(), (U256::from(6010),U256::from_dec_str("1000000000000000000").unwrap())),
//band-protocol 18
(Address::from_str("ba11d00c5f74255f56a5e366f4f77f5a186d7f55").unwrap(), (U256::from(53700),U256::from_dec_str("1000000000000000000").unwrap())),
//trueusd 18
(Address::from_str("0000000000085d4780B73119b644AE5ecd22b376").unwrap(), (U256::from(10100),U256::from_dec_str("1000000000000000000").unwrap())),
//ghostblade-inu 9
(Address::from_str("54b8e638aa2c7a6040f2820f8118237a7bfa0c0d").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//chartex 18
(Address::from_str("1d37986f252d0e349522ea6c3b98cb935495e63e").unwrap(), (U256::from(85),U256::from_dec_str("1000000000000000000").unwrap())),
//peakdefi 8
(Address::from_str("630d98424efe0ea27fb1b3ab7741907dffeaad78").unwrap(), (U256::from(1077),U256::from_dec_str("100000000").unwrap())),
//duck-dao 18
(Address::from_str("c0ba369c8db6eb3924965e5c4fd0b4c1b91e305f").unwrap(), (U256::from(2097),U256::from_dec_str("1000000000000000000").unwrap())),
//add-xyz 18
(Address::from_str("635d081fd8f6670135d8a3640e2cf78220787d56").unwrap(), (U256::from(2038),U256::from_dec_str("1000000000000000000").unwrap())),
//nyan-v2 18
(Address::from_str("bf4a9a37ecfc21825011285222c36ab35de51f14").unwrap(), (U256::from(281800),U256::from_dec_str("1000000000000000000").unwrap())),
//world-token 18
(Address::from_str("bf494f02ee3fde1f20bee6242bce2d1ed0c15e47").unwrap(), (U256::from(157),U256::from_dec_str("1000000000000000000").unwrap())),
//bnsd-finance 18
(Address::from_str("668DbF100635f593A3847c0bDaF21f0a09380188").unwrap(), (U256::from(107),U256::from_dec_str("1000000000000000000").unwrap())),
//non-fungible-yearn 18
(Address::from_str("1cbb83ebcd552d5ebf8131ef8c9cd9d9bab342bc").unwrap(), (U256::from(126400),U256::from_dec_str("1000000000000000000").unwrap())),
//wolfystreetbets 9
(Address::from_str("7dbbcae15d4db168e01673400d7844870cc1e36f").unwrap(), (U256::from(13),U256::from_dec_str("1000000000").unwrap())),
//doyourtip 18
(Address::from_str("740623d2c797b7D8D1EcB98e9b4Afcf99Ec31E14").unwrap(), (U256::from(2169),U256::from_dec_str("1000000000000000000").unwrap())),
//island-doges 9
(Address::from_str("a0dc5132c91ea4d94fcf1727c32cc5a303b34cfc").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//fetch 18
(Address::from_str("aea46A60368A7bD060eec7DF8CBa43b7EF41Ad85").unwrap(), (U256::from(5068),U256::from_dec_str("1000000000000000000").unwrap())),
//rogue-doge 9
(Address::from_str("45734927fa2f616fbe19e65f42a0ef3d37d1c80a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//polinate 18
(Address::from_str("a1a36d3537bbe375cc9694795f663ddc8d516db9").unwrap(), (U256::from(105),U256::from_dec_str("1000000000000000000").unwrap())),
//edge 18
(Address::from_str("4ec1b60b96193a64acae44778e51f7bff2007831").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//floki-pup 9
(Address::from_str("259fba5ae8b626483e1e589e8d60a5413a2157d2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//wiva 18
(Address::from_str("a00055e6ee4d1f4169096ecb682f70caa8c29987").unwrap(), (U256::from(497),U256::from_dec_str("1000000000000000000").unwrap())),
//tycoon 18
(Address::from_str("3A82D3111aB5faF39d847D46023d9090261A658F").unwrap(), (U256::from(390),U256::from_dec_str("1000000000000000000").unwrap())),
//hotbit-token 18
(Address::from_str("6be61833fc4381990e82d7d4a9f4c9b3f67ea941").unwrap(), (U256::from(446),U256::from_dec_str("1000000000000000000").unwrap())),
//playdapp 18
(Address::from_str("3a4f40631a4f906c2BaD353Ed06De7A5D3fCb430").unwrap(), (U256::from(12183),U256::from_dec_str("1000000000000000000").unwrap())),
//auric-network 18
(Address::from_str("1c7bbadc81e18f7177a95eb1593e5f5f35861b10").unwrap(), (U256::from(117),U256::from_dec_str("1000000000000000000").unwrap())),
//xyo 18
(Address::from_str("55296f69f40ea6d20e478533c15a6b08b654e758").unwrap(), (U256::from(296),U256::from_dec_str("1000000000000000000").unwrap())),
//murall 18
(Address::from_str("4c6ec08cf3fc987c6c4beb03184d335a2dfc4042").unwrap(), (U256::from(2),U256::from_dec_str("1000000000000000000").unwrap())),
//katalyo 18
(Address::from_str("24E3794605C84E580EEA4972738D633E8a7127c8").unwrap(), (U256::from(914),U256::from_dec_str("1000000000000000000").unwrap())),
//solomon-defi 18
(Address::from_str("07a0ad7a9dfc3854466f8f29a173bf04bba5686e").unwrap(), (U256::from(313),U256::from_dec_str("1000000000000000000").unwrap())),
//topbidder 18
(Address::from_str("00000000000045166c45af0fc6e4cf31d9e14b9a").unwrap(), (U256::from(4954),U256::from_dec_str("1000000000000000000").unwrap())),
//women-empowerment-token 18
(Address::from_str("79E52C8D2cA6Ad34791899fCD19752A8bc51DEa5").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//evolution-finance 18
(Address::from_str("9af15d7b8776fa296019979e70a5be53c714a7ec").unwrap(), (U256::from(501453),U256::from_dec_str("1000000000000000000").unwrap())),
//cryptokek 18
(Address::from_str("3fa400483487A489EC9b1dB29C4129063EEC4654").unwrap(), (U256::from(334),U256::from_dec_str("1000000000000000000").unwrap())),
//balpha 18
(Address::from_str("7a5ce6abD131EA6B148a022CB76fc180ae3315A6").unwrap(), (U256::from(222134),U256::from_dec_str("1000000000000000000").unwrap())),
//2crazynft 18
(Address::from_str("2c9c19ce3b15ae77c6d80aec3c1194cfd6f7f3fa").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//pika 18
(Address::from_str("60F5672A271C7E39E787427A18353ba59A4A3578").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//spacechain 18
(Address::from_str("86ed939b500e121c0c5f493f399084db596dad20").unwrap(), (U256::from(119),U256::from_dec_str("1000000000000000000").unwrap())),
//stater 18
(Address::from_str("84bb947fcedba6b9c7dcead42df07e113bb03007").unwrap(), (U256::from(445),U256::from_dec_str("1000000000000000000").unwrap())),
//idle 18
(Address::from_str("875773784Af8135eA0ef43b5a374AaD105c5D39e").unwrap(), (U256::from(17987),U256::from_dec_str("1000000000000000000").unwrap())),
//litentry 18
(Address::from_str("b59490ab09a0f526cc7305822ac65f2ab12f9723").unwrap(), (U256::from(26100),U256::from_dec_str("1000000000000000000").unwrap())),
//torn 18
(Address::from_str("77777feddddffc19ff86db637967013e6c6a116c").unwrap(), (U256::from(300000),U256::from_dec_str("1000000000000000000").unwrap())),
//spaceswap 18
(Address::from_str("80c8c3dcfb854f9542567c8dac3f44d709ebc1de").unwrap(), (U256::from(1447),U256::from_dec_str("1000000000000000000").unwrap())),
//senator-karen 9
(Address::from_str("2881080650b782a48b03a1f5bd30df117b6a5bd5").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//sentiment-token 18
(Address::from_str("97abee33cd075c58bfdd174e0885e08e8f03556f").unwrap(), (U256::from(445),U256::from_dec_str("1000000000000000000").unwrap())),
//badger-dao 18
(Address::from_str("3472a5a71965499acd81997a54bba8d852c6e53d").unwrap(), (U256::from(125317),U256::from_dec_str("1000000000000000000").unwrap())),
//peoples-punk 18
(Address::from_str("8ca9a0fbd8db501f013f2e9e33a1b9dc129a48e0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//pinkslip-finance 18
(Address::from_str("36ce7a52cda404b8fa87a98d0d17ec7dd0b144ed").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//humans-ai 18
(Address::from_str("8FAc8031e079F409135766C7d5De29cf22EF897C").unwrap(), (U256::from(1829),U256::from_dec_str("1000000000000000000").unwrap())),
//compound-dai 8
(Address::from_str("5d3a536e4d6dbd6114cc1ead35777bab948e3643").unwrap(), (U256::from(221),U256::from_dec_str("100000000").unwrap())),
//arpa-chain 18
(Address::from_str("ba50933c268f567bdc86e1ac131be072c6b0b71a").unwrap(), (U256::from(875),U256::from_dec_str("1000000000000000000").unwrap())),
//luxfi 18
(Address::from_str("a799c4adcf62e025ce4d8abe6a77cebc487d772a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//nms-token 18
(Address::from_str("77252494C25444F8598A0c74Ffc90ADc535291a9").unwrap(), (U256::from(56),U256::from_dec_str("1000000000000000000").unwrap())),
//unisocks 18
(Address::from_str("23b608675a2b2fb1890d3abbd85c5775c51691d5").unwrap(), (U256::from(715300000),U256::from_dec_str("1000000000000000000").unwrap())),
//softbtc 9
(Address::from_str("309013d55fb0e8c17363bcc79f25d92f711a5802").unwrap(), (U256::from(191),U256::from_dec_str("1000000000").unwrap())),
//hypersign-identity 18
(Address::from_str("b14ebf566511b9e6002bb286016ab2497b9b9c9d").unwrap(), (U256::from(901),U256::from_dec_str("1000000000000000000").unwrap())),
//gamyfi-platform 18
(Address::from_str("65ad6a2288b2dd23e466226397c8f5d1794e58fc").unwrap(), (U256::from(6835),U256::from_dec_str("1000000000000000000").unwrap())),
//chow-inu 18
(Address::from_str("7ad8bc51c917076e5652954943cf0a9991e5a9f9").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//yearn-finance-ii 18
(Address::from_str("a1d0E215a23d7030842FC67cE582a6aFa3CCaB83").unwrap(), (U256::from(26459968),U256::from_dec_str("1000000000000000000").unwrap())),
//labs-group 18
(Address::from_str("8b0e42f366ba502d787bb134478adfae966c8798").unwrap(), (U256::from(47),U256::from_dec_str("1000000000000000000").unwrap())),
//creator-platform 18
(Address::from_str("923b83c26B3809d960fF80332Ed00aA46D7Ed375").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//xtoken 18
(Address::from_str("7f3edcdd180dbe4819bd98fee8929b5cedb3adeb").unwrap(), (U256::from(299),U256::from_dec_str("1000000000000000000").unwrap())),
//solve 8
(Address::from_str("446c9033e7516d820cc9a2ce2d0b7328b579406f").unwrap(), (U256::from(1120),U256::from_dec_str("100000000").unwrap())),
//kwikswap 18
(Address::from_str("286c0936c7eaf6651099ab5dab9ee5a6cb5d229d").unwrap(), (U256::from(251),U256::from_dec_str("1000000000000000000").unwrap())),
//mochi-inu 18
(Address::from_str("60ef10edff6d600cd91caeca04caed2a2e605fe5").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//wild-credit 18
(Address::from_str("08a75dbc7167714ceac1a8e43a8d643a4edd625a").unwrap(), (U256::from(2342),U256::from_dec_str("1000000000000000000").unwrap())),
//mini-saitama 9
(Address::from_str("0c3685559af6f3d20c501b1076a8056a0a14426a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//paypolitan-token 18
(Address::from_str("72630b1e3b42874bf335020ba0249e3e9e47bafc").unwrap(), (U256::from(735),U256::from_dec_str("1000000000000000000").unwrap())),
//earthfund 18
(Address::from_str("9e04f519b094f5f8210441e285f603f4d2b50084").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//scaleswap 18
(Address::from_str("1FbD3dF007eB8A7477A1Eab2c63483dCc24EfFD6").unwrap(), (U256::from(1637),U256::from_dec_str("1000000000000000000").unwrap())),
//communifty 18
(Address::from_str("8e2b4badac15a4ec8c56020f4ce60faa7558c052").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//rougecoin 18
(Address::from_str("a1c7d450130bb77c6a23ddfaecbc4a060215384b").unwrap(), (U256::from(2),U256::from_dec_str("1000000000000000000").unwrap())),
//kittenfinance 18
(Address::from_str("177ba0cac51bfc7ea24bad39d81dcefd59d74faa").unwrap(), (U256::from(281867),U256::from_dec_str("1000000000000000000").unwrap())),
//society-of-galactic-exploration 9
(Address::from_str("ab456bdb0a373bbac6c4a76176e9f159cacd5752").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//axia-protocol 18
(Address::from_str("793786e2dd4Cc492ed366a94B88a3Ff9ba5E7546").unwrap(), (U256::from(3640),U256::from_dec_str("1000000000000000000").unwrap())),
//dehive 18
(Address::from_str("62Dc4817588d53a056cBbD18231d91ffCcd34b2A").unwrap(), (U256::from(5485),U256::from_dec_str("1000000000000000000").unwrap())),
//anatha 18
(Address::from_str("3383c5a8969Dc413bfdDc9656Eb80A1408E4bA20").unwrap(), (U256::from(936),U256::from_dec_str("1000000000000000000").unwrap())),
//hero-inu 9
(Address::from_str("97bFC1700bAF347659b525336B967AA375c05b01").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//satoru-inu 9
(Address::from_str("af6f6abf18d2cc611921e6a683164efaa9165b43").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//daddybezos 9
(Address::from_str("bf825207c74b6c3c01ab807c4f4a4fce26ebdf0f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//1millionnfts 18
(Address::from_str("a4ef4b0b23c1fc81d3f9ecf93510e64f58a4a016").unwrap(), (U256::from(7943),U256::from_dec_str("1000000000000000000").unwrap())),
//shih-tzu 18
(Address::from_str("841fb148863454a3b3570f515414759be9091465").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//collateral-pay 18
(Address::from_str("957891c11616d3e0b0a76a76fb42724c382e0ef3").unwrap(), (U256::from(765),U256::from_dec_str("1000000000000000000").unwrap())),
//cryptonovae 18
(Address::from_str("4ee438be38f8682abb089f2bfea48851c5e71eaf").unwrap(), (U256::from(610),U256::from_dec_str("1000000000000000000").unwrap())),
//sav3token 18
(Address::from_str("6e10aacb89a28d6fa0fe68790777fec7e7f01890").unwrap(), (U256::from(485),U256::from_dec_str("1000000000000000000").unwrap())),
//shake 18
(Address::from_str("6006FC2a849fEdABa8330ce36F5133DE01F96189").unwrap(), (U256::from(8265141),U256::from_dec_str("1000000000000000000").unwrap())),
//nsure-network 18
(Address::from_str("20945cA1df56D237fD40036d47E866C7DcCD2114").unwrap(), (U256::from(838),U256::from_dec_str("1000000000000000000").unwrap())),
//munch-token 9
(Address::from_str("944eee930933be5e23b690c8589021ec8619a301").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//demodyfi 18
(Address::from_str("5f6c5c2fb289db2228d159c69621215e354218d7").unwrap(), (U256::from(867),U256::from_dec_str("1000000000000000000").unwrap())),
//soliditylabs 9
(Address::from_str("368dd0d9a2e595a7a617c3768cdb9a464e06ea69").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//ibeth 18
(Address::from_str("67B66C99D3Eb37Fa76Aa3Ed1ff33E8e39F0b9c7A").unwrap(), (U256::from(34965209),U256::from_dec_str("1000000000000000000").unwrap())),
//froge-finance 9
(Address::from_str("29502fe4d233ef0b45c3647101fa1252ce0634bd").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//energy-ledger 18
(Address::from_str("9048c33c7bae0bbe9ad702b17b4453a83900d154").unwrap(), (U256::from(133),U256::from_dec_str("1000000000000000000").unwrap())),
//coinburp 18
(Address::from_str("33f391f4c4fe802b70b77ae37670037a92114a7c").unwrap(), (U256::from(483),U256::from_dec_str("1000000000000000000").unwrap())),
//boringdao-new 18
(Address::from_str("bc19712feb3a26080ebf6f2f7849b417fdd792ca").unwrap(), (U256::from(420),U256::from_dec_str("1000000000000000000").unwrap())),
//digible 18
(Address::from_str("3CbF23c081fAA5419810ce0F6BC1ECb73006d848").unwrap(), (U256::from(297),U256::from_dec_str("1000000000000000000").unwrap())),
//jarvis-network 18
(Address::from_str("8a9c67fee641579deba04928c4bc45f66e26343a").unwrap(), (U256::from(833),U256::from_dec_str("1000000000000000000").unwrap())),
//atomic-wallet-coin 8
(Address::from_str("ad22f63404f7305e4713ccbd4f296f34770513f4").unwrap(), (U256::from(10430),U256::from_dec_str("100000000").unwrap())),
//blockchain-monster-hunt 18
(Address::from_str("2BA8349123de45E931a8C8264c332E6e9CF593F9").unwrap(), (U256::from(10100),U256::from_dec_str("1000000000000000000").unwrap())),
//polkarare 18
(Address::from_str("2C2f7e7C5604D162d75641256b80F1Bf6f4dC796").unwrap(), (U256::from(407),U256::from_dec_str("1000000000000000000").unwrap())),
//yvault-lp-ycurve 18
(Address::from_str("5dbcF33D8c2E976c6b560249878e6F1491Bca25c").unwrap(), (U256::from(12139),U256::from_dec_str("1000000000000000000").unwrap())),
//bitbase-token 18
(Address::from_str("32e6c34cd57087abbd59b5a4aecc4cb495924356").unwrap(), (U256::from(5217),U256::from_dec_str("1000000000000000000").unwrap())),
//non-fungible-toke 18
(Address::from_str("98ddc72bd02d448f68c4226f26122c66c5bd711e").unwrap(), (U256::from(13),U256::from_dec_str("1000000000000000000").unwrap())),
//digital-reserve-currency 0
(Address::from_str("a150Db9b1Fa65b44799d4dD949D922c0a33Ee606").unwrap(), (U256::from(7),U256::from_dec_str("1").unwrap())),
//centaur 18
(Address::from_str("03042482d64577a7bdb282260e2ea4c8a89c064b").unwrap(), (U256::from(10),U256::from_dec_str("1000000000000000000").unwrap())),
//goji-crypto 12
(Address::from_str("72e5390edb7727e3d4e3436451dadaff675dbcc0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000").unwrap())),
//tranche-finance 18
(Address::from_str("0aee8703d34dd9ae107386d3eff22ae75dd616d1").unwrap(), (U256::from(6115),U256::from_dec_str("1000000000000000000").unwrap())),
//relevant 18
(Address::from_str("b6c4267c4877bb0d6b1685cfd85b0fbe82f105ec").unwrap(), (U256::from(6299),U256::from_dec_str("1000000000000000000").unwrap())),
//kelvpn 18
(Address::from_str("4abb9cc67bd3da9eb966d1159a71a0e68bd15432").unwrap(), (U256::from(65),U256::from_dec_str("1000000000000000000").unwrap())),
//equalizer 18
(Address::from_str("1Da87b114f35E1DC91F72bF57fc07A768Ad40Bb0").unwrap(), (U256::from(1877),U256::from_dec_str("1000000000000000000").unwrap())),
//cirus-foundation 18
(Address::from_str("a01199c61841fce3b3dafb83fefc1899715c8756").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//dctdao 18
(Address::from_str("b566e883555aebf5b1db211070b530ab00a4b18a").unwrap(), (U256::from(1180),U256::from_dec_str("1000000000000000000").unwrap())),
//yup-token 18
(Address::from_str("69bBC3F8787d573F1BBDd0a5f40C7bA0Aee9BCC9").unwrap(), (U256::from(5910),U256::from_dec_str("1000000000000000000").unwrap())),
//delta-theta 18
(Address::from_str("0000000de40dfa9b17854cbc7869d80f9f98d823").unwrap(), (U256::from(665),U256::from_dec_str("1000000000000000000").unwrap())),
//open-governance-token 18
(Address::from_str("69e8b9528CABDA89fe846C67675B5D73d463a916").unwrap(), (U256::from(857),U256::from_dec_str("1000000000000000000").unwrap())),
//juggernaut 18
(Address::from_str("73374ea518de7addd4c2b624c0e8b113955ee041").unwrap(), (U256::from(4365),U256::from_dec_str("1000000000000000000").unwrap())),
//te-food 18
(Address::from_str("2ab6bb8408ca3199b8fa6c92d5b455f820af03c4").unwrap(), (U256::from(225),U256::from_dec_str("1000000000000000000").unwrap())),
//doki-doki-finance 18
(Address::from_str("9ceb84f92a0561fa3cc4132ab9c0b76a59787544").unwrap(), (U256::from(291975),U256::from_dec_str("1000000000000000000").unwrap())),
//imperial-obelisk 9
(Address::from_str("42a0d24cb5c423eaaf926ce3984aaff0c4ff6fe2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//keanu-inu 9
(Address::from_str("106552c11272420aad5d7e94f8acab9095a6c952").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//wallfair 18
(Address::from_str("C6065B9fc8171Ad3D29bad510709249681758972").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//vanilla-network 18
(Address::from_str("B97FaF860045483E0C7F08c56acb31333084a988").unwrap(), (U256::from(54821),U256::from_dec_str("1000000000000000000").unwrap())),
//clintex-cti 18
(Address::from_str("8c18d6a985ef69744b9d57248a45c0861874f244").unwrap(), (U256::from(428),U256::from_dec_str("1000000000000000000").unwrap())),
//connectico 18
(Address::from_str("40d2025ed2e89632d3a41d8541df9ed2ac0e2b1c").unwrap(), (U256::from(1999),U256::from_dec_str("1000000000000000000").unwrap())),
//defipie 18
(Address::from_str("607C794cDa77efB21F8848B7910ecf27451Ae842").unwrap(), (U256::from(137),U256::from_dec_str("1000000000000000000").unwrap())),
//octofi 18
(Address::from_str("7240aC91f01233BaAf8b064248E80feaA5912BA3").unwrap(), (U256::from(51914),U256::from_dec_str("1000000000000000000").unwrap())),
//base-protocol 9
(Address::from_str("07150e919b4de5fd6a63de1f9384828396f25fdc").unwrap(), (U256::from(13783),U256::from_dec_str("1000000000").unwrap())),
//dsla-protocol 18
(Address::from_str("3affcca64c2a6f4e3b6bd9c64cd2c969efd1ecbe").unwrap(), (U256::from(40),U256::from_dec_str("1000000000000000000").unwrap())),
//nftfy 18
(Address::from_str("bf6ff49ffd3d104302ef0ab0f10f5a84324c091c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//hoff-coin 8
(Address::from_str("b3f822dbbd694901e2051a2495a8755d6cfd5133").unwrap(), (U256::from(0),U256::from_dec_str("100000000").unwrap())),
//jindoge 18
(Address::from_str("3f4cd830543db25254ec0f05eac058d4d6e86166").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//hakka-finance 18
(Address::from_str("0e29e5abbb5fd88e28b2d355774e73bd47de3bcd").unwrap(), (U256::from(161),U256::from_dec_str("1000000000000000000").unwrap())),
//mm-token 18
(Address::from_str("a283aa7cfbb27ef0cfbcb2493dd9f4330e0fd304").unwrap(), (U256::from(18100),U256::from_dec_str("1000000000000000000").unwrap())),
//salt 8
(Address::from_str("4156D3342D5c385a87D264F90653733592000581").unwrap(), (U256::from(1052),U256::from_dec_str("100000000").unwrap())),
//status 18
(Address::from_str("744d70fdbe2ba4cf95131626614a1763df805b9e").unwrap(), (U256::from(635),U256::from_dec_str("1000000000000000000").unwrap())),
//hina-inu 9
(Address::from_str("bd0a4bf098261673d5e6e600fd87ddcd756e6764").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//kakashiinuv2 9
(Address::from_str("15a6d1392188cc1fc1d99936e7d3c09e28c21465").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//moar-finance 18
(Address::from_str("187eff9690e1f1a61d578c7c492296eaab82701a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//morpher 18
(Address::from_str("6369c3dadfc00054a42ba8b2c09c48131dd4aa38").unwrap(), (U256::from(230),U256::from_dec_str("1000000000000000000").unwrap())),
//moontools 18
(Address::from_str("260e63d91fCCC499606BAe3FE945c4ed1CF56A56").unwrap(), (U256::from(141264),U256::from_dec_str("1000000000000000000").unwrap())),
//fat-doge 9
(Address::from_str("76851a93977bea9264c32255b6457882035c7501").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//ethbox 18
(Address::from_str("33840024177a7daca3468912363bed8b425015c5").unwrap(), (U256::from(357),U256::from_dec_str("1000000000000000000").unwrap())),
//polkainsure-finance 18
(Address::from_str("834ce7ad163ab3be0c5fd4e0a81e67ac8f51e00c").unwrap(), (U256::from(65600),U256::from_dec_str("1000000000000000000").unwrap())),
//definity 18
(Address::from_str("5F474906637bdCDA05f29C74653F6962bb0f8eDa").unwrap(), (U256::from(278),U256::from_dec_str("1000000000000000000").unwrap())),
//iconic-token 18
(Address::from_str("b3e2cb7cccfe139f8ff84013823bf22da6b6390a").unwrap(), (U256::from(3733),U256::from_dec_str("1000000000000000000").unwrap())),
//derivadao 18
(Address::from_str("3a880652f47bfaa771908c07dd8673a787daed3a").unwrap(), (U256::from(39200),U256::from_dec_str("1000000000000000000").unwrap())),
//cometh 18
(Address::from_str("9c78ee466d6cb57a4d01fd887d2b5dfb2d46288f").unwrap(), (U256::from(659000),U256::from_dec_str("1000000000000000000").unwrap())),
//truefi-token 8
(Address::from_str("4c19596f5aaff459fa38b0f7ed92f11ae6543784").unwrap(), (U256::from(2703),U256::from_dec_str("100000000").unwrap())),
//xsigma 18
(Address::from_str("7777777777697cfeecf846a76326da79cc606517").unwrap(), (U256::from(918),U256::from_dec_str("1000000000000000000").unwrap())),
//polkapets 18
(Address::from_str("6afcff9189e8ed3fcc1cffa184feb1276f6a82a5").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//pupper 18
(Address::from_str("81dBc1c8e40C3095071949Eda9800C2209a7279A").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//metarcade 18
(Address::from_str("b120b0b309f6ee56b67a7a6af216ab2fe56c3ed2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//lympo 18
(Address::from_str("c690f7c7fcffa6a82b79fab7508c466fefdfc8c5").unwrap(), (U256::from(85),U256::from_dec_str("1000000000000000000").unwrap())),
//nft-tech 18
(Address::from_str("5fA2E9Ba5757504B3d6e8f6da03cc40d4ce19499").unwrap(), (U256::from(651),U256::from_dec_str("1000000000000000000").unwrap())),
//smartcredit-token 18
(Address::from_str("72e9d9038ce484ee986fea183f8d8df93f9ada13").unwrap(), (U256::from(21676),U256::from_dec_str("1000000000000000000").unwrap())),
//keisuke-inu 9
(Address::from_str("c0114f14638a333a4d5c3b04f09b96372348a842").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//pundix-new 18
(Address::from_str("a15c7ebe1f07caf6bff097d8a589fb8ac49ae5b3").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//dragonbite 18
(Address::from_str("4eed0fa8de12d5a86517f214c2f11586ba2ed88d").unwrap(), (U256::from(18),U256::from_dec_str("1000000000000000000").unwrap())),
//meta-shiba 18
(Address::from_str("9cF77be84214beb066F26a4ea1c38ddcc2AFbcf7").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//bnktothefuture 18
(Address::from_str("01ff50f8b7f74e4f00580d9596cd3d0d6d6e326f").unwrap(), (U256::from(112),U256::from_dec_str("1000000000000000000").unwrap())),
//shinjutsu 9
(Address::from_str("6e6c6b24371d2ee18fc39b4bc534b4344d2bbd61").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//blockv 18
(Address::from_str("340d2bde5eb28c1eed91b2f790723e3b160613b7").unwrap(), (U256::from(78),U256::from_dec_str("1000000000000000000").unwrap())),
//hayate-inu 18
(Address::from_str("903aed40b7fcbe8de84a699151c9055f4c0a6db3").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//axioms 18
(Address::from_str("73ee6d7e6b203125add89320e9f343d65ec7c39a").unwrap(), (U256::from(343),U256::from_dec_str("1000000000000000000").unwrap())),
//bittoken 18
(Address::from_str("9f9913853f749b3fe6d6d4e16a1cc3c1656b6d51").unwrap(), (U256::from(1049),U256::from_dec_str("1000000000000000000").unwrap())),
//chopper-inu 9
(Address::from_str("28c5805b64d163588a909012a628b5a03c1041f9").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//unreal-finance 18
(Address::from_str("9cf98eb8a8b28c83e8612046cf55701ce3eb0063").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//multiplier 8
(Address::from_str("8a6f3bf52a26a21531514e23016eeae8ba7e7018").unwrap(), (U256::from(80),U256::from_dec_str("100000000").unwrap())),
//governor-dao 18
(Address::from_str("515d7e9d75e2b76db60f8a051cd890eba23286bc").unwrap(), (U256::from(7691),U256::from_dec_str("1000000000000000000").unwrap())),
//crypto-excellence 18
(Address::from_str("8f12dfc7981de79a8a34070a732471f2d335eece").unwrap(), (U256::from(40100),U256::from_dec_str("1000000000000000000").unwrap())),
//statera 18
(Address::from_str("a7DE087329BFcda5639247F96140f9DAbe3DeED1").unwrap(), (U256::from(291),U256::from_dec_str("1000000000000000000").unwrap())),
//prostarter 18
(Address::from_str("2341dd0a96a0dab62aa1efb93d59ff7f3bdb8932").unwrap(), (U256::from(528),U256::from_dec_str("1000000000000000000").unwrap())),
//sewer-rat-social-club-chiz-token 18
(Address::from_str("5c761c1a21637362374204000e383204d347064c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//parachute 18
(Address::from_str("1beef31946fbbb40b877a72e4ae04a8d1a5cee06").unwrap(), (U256::from(22),U256::from_dec_str("1000000000000000000").unwrap())),
//cash-tech 18
(Address::from_str("a42f266684ac2ad6ecb00df95b1c76efbb6f136c").unwrap(), (U256::from(44),U256::from_dec_str("1000000000000000000").unwrap())),
//safe-shield 9
(Address::from_str("11a605d7e12b64d713e93c487277d819a1d14b99").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//hedget 6
(Address::from_str("7968bc6a03017ea2de509aaa816f163db0f35148").unwrap(), (U256::from(24600),U256::from_dec_str("1000000").unwrap())),
//piedao-dough-v2 18
(Address::from_str("ad32A8e6220741182940c5aBF610bDE99E737b2D").unwrap(), (U256::from(3549),U256::from_dec_str("1000000000000000000").unwrap())),
//sync-network 18
(Address::from_str("b6ff96b8a8d214544ca0dbc9b33f7ad6503efd32").unwrap(), (U256::from(123),U256::from_dec_str("1000000000000000000").unwrap())),
//bitgear 18
(Address::from_str("1b980e05943dE3dB3a459C72325338d327B6F5a9").unwrap(), (U256::from(149),U256::from_dec_str("1000000000000000000").unwrap())),
//earnbase 18
(Address::from_str("a6fb1df483b24eeab569e19447e0e107003b9e15").unwrap(), (U256::from(10743),U256::from_dec_str("1000000000000000000").unwrap())),
//mysterium 18
(Address::from_str("4Cf89ca06ad997bC732Dc876ed2A7F26a9E7f361").unwrap(), (U256::from(4351),U256::from_dec_str("1000000000000000000").unwrap())),
//infinity-pad 18
(Address::from_str("36ed7baad9a571b5dad55d096c0ed902188d6d3c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//liti-capital 18
(Address::from_str("0b63128c40737b13647552e0c926bcfeccc35f93").unwrap(), (U256::from(86),U256::from_dec_str("1000000000000000000").unwrap())),
//quantstamp 18
(Address::from_str("99ea4db9ee77acd40b119bd1dc4e33e1c070b80d").unwrap(), (U256::from(350),U256::from_dec_str("1000000000000000000").unwrap())),
//renascent-finance 18
(Address::from_str("56de8bc61346321d4f2211e3ac3c0a7f00db9b76").unwrap(), (U256::from(4557),U256::from_dec_str("1000000000000000000").unwrap())),
//evidenz 18
(Address::from_str("acfa209fb73bf3dd5bbfb1101b9bc999c49062a5").unwrap(), (U256::from(1414),U256::from_dec_str("1000000000000000000").unwrap())),
//degen-arts 18
(Address::from_str("8281ee37f164c0e26e6b6f87e7695baac256df07").unwrap(), (U256::from(39500),U256::from_dec_str("1000000000000000000").unwrap())),
//american-shiba 9
(Address::from_str("b893a8049f250b57efa8c62d51527a22404d7c9a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//finminity 18
(Address::from_str("99c6e435eC259A7E8d65E1955C9423DB624bA54C").unwrap(), (U256::from(1030),U256::from_dec_str("1000000000000000000").unwrap())),
//rogue-west 8
(Address::from_str("6ac665c0de9a6ca72b85757b141aa9c428828aca").unwrap(), (U256::from(0),U256::from_dec_str("100000000").unwrap())),
//electric-vehicle-direct-currency 18
(Address::from_str("704eae6d452ca63ce479c59727177c5f3ba0d90c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//ydragon 18
(Address::from_str("3757232b55e60da4a8793183ac030cfce4c3865d").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//wolves-of-wall-street 18
(Address::from_str("672EF7E4Fe230B5cA1466C5fDD40588d30FdF90a").unwrap(), (U256::from(316000),U256::from_dec_str("1000000000000000000").unwrap())),
//labracoin 9
(Address::from_str("106d3c66d22d2dd0446df23d7f5960752994d600").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//int-chain 18
(Address::from_str("be038a2fdfec62cf1bed852f141a43005035edcc").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//follow-token 18
(Address::from_str("b2a63a5dd36c91ec2da59b188ff047f66fac122a").unwrap(), (U256::from(140),U256::from_dec_str("1000000000000000000").unwrap())),
//phoenixdao 18
(Address::from_str("38A2fDc11f526Ddd5a607C1F251C065f40fBF2f7").unwrap(), (U256::from(446),U256::from_dec_str("1000000000000000000").unwrap())),
//chihua-token 18
(Address::from_str("26ff6d16549a00ba8b36ce3159b5277e6e798d18").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//tails 9
(Address::from_str("3d79abb948bc76794ff4a0bcd60170a741f26360").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//spherium 18
(Address::from_str("8a0cdfab62ed35b836dc0633482798421c81b3ec").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//cxn-network 18
(Address::from_str("b48E0F69e6A3064f5498D495F77AD83e0874ab28").unwrap(), (U256::from(10),U256::from_dec_str("1000000000000000000").unwrap())),
//power-ledger 6
(Address::from_str("595832f8fc6bf59c85c527fec3740a1b7a361269").unwrap(), (U256::from(5340),U256::from_dec_str("1000000").unwrap())),
//bartertrade 18
(Address::from_str("54c9ea2e9c9e8ed865db4a4ce6711c2a0d5063ba").unwrap(), (U256::from(38),U256::from_dec_str("1000000000000000000").unwrap())),
//unicat 9
(Address::from_str("87c0192b1b81b9550d495558aac9753972f6db0d").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//shibaken-finance 0
(Address::from_str("a4cf2afd3b165975afffbf7e487cdd40c894ab6b").unwrap(), (U256::from(0),U256::from_dec_str("1").unwrap())),
//meme-inu 18
(Address::from_str("74b988156925937bd4e082f0ed7429da8eaea8db").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//justbet 18
(Address::from_str("27460aac4b005de72e2326bd8391c27fb41780f8").unwrap(), (U256::from(2),U256::from_dec_str("1000000000000000000").unwrap())),
//vox-finance 18
(Address::from_str("12D102F06da35cC0111EB58017fd2Cd28537d0e1").unwrap(), (U256::from(98005),U256::from_dec_str("1000000000000000000").unwrap())),
//whiteheart 18
(Address::from_str("5f0e628b693018f639d10e4a4f59bd4d8b2b6b44").unwrap(), (U256::from(4119500),U256::from_dec_str("1000000000000000000").unwrap())),
//swarm-network 18
(Address::from_str("3505f494c3f0fed0b594e01fa41dd3967645ca39").unwrap(), (U256::from(167),U256::from_dec_str("1000000000000000000").unwrap())),
//acoconut 18
(Address::from_str("9A0aBA393aac4dFbFf4333B06c407458002C6183").unwrap(), (U256::from(1759),U256::from_dec_str("1000000000000000000").unwrap())),
//swftcoin 8
(Address::from_str("0bb217E40F8a5Cb79Adf04E1aAb60E5abd0dfC1e").unwrap(), (U256::from(15),U256::from_dec_str("100000000").unwrap())),
//davincij15-token 9
(Address::from_str("5d269fac3b2e0552b0f34cdc253bdb427682a4b9").unwrap(), (U256::from(1149728),U256::from_dec_str("1000000000").unwrap())),
//baby-floki-doge 9
(Address::from_str("747c4ce9622ea750ea8048423b38a746b096c8e8").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//coshi-inu 9
(Address::from_str("668c50b1c7f46effbe3f242687071d7908aab00a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//nft-stars 18
(Address::from_str("08037036451c768465369431da5c671ad9b37dbc").unwrap(), (U256::from(5988),U256::from_dec_str("1000000000000000000").unwrap())),
//money-party 6
(Address::from_str("314bd765cab4774b2e547eb0aa15013e03ff74d2").unwrap(), (U256::from(11),U256::from_dec_str("1000000").unwrap())),
//first-eleven 18
(Address::from_str("309c1b3282c49E4dC6796644417f8c76b7C8233C").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//jomon-shiba 9
(Address::from_str("1426cC6D52D1B14e2B3b1Cb04d57ea42B39c4c7c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//lelouch-lamperouge 9
(Address::from_str("4546d782ffb14a465a3bb518eecf1a181da85332").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//yfox-finance 6
(Address::from_str("706CB9E741CBFee00Ad5b3f5ACc8bd44D1644a74").unwrap(), (U256::from(158001),U256::from_dec_str("1000000").unwrap())),
//nft-wars 18
(Address::from_str("4d75D9e37667a2d4677Ec3d74bDD9049326Ad8d6").unwrap(), (U256::from(1916),U256::from_dec_str("1000000000000000000").unwrap())),
//scifi-finance 18
(Address::from_str("1fdab294eda5112b7d066ed8f2e4e562d5bcc664").unwrap(), (U256::from(1713),U256::from_dec_str("1000000000000000000").unwrap())),
//mars 18
(Address::from_str("66C0DDEd8433c9EA86C8cf91237B14e10b4d70B7").unwrap(), (U256::from(57),U256::from_dec_str("1000000000000000000").unwrap())),
//kimchi-finance 18
(Address::from_str("1e18821e69b9faa8e6e75dffe54e7e25754beda0").unwrap(), (U256::from(1),U256::from_dec_str("1000000000000000000").unwrap())),
//dao1 18
(Address::from_str("ce3f6f6672616c39d8b6858f8dac9902eca42c84").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//mozik 18
(Address::from_str("7BD82B320EbC28D8EB3C4F5Fa2af7B14dA5b90C3").unwrap(), (U256::from(30),U256::from_dec_str("1000000000000000000").unwrap())),
//pink-panther 18
(Address::from_str("a113b79c09f0794568b8864a24197e0b817041ea").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//combine-finance 18
(Address::from_str("7d36cce46dd2b0d28dde12a859c2ace4a21e3678").unwrap(), (U256::from(366284),U256::from_dec_str("1000000000000000000").unwrap())),
//kangal 18
(Address::from_str("6e765d26388a17a6e86c49a8e41df3f58abcd337").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//low-orbit-crypto-cannon 18
(Address::from_str("556938621C19e5eae58C94a806da9d237b969bd8").unwrap(), (U256::from(3768639),U256::from_dec_str("1000000000000000000").unwrap())),
//megashibox-inu 18
(Address::from_str("0441890a456a61098fe1ee4082c2006a2c2b9330").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//froggies 9
(Address::from_str("7c3ff33c76c919b3f5fddaf7bdddbb20a826dc61").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//small-dogecoin 18
(Address::from_str("537edd52ebcb9f48ff2f8a28c51fcdb9d6a6e0d4").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//deflect 9
(Address::from_str("3aa5f749d4a6bcf67dac1091ceb69d1f5d86fa53").unwrap(), (U256::from(16600),U256::from_dec_str("1000000000").unwrap())),
//bankroll-vault 18
(Address::from_str("6b785a0322126826d8226d77e173d75dafb84d11").unwrap(), (U256::from(2980),U256::from_dec_str("1000000000000000000").unwrap())),
//floki-adventure 9
(Address::from_str("8b23b79ea039cf7242a91f2e3ef88df6f565d1ff").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//cryptotwitter 9
(Address::from_str("2e9cce8c3bf731f9bfc39e3d345a70907f454d40").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//king-arthur 9
(Address::from_str("1ca02dd95f3f1e33da7f5afe15ea866dab07af04").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//elonspets 18
(Address::from_str("40b50a516e081945b95d30fcbbb31476a63ffb4a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//stakerdao 18
(Address::from_str("89dcff5fd892f2bfc8b75dba12804b651f769579").unwrap(), (U256::from(153),U256::from_dec_str("1000000000000000000").unwrap())),
//ledgerscore 18
(Address::from_str("72De803b67B6AB05B61EFab2Efdcd414D16eBF6D").unwrap(), (U256::from(91),U256::from_dec_str("1000000000000000000").unwrap())),
//mochi-market 18
(Address::from_str("bd1848e1491d4308ad18287a745dd4db2a4bd55b").unwrap(), (U256::from(558),U256::from_dec_str("1000000000000000000").unwrap())),
//aloha 18
(Address::from_str("455f7ef6d8bcfc35f9337e85aee1b0600a59fabe").unwrap(), (U256::from(119),U256::from_dec_str("1000000000000000000").unwrap())),
//defiplaza 18
(Address::from_str("2F57430a6ceDA85a67121757785877b4a71b8E6D").unwrap(), (U256::from(936),U256::from_dec_str("1000000000000000000").unwrap())),
//font 18
(Address::from_str("4c25bdf026ea05f32713f00f73ca55857fbf6342").unwrap(), (U256::from(10900),U256::from_dec_str("1000000000000000000").unwrap())),
//cryptotask 18
(Address::from_str("196c81385bc536467433014042788eb707703934").unwrap(), (U256::from(2574),U256::from_dec_str("1000000000000000000").unwrap())),
//kimetsu-inu 9
(Address::from_str("91e8d1b5f386204a82e6de32d4bae11d0b042f0f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//just-ape 9
(Address::from_str("40e0a6ef9dbadfc83c5e0d15262feb4638588d77").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//peerex 18
(Address::from_str("3c6ff50c9ec362efa359317009428d52115fe643").unwrap(), (U256::from(8),U256::from_dec_str("1000000000000000000").unwrap())),
//lunafox 9
(Address::from_str("0924d87605e51764a4620b8c41712a29e9c234c9").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//onooks 18
(Address::from_str("69d9905b2e5f6f5433212b7f3c954433f23c1572").unwrap(), (U256::from(11472),U256::from_dec_str("1000000000000000000").unwrap())),
//zero-utility-token 18
(Address::from_str("83F873388Cd14b83A9f47FabDe3C9850b5C74548").unwrap(), (U256::from(3132100),U256::from_dec_str("1000000000000000000").unwrap())),
//suni 18
(Address::from_str("4a22a69e45ab29f9f7276b0267797474daf1f27c").unwrap(), (U256::from(43),U256::from_dec_str("1000000000000000000").unwrap())),
//n-word-pass 18
(Address::from_str("28b1c08335fc02a82cbf7af850b01b01b9dc34e6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//dimitra-token 18
(Address::from_str("51cB253744189f11241becb29BeDd3F1b5384fdB").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//dether 18
(Address::from_str("5adc961d6ac3f7062d2ea45fefb8d8167d44b190").unwrap(), (U256::from(33),U256::from_dec_str("1000000000000000000").unwrap())),
//monolith 8
(Address::from_str("aaaf91d9b90df800df4f55c205fd6989c977e73a").unwrap(), (U256::from(2000),U256::from_dec_str("100000000").unwrap())),
//rain-network 18
(Address::from_str("61cdb66e56fad942a7b5ce3f419ffe9375e31075").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//unidollar 18
(Address::from_str("256845e721c0c46d54e6afbd4fa3b52cb72353ea").unwrap(), (U256::from(13),U256::from_dec_str("1000000000000000000").unwrap())),
//antiscamtoken 18
(Address::from_str("a872e0a44bbd66c1486a756cb5bd3f0beec4e32e").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//kids-cash 18
(Address::from_str("2c50ba1ed5e4574c1b613b044bd1876f0b0b87a9").unwrap(), (U256::from(1127),U256::from_dec_str("1000000000000000000").unwrap())),
//orchid 18
(Address::from_str("4575f41308EC1483f3d399aa9a2826d74Da13Deb").unwrap(), (U256::from(3267),U256::from_dec_str("1000000000000000000").unwrap())),
//trustdao 18
(Address::from_str("57700244B20f84799a31c6C96DadFF373ca9D6c5").unwrap(), (U256::from(52),U256::from_dec_str("1000000000000000000").unwrap())),
//aludra-network 18
(Address::from_str("b339FcA531367067e98d7c4f9303Ffeadff7B881").unwrap(), (U256::from(9),U256::from_dec_str("1000000000000000000").unwrap())),
//libera 18
(Address::from_str("0bf6261297198d91d4fa460242c69232146a5703").unwrap(), (U256::from(11200),U256::from_dec_str("1000000000000000000").unwrap())),
//meridian-network 18
(Address::from_str("95172ccBe8344fecD73D0a30F54123652981BD6F").unwrap(), (U256::from(147),U256::from_dec_str("1000000000000000000").unwrap())),
//defi-bids 18
(Address::from_str("1dA01e84F3d4e6716F274c987Ae4bEE5DC3C8288").unwrap(), (U256::from(104),U256::from_dec_str("1000000000000000000").unwrap())),
//keysians-network 18
(Address::from_str("6a7ef4998eb9d0f706238756949f311a59e05745").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//yield-stake-finance 18
(Address::from_str("03e4bdce611104289333f35c8177558b04cc99ff").unwrap(), (U256::from(67106),U256::from_dec_str("1000000000000000000").unwrap())),
//xiotri 3
(Address::from_str("31024a4c3e9aeeb256b825790f5cb7ac645e7cd5").unwrap(), (U256::from(964016),U256::from_dec_str("1000").unwrap())),
//lien 8
(Address::from_str("ab37e1358b639fd877f015027bb62d3ddaa7557e").unwrap(), (U256::from(13000),U256::from_dec_str("100000000").unwrap())),
//walnut-finance 18
(Address::from_str("0501e7a02c285b9b520fdbf1badc74ae931ad75d").unwrap(), (U256::from(10887),U256::from_dec_str("1000000000000000000").unwrap())),
//bitto 18
(Address::from_str("55a290f08bb4cae8dcf1ea5635a3fcfd4da60456").unwrap(), (U256::from(959),U256::from_dec_str("1000000000000000000").unwrap())),
//satopay-network 18
(Address::from_str("8c3ee4f778e282b59d42d693a97b80b1ed80f4ee").unwrap(), (U256::from(13),U256::from_dec_str("1000000000000000000").unwrap())),
//swapfolio 18
(Address::from_str("ba21ef4c9f433ede00badefcc2754b8e74bd538a").unwrap(), (U256::from(874),U256::from_dec_str("1000000000000000000").unwrap())),
//seigniorage-shares 9
(Address::from_str("39795344CBCc76cC3Fb94B9D1b15C23c2070C66D").unwrap(), (U256::from(159),U256::from_dec_str("1000000000").unwrap())),
//yfbeta 18
(Address::from_str("89ee58af4871b474c30001982c3d7439c933c838").unwrap(), (U256::from(93297),U256::from_dec_str("1000000000000000000").unwrap())),
//defiat 18
(Address::from_str("b6ee603933e024d8d53dde3faa0bf98fe2a3d6f1").unwrap(), (U256::from(2397),U256::from_dec_str("1000000000000000000").unwrap())),
//geodb 18
(Address::from_str("147faf8de9d8d8daae129b187f0d02d819126750").unwrap(), (U256::from(218),U256::from_dec_str("1000000000000000000").unwrap())),
//coin-artist 18
(Address::from_str("87b008E57F640D94Ee44Fd893F0323AF933F9195").unwrap(), (U256::from(9039),U256::from_dec_str("1000000000000000000").unwrap())),
//the-forms 18
(Address::from_str("8b80596660f007342dc590e5c53bbddd2cd550fb").unwrap(), (U256::from(67),U256::from_dec_str("1000000000000000000").unwrap())),
//foresight 18
(Address::from_str("b1EC548F296270BC96B8A1b3b3C8F3f04b494215").unwrap(), (U256::from(318),U256::from_dec_str("1000000000000000000").unwrap())),
//lead-wallet 18
(Address::from_str("1dd80016e3d4ae146ee2ebb484e8edd92dacc4ce").unwrap(), (U256::from(37),U256::from_dec_str("1000000000000000000").unwrap())),
//blockclout 18
(Address::from_str("a10ae543db5d967a73e9abcc69c81a18a7fc0a78").unwrap(), (U256::from(2),U256::from_dec_str("1000000000000000000").unwrap())),
//fireball 18
(Address::from_str("3F8A2f7bcD70e7F7Bdd3FbB079c11d073588DEA2").unwrap(), (U256::from(300),U256::from_dec_str("1000000000000000000").unwrap())),
//aga 4
(Address::from_str("2d80f5f5328fdcb6eceb7cacf5dd8aedaec94e20").unwrap(), (U256::from(2455),U256::from_dec_str("10000").unwrap())),
//waifu-token 18
(Address::from_str("b2279b6769cfba691416f00609b16244c0cf4b20").unwrap(), (U256::from(10),U256::from_dec_str("1000000000000000000").unwrap())),
//equus-mining-token 18
(Address::from_str("a462d0E6Bb788c7807B1B1C96992CE1f7069E195").unwrap(), (U256::from(16),U256::from_dec_str("1000000000000000000").unwrap())),
//myx-network 18
(Address::from_str("2129fF6000b95A973236020BCd2b2006B0D8E019").unwrap(), (U256::from(15),U256::from_dec_str("1000000000000000000").unwrap())),
//lync-network 18
(Address::from_str("8f87Ec6aAd3B2A8C44f1298A1af56169B8e574cf").unwrap(), (U256::from(4626),U256::from_dec_str("1000000000000000000").unwrap())),
//decraft-finance 18
(Address::from_str("a09ff006c652496e72d648cef2f4ee6777efdf6f").unwrap(), (U256::from(385232),U256::from_dec_str("1000000000000000000").unwrap())),
//insights-network 4
(Address::from_str("8193711b2763bc7dfd67da0d6c8c26642eafdaf3").unwrap(), (U256::from(0),U256::from_dec_str("10000").unwrap())),
//yrise-finance 18
(Address::from_str("6051C1354Ccc51b4d561e43b02735DEaE64768B8").unwrap(), (U256::from(16000),U256::from_dec_str("1000000000000000000").unwrap())),
//fera 18
(Address::from_str("539f3615c1dbafa0d008d87504667458acbd16fa").unwrap(), (U256::from(46),U256::from_dec_str("1000000000000000000").unwrap())),
//degenvc 18
(Address::from_str("26E43759551333e57F073bb0772F50329A957b30").unwrap(), (U256::from(3810),U256::from_dec_str("1000000000000000000").unwrap())),
//zoom-protocol 18
(Address::from_str("1a231e75538a931c395785ef5d1a5581ec622b0e").unwrap(), (U256::from(67340),U256::from_dec_str("1000000000000000000").unwrap())),
//bananodos 18
(Address::from_str("1706c33B9a5B12aeB85B862215378dEe9480EB95").unwrap(), (U256::from(1491517),U256::from_dec_str("1000000000000000000").unwrap())),
//coil 9
(Address::from_str("3936ad01cf109a36489d93cabda11cf062fd3d48").unwrap(), (U256::from(9958),U256::from_dec_str("1000000000").unwrap())),
//xfinance 18
(Address::from_str("5BEfBB272290dD5b8521D4a938f6c4757742c430").unwrap(), (U256::from(720699),U256::from_dec_str("1000000000000000000").unwrap())),
//swag-finance 18
(Address::from_str("87edffde3e14c7a66c9b9724747a1c5696b742e6").unwrap(), (U256::from(177),U256::from_dec_str("1000000000000000000").unwrap())),
//finswap 18
(Address::from_str("3B78dc5736a49BD297Dd2E4d62daA83D35A22749").unwrap(), (U256::from(1095),U256::from_dec_str("1000000000000000000").unwrap())),
//yfe-money 18
(Address::from_str("33811d4edbcaed10a685254eb5d3c4e4398520d2").unwrap(), (U256::from(37800),U256::from_dec_str("1000000000000000000").unwrap())),
//chads-vc 18
(Address::from_str("69692D3345010a207b759a7D1af6fc7F38b35c5E").unwrap(), (U256::from(501),U256::from_dec_str("1000000000000000000").unwrap())),
//bellevue-network 18
(Address::from_str("8DA25B8eD753a5910013167945A676921e864436").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//yfpro-finance 18
(Address::from_str("0fdc5313333533cc0c00c22792bff7383d3055f2").unwrap(), (U256::from(36835),U256::from_dec_str("1000000000000000000").unwrap())),
//tsunami 18
(Address::from_str("7eb4db4dddb16a329c5ade17a8a0178331267e28").unwrap(), (U256::from(1177811),U256::from_dec_str("1000000000000000000").unwrap())),
//yeld-finance 18
(Address::from_str("468ab3b1f63A1C14b361bC367c3cC92277588Da1").unwrap(), (U256::from(47938),U256::from_dec_str("1000000000000000000").unwrap())),
//upbots 18
(Address::from_str("8564653879a18C560E7C0Ea0E084c516C62F5653").unwrap(), (U256::from(224),U256::from_dec_str("1000000000000000000").unwrap())),
//ofin-token 18
(Address::from_str("3b4cAAAF6F3ce5Bee2871C89987cbd825Ac30822").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//tribute 18
(Address::from_str("7031ab87dcc46818806ec07af46fa8c2ad2a2bfc").unwrap(), (U256::from(4556),U256::from_dec_str("1000000000000000000").unwrap())),
//momentum 10
(Address::from_str("9a7a4c141a3bcce4a31e42c1192ac6add35069b4").unwrap(), (U256::from(13),U256::from_dec_str("10000000000").unwrap())),
//ytsla-finance 18
(Address::from_str("5322a3556f979ce2180b30e689a9436fddcb1021").unwrap(), (U256::from(104465),U256::from_dec_str("1000000000000000000").unwrap())),
//payship 18
(Address::from_str("88D59Ba796fDf639dEd3b5E720988D59fDb71Eb8").unwrap(), (U256::from(318600),U256::from_dec_str("1000000000000000000").unwrap())),
//swapship 18
(Address::from_str("3ac2AB91dDF57e2385089202Ca221C360CED0062").unwrap(), (U256::from(46426),U256::from_dec_str("1000000000000000000").unwrap())),
//shill-win 18
(Address::from_str("685aea4F02E39E5a5BB7f7117E88DB1151F38364").unwrap(), (U256::from(1),U256::from_dec_str("1000000000000000000").unwrap())),
//wrapped-leo 3
(Address::from_str("73a9fb46e228628f8f9bb9004eca4f4f529d3998").unwrap(), (U256::from(2092),U256::from_dec_str("1000").unwrap())),
//owl-token-stealthswap 18
(Address::from_str("2a7f709ee001069771ceb6d42e85035f7d18e736").unwrap(), (U256::from(1519),U256::from_dec_str("1000000000000000000").unwrap())),
//dracula-token 18
(Address::from_str("b78B3320493a4EFaa1028130C5Ba26f0B6085Ef8").unwrap(), (U256::from(302),U256::from_dec_str("1000000000000000000").unwrap())),
//tomochain 18
(Address::from_str("05d3606d5c81eb9b7b18530995ec9b29da05faba").unwrap(), (U256::from(16503),U256::from_dec_str("1000000000000000000").unwrap())),
//yearn-finance-ecosystem 8
(Address::from_str("2e6e152d29053b6337e434bc9be17504170f8a5b").unwrap(), (U256::from(0),U256::from_dec_str("100000000").unwrap())),
//quiverx 18
(Address::from_str("6e0dade58d2d89ebbe7afc384e3e4f15b70b14d8").unwrap(), (U256::from(170),U256::from_dec_str("1000000000000000000").unwrap())),
//moonday-finance 18
(Address::from_str("1ad606adde97c0c28bd6ac85554176bc55783c01").unwrap(), (U256::from(874400),U256::from_dec_str("1000000000000000000").unwrap())),
//chonk 18
(Address::from_str("84679bc467DC6c2c40ab04538813AfF3796351f1").unwrap(), (U256::from(211624),U256::from_dec_str("1000000000000000000").unwrap())),
//enoki-finance 18
(Address::from_str("a4bad5d040d4464ec5ce130987731f2f428c9307").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//pria 18
(Address::from_str("b9871cb10738eada636432e86fc0cb920dc3de24").unwrap(), (U256::from(14080),U256::from_dec_str("1000000000000000000").unwrap())),
//neutrino-usd 18
(Address::from_str("674C6Ad92Fd080e4004b2312b45f796a192D27a0").unwrap(), (U256::from(9618),U256::from_dec_str("1000000000000000000").unwrap())),
//sergs 18
(Address::from_str("79BA92DDA26FcE15e1e9af47D5cFdFD2A093E000").unwrap(), (U256::from(1810),U256::from_dec_str("1000000000000000000").unwrap())),
//csp-dao 18
(Address::from_str("7f0c8b125040f707441cad9e5ed8a8408673b455").unwrap(), (U256::from(91500),U256::from_dec_str("1000000000000000000").unwrap())),
//fundamenta 18
(Address::from_str("aa9d866666c2a3748d6b23ff69e63e52f08d9ab4").unwrap(), (U256::from(2822),U256::from_dec_str("1000000000000000000").unwrap())),
//yfi-mobi 18
(Address::from_str("2e2f3246b6c65ccc4239c9ee556ec143a7e5de2c").unwrap(), (U256::from(97250),U256::from_dec_str("1000000000000000000").unwrap())),
//keep4r 18
(Address::from_str("a89ac6e529acf391cfbbd377f3ac9d93eae9664e").unwrap(), (U256::from(111745),U256::from_dec_str("1000000000000000000").unwrap())),
//social-rocket 18
(Address::from_str("0829d2d5cc09d3d341e813c821b0cfae272d9fb2").unwrap(), (U256::from(394),U256::from_dec_str("1000000000000000000").unwrap())),
//swiss-finance 18
(Address::from_str("692eb773e0b5b7a79efac5a015c8b36a2577f65c").unwrap(), (U256::from(339073),U256::from_dec_str("1000000000000000000").unwrap())),
//liquidefi 18
(Address::from_str("72ca0501427bb8f089c1c4f767cb17d017e803a9").unwrap(), (U256::from(212616),U256::from_dec_str("1000000000000000000").unwrap())),
//empty-set-dollar 18
(Address::from_str("36f3fd68e7325a35eb768f1aedaae9ea0689d723").unwrap(), (U256::from(185),U256::from_dec_str("1000000000000000000").unwrap())),
//reflect-finance 9
(Address::from_str("a1afffe3f4d611d252010e3eaf6f4d77088b0cd7").unwrap(), (U256::from(327),U256::from_dec_str("1000000000").unwrap())),
//tadpole-finance 18
(Address::from_str("9f7229af0c4b9740e207ea283b9094983f78ba04").unwrap(), (U256::from(60199),U256::from_dec_str("1000000000000000000").unwrap())),
//komet 18
(Address::from_str("6cfb6df56bbdb00226aeffcdb2cd1fe8da1abda7").unwrap(), (U256::from(295276),U256::from_dec_str("1000000000000000000").unwrap())),
//gny 18
(Address::from_str("b1f871Ae9462F1b2C6826E88A7827e76f86751d4").unwrap(), (U256::from(1967),U256::from_dec_str("1000000000000000000").unwrap())),
//itchiro-games 18
(Address::from_str("21cf09BC065082478Dcc9ccB5fd215A978Dc8d86").unwrap(), (U256::from(23111),U256::from_dec_str("1000000000000000000").unwrap())),
//baepay 4
(Address::from_str("6bffa07a1b0cebc474ce6833eaf2be6326252449").unwrap(), (U256::from(664),U256::from_dec_str("10000").unwrap())),
//bifrost 18
(Address::from_str("0c7D5ae016f806603CB1782bEa29AC69471CAb9c").unwrap(), (U256::from(1773),U256::from_dec_str("1000000000000000000").unwrap())),
//elysia 18
(Address::from_str("2781246fe707bb15cee3e5ea354e2154a2877b16").unwrap(), (U256::from(92),U256::from_dec_str("1000000000000000000").unwrap())),
//prophet 9
(Address::from_str("8d5db0c1f0681071cb38a382ae6704588d9da587").unwrap(), (U256::from(1141),U256::from_dec_str("1000000000").unwrap())),
//buy-sell 18
(Address::from_str("a30189d8255322a2f8b2a77906b000aeb005570c").unwrap(), (U256::from(12249),U256::from_dec_str("1000000000000000000").unwrap())),
//basis-cash 18
(Address::from_str("3449FC1Cd036255BA1EB19d65fF4BA2b8903A69a").unwrap(), (U256::from(265),U256::from_dec_str("1000000000000000000").unwrap())),
//predictz 18
(Address::from_str("4e085036a1b732cbe4ffb1c12ddfdd87e7c3664d").unwrap(), (U256::from(121300),U256::from_dec_str("1000000000000000000").unwrap())),
//xvix 18
(Address::from_str("4bAE380B5D762D543d426331b8437926443ae9ec").unwrap(), (U256::from(310000),U256::from_dec_str("1000000000000000000").unwrap())),
//seth 18
(Address::from_str("5e74c9036fb86bd7ecdcb084a0673efc32ea31cb").unwrap(), (U256::from(32546600),U256::from_dec_str("1000000000000000000").unwrap())),
//basis-share 18
(Address::from_str("106538CC16F938776c7c180186975BCA23875287").unwrap(), (U256::from(17419),U256::from_dec_str("1000000000000000000").unwrap())),
//mirrored-ishares-gold-trust 18
(Address::from_str("1d350417d9787E000cc1b95d70E9536DcD91F373").unwrap(), (U256::from(352720),U256::from_dec_str("1000000000000000000").unwrap())),
//goldenratioperliquidity 18
(Address::from_str("15e4132dcd932e8990e794d1300011a472819cbd").unwrap(), (U256::from(857569),U256::from_dec_str("1000000000000000000").unwrap())),
//n3rd-finance 18
(Address::from_str("32c868f6318d6334b2250f323d914bc2239e4eee").unwrap(), (U256::from(195100),U256::from_dec_str("1000000000000000000").unwrap())),
//unilock-network 18
(Address::from_str("354e514c135c8603f840ffadb4c33cde6d2a37e0").unwrap(), (U256::from(359),U256::from_dec_str("1000000000000000000").unwrap())),
//tornado 18
(Address::from_str("7A3D5d49D64E57DBd6FBB21dF7202bD3EE7A2253").unwrap(), (U256::from(709400),U256::from_dec_str("1000000000000000000").unwrap())),
//bitpower 18
(Address::from_str("52d904eff2605463c2f0b338d34abc9b7c3e3b08").unwrap(), (U256::from(90),U256::from_dec_str("1000000000000000000").unwrap())),
//royale-finance 18
(Address::from_str("7eaf9c89037e4814dc0d9952ac7f888c784548db").unwrap(), (U256::from(444),U256::from_dec_str("1000000000000000000").unwrap())),
//xdef-finance 9
(Address::from_str("5166d4ce79b9bf7df477da110c560ce3045aa889").unwrap(), (U256::from(4554),U256::from_dec_str("1000000000").unwrap())),
//wrapped-monero 18
(Address::from_str("465e07d6028830124be2e4aa551fbe12805db0f5").unwrap(), (U256::from(2328900),U256::from_dec_str("1000000000000000000").unwrap())),
//prophecy 18
(Address::from_str("3C81D482172cC273c3b91dD9D8eb212023D00521").unwrap(), (U256::from(31),U256::from_dec_str("1000000000000000000").unwrap())),
//defisocial-gaming 18
(Address::from_str("54ee01beB60E745329E6a8711Ad2D6cb213e38d7").unwrap(), (U256::from(2797400),U256::from_dec_str("1000000000000000000").unwrap())),
//armor-nxm 18
(Address::from_str("1337def18c680af1f9f45cbcab6309562975b1dd").unwrap(), (U256::from(613100),U256::from_dec_str("1000000000000000000").unwrap())),
//yftether 18
(Address::from_str("94f31ac896c9823d81cf9c2c93feceed4923218f").unwrap(), (U256::from(163266),U256::from_dec_str("1000000000000000000").unwrap())),
//newscrypto 18
(Address::from_str("968f6f898a6df937fc1859b323ac2f14643e3fed").unwrap(), (U256::from(3620),U256::from_dec_str("1000000000000000000").unwrap())),
//xstable-protocol 9
(Address::from_str("91383a15c391c142b80045d8b4730c1c37ac0378").unwrap(), (U256::from(2106),U256::from_dec_str("1000000000").unwrap())),
//protocol-finance 18
(Address::from_str("7b69d465c0f9fb22affae56aa86149973e9b0966").unwrap(), (U256::from(132900),U256::from_dec_str("1000000000000000000").unwrap())),
//qfinance 18
(Address::from_str("6fe88a211863d0d818608036880c9a4b0ea86795").unwrap(), (U256::from(3247),U256::from_dec_str("1000000000000000000").unwrap())),
//yfione 18
(Address::from_str("ac0c8da4a4748d8d821a0973d00b157aa78c473d").unwrap(), (U256::from(494900),U256::from_dec_str("1000000000000000000").unwrap())),
//lotto 18
(Address::from_str("b0dFd28d3CF7A5897C694904Ace292539242f858").unwrap(), (U256::from(142),U256::from_dec_str("1000000000000000000").unwrap())),
//mp3 18
(Address::from_str("018fb5af9d015af25592a014c4266a84143de7a0").unwrap(), (U256::from(156),U256::from_dec_str("1000000000000000000").unwrap())),
//interop 18
(Address::from_str("2eC75589856562646afE393455986CaD26c4Cc5f").unwrap(), (U256::from(7645),U256::from_dec_str("1000000000000000000").unwrap())),
//name-change-token 18
(Address::from_str("8a9c4dfe8b9d8962b31e4e16f8321c44d48e246e").unwrap(), (U256::from(154),U256::from_dec_str("1000000000000000000").unwrap())),
//chow-chow 9
(Address::from_str("925f2c11b99c1a4c46606898ee91ed3d450cfeda").unwrap(), (U256::from(32),U256::from_dec_str("1000000000").unwrap())),
//wrapped-cryptokitties 18
(Address::from_str("09fE5f0236F0Ea5D930197DCE254d77B04128075").unwrap(), (U256::from(57213),U256::from_dec_str("1000000000000000000").unwrap())),
//soar-fi 9
(Address::from_str("bae5f2d8a1299e5c4963eaff3312399253f27ccb").unwrap(), (U256::from(441),U256::from_dec_str("1000000000").unwrap())),
//tama-egg-niftygotchi 18
(Address::from_str("6e742e29395cf5736c358538f0f1372ab3dfe731").unwrap(), (U256::from(591704),U256::from_dec_str("1000000000000000000").unwrap())),
//basix 18
(Address::from_str("23157662a9cb9be32d4d9bd019d9bcbaa040a62b").unwrap(), (U256::from(4556),U256::from_dec_str("1000000000000000000").unwrap())),
//unidexgas 18
(Address::from_str("a5959e9412d27041194c3c3bcbe855face2864f7").unwrap(), (U256::from(188221),U256::from_dec_str("1000000000000000000").unwrap())),
//previse 18
(Address::from_str("a36e59c08c9f251a6b7a9eb6be6e32fd6157acd0").unwrap(), (U256::from(1106),U256::from_dec_str("1000000000000000000").unwrap())),
//bt-finance 18
(Address::from_str("76c5449f4950f6338a393f53cda8b53b0cd3ca3a").unwrap(), (U256::from(5946),U256::from_dec_str("1000000000000000000").unwrap())),
//dexmex 18
(Address::from_str("0020d80229877b495d2bf3269a4c13f6f1e1b9d3").unwrap(), (U256::from(92),U256::from_dec_str("1000000000000000000").unwrap())),
//whaleroom 18
(Address::from_str("2af72850c504ddd3c1876c66a914caee7ff8a46a").unwrap(), (U256::from(39244),U256::from_dec_str("1000000000000000000").unwrap())),
//mcdonalds-coin 2
(Address::from_str("8937041c8c52a78c25aa54051f6a9dada23d42a2").unwrap(), (U256::from(25),U256::from_dec_str("100").unwrap())),
//rug-proof 18
(Address::from_str("a0bb0027c28ade4ac628b7f81e7b93ec71b4e020").unwrap(), (U256::from(956),U256::from_dec_str("1000000000000000000").unwrap())),
//defi-wizard 18
(Address::from_str("7dee45dff03ec7137979586ca20a2f4917bac9fa").unwrap(), (U256::from(6653),U256::from_dec_str("1000000000000000000").unwrap())),
//marsan-exchange-token 18
(Address::from_str("9af5a20aac8d83230ba68542ba29d132d50cbe08").unwrap(), (U256::from(245),U256::from_dec_str("1000000000000000000").unwrap())),
//vow 18
(Address::from_str("1BBf25e71EC48B84d773809B4bA55B6F4bE946Fb").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//deor 10
(Address::from_str("63726dae7c57d25e90ec829ce9a5c745ffd984d3").unwrap(), (U256::from(44),U256::from_dec_str("10000000000").unwrap())),
//prime-whiterock-company 18
(Address::from_str("a3d93c0616dbc31fef1e112c7665a4ba4ddbf0be").unwrap(), (U256::from(40),U256::from_dec_str("1000000000000000000").unwrap())),
//playcent 18
(Address::from_str("657B83A0336561C8f64389a6f5aDE675C04b0C3b").unwrap(), (U256::from(568),U256::from_dec_str("1000000000000000000").unwrap())),
//unifund 18
(Address::from_str("04b5e13000c6e9a3255dc057091f3e3eeee7b0f0").unwrap(), (U256::from(60),U256::from_dec_str("1000000000000000000").unwrap())),
//next-coin 18
(Address::from_str("377d552914e7a104bc22b4f3b6268ddc69615be7").unwrap(), (U256::from(1256),U256::from_dec_str("1000000000000000000").unwrap())),
//transmute-protocol 18
(Address::from_str("bC81BF5B3173BCCDBE62dba5f5b695522aD63559").unwrap(), (U256::from(2847),U256::from_dec_str("1000000000000000000").unwrap())),
//agoras-tokens 8
(Address::from_str("738865301a9b7dd80dc3666dd48cf034ec42bdda").unwrap(), (U256::from(8023),U256::from_dec_str("100000000").unwrap())),
//keytango 18
(Address::from_str("182f4c4c97cd1c24e1df8fc4c053e5c47bf53bef").unwrap(), (U256::from(605),U256::from_dec_str("1000000000000000000").unwrap())),
//shadetech 18
(Address::from_str("8a8221628361fa25294a83a172dd4f0133207b37").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//exrt-network 8
(Address::from_str("b20043F149817bff5322F1b928e89aBFC65A9925").unwrap(), (U256::from(21),U256::from_dec_str("100000000").unwrap())),
//rocket-bunny 9
(Address::from_str("3ea50b7ef6a7eaf7e966e2cb72b519c16557497c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//cops-finance 18
(Address::from_str("14dfa5cfaafe89d81d7bf3df4e11eaeda0416618").unwrap(), (U256::from(4850123),U256::from_dec_str("1000000000000000000").unwrap())),
//quai-dao 18
(Address::from_str("40821cd074dfecb1524286923bc69315075b5c89").unwrap(), (U256::from(252),U256::from_dec_str("1000000000000000000").unwrap())),
//farming-bad 18
(Address::from_str("11003e410ca3fcd220765b3d2f343433a0b2bffd").unwrap(), (U256::from(57),U256::from_dec_str("1000000000000000000").unwrap())),
//rare-pepe 18
(Address::from_str("0e9b56d2233ea2b5883861754435f9c51dbca141").unwrap(), (U256::from(151),U256::from_dec_str("1000000000000000000").unwrap())),
//collective 18
(Address::from_str("75739d5944534115d7c54ee8c73f186d793bae02").unwrap(), (U256::from(5301),U256::from_dec_str("1000000000000000000").unwrap())),
//delta-finance 18
(Address::from_str("9ea3b5b4ec044b70375236a281986106457b20ef").unwrap(), (U256::from(28561),U256::from_dec_str("1000000000000000000").unwrap())),
//nodeseeds 18
(Address::from_str("747f564d258612ec5c4e24742c5fd4110bcbe46b").unwrap(), (U256::from(474400),U256::from_dec_str("1000000000000000000").unwrap())),
//yield-protocol 18
(Address::from_str("a8B61CfF52564758A204F841E636265bEBC8db9B").unwrap(), (U256::from(141),U256::from_dec_str("1000000000000000000").unwrap())),
//mu-dank 18
(Address::from_str("9ea1ae46c15a4164b74463bc26f8aa3b0eea2e6e").unwrap(), (U256::from(10),U256::from_dec_str("1000000000000000000").unwrap())),
//method-finance 18
(Address::from_str("84ba4aecfde39d69686a841bab434c32d179a169").unwrap(), (U256::from(65),U256::from_dec_str("1000000000000000000").unwrap())),
//bta-protocol 18
(Address::from_str("270371c58d9d775ed73971dd414656107384f235").unwrap(), (U256::from(11),U256::from_dec_str("1000000000000000000").unwrap())),
//xdefi 18
(Address::from_str("000000000000d0151e748d25b766e77efe2a6c83").unwrap(), (U256::from(346),U256::from_dec_str("1000000000000000000").unwrap())),
//b21-invest 18
(Address::from_str("6faa826af0568d1866fca570da79b318ef114dab").unwrap(), (U256::from(429),U256::from_dec_str("1000000000000000000").unwrap())),
//artx-trading 18
(Address::from_str("741b0428efdf4372a8df6fb54b018db5e5ab7710").unwrap(), (U256::from(814),U256::from_dec_str("1000000000000000000").unwrap())),
//frogdao-dime 18
(Address::from_str("14cfc7aeaa468e8c789785c39e0b753915aeb426").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//shardingdao 18
(Address::from_str("5845cd0205b5d43af695412a79cf7c1aeddb060f").unwrap(), (U256::from(2580),U256::from_dec_str("1000000000000000000").unwrap())),
//secure-pad-token 18
(Address::from_str("10994aa2fb8e6ba5d9fb2bc127ff228c4fe6167f").unwrap(), (U256::from(13032),U256::from_dec_str("1000000000000000000").unwrap())),
//saren 18
(Address::from_str("bd4a858139b155219e2c8d10135003fdef720b6b").unwrap(), (U256::from(346),U256::from_dec_str("1000000000000000000").unwrap())),
//busy 18
(Address::from_str("5CB3ce6D081fB00d5f6677d196f2d70010EA3f4a").unwrap(), (U256::from(149),U256::from_dec_str("1000000000000000000").unwrap())),
//dart-insurance 18
(Address::from_str("5a4623F305A8d7904ED68638AF3B4328678edDBF").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//cyclone-protocol 18
(Address::from_str("8861cff2366c1128fd699b68304ad99a0764ef9a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//impermax 18
(Address::from_str("7b35ce522cb72e4077baeb96cb923a5529764a00").unwrap(), (U256::from(1666),U256::from_dec_str("1000000000000000000").unwrap())),
//unitedcrowd 18
(Address::from_str("6d1dc3928604b00180bb570bdae94b9698d33b79").unwrap(), (U256::from(251),U256::from_dec_str("1000000000000000000").unwrap())),
//franklin 4
(Address::from_str("85f6eb2bd5a062f5f8560be93fb7147e16c81472").unwrap(), (U256::from(110),U256::from_dec_str("10000").unwrap())),
//apehaven 18
(Address::from_str("14dd7ebe6cb084cb73ef377e115554d47dc9d61e").unwrap(), (U256::from(1),U256::from_dec_str("1000000000000000000").unwrap())),
//peri-finance 18
(Address::from_str("5d30aD9C6374Bf925D0A75454fa327AACf778492").unwrap(), (U256::from(5496),U256::from_dec_str("1000000000000000000").unwrap())),
//teslafan 18
(Address::from_str("2d5bed63b0fe325ed3b865ae2cdaa3649eb25461").unwrap(), (U256::from(332),U256::from_dec_str("1000000000000000000").unwrap())),
//island-coin 9
(Address::from_str("1681bcB589b3cFCF0c0616B0cE9b19b240643dc1").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//burnx20 9
(Address::from_str("1e950AF2F6f8505c09F0Ca42c4b38F10979cb22E").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//dick 18
(Address::from_str("20af547291dfe691baf43658f2c8515076d18408").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//vision-network 18
(Address::from_str("456ae45c0ce901e2e7c99c0718031cec0a7a59ff").unwrap(), (U256::from(7),U256::from_dec_str("1000000000000000000").unwrap())),
//freela 18
(Address::from_str("29ceddcf0da3c1d8068a7dfbd0fb06c2e438ff70").unwrap(), (U256::from(45),U256::from_dec_str("1000000000000000000").unwrap())),
//kombai-inu 9
(Address::from_str("3fce6ae1f55656663ba6a5b0e0812463cf45c2ee").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//bulk 18
(Address::from_str("a143ac515dca260a46c742c7251ef3b268639593").unwrap(), (U256::from(292),U256::from_dec_str("1000000000000000000").unwrap())),
//direwolf 2
(Address::from_str("bdea5bb640dbfc4593809deec5cdb8f99b704cd2").unwrap(), (U256::from(0),U256::from_dec_str("100").unwrap())),
//give-global 18
(Address::from_str("ba8e5a4c64c1be42230910f7b39a6388f3d4297c").unwrap(), (U256::from(4),U256::from_dec_str("1000000000000000000").unwrap())),
//jomon-inu 9
(Address::from_str("439dd02bFd144A5d6A5967895358E0d25d5ab784").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//tinku 9
(Address::from_str("47FA4B26c1c52Bc35654F98D10Cd61b9f3E10267").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//cryption-network 18
(Address::from_str("429876c4a6f89fb470e92456b8313879df98b63c").unwrap(), (U256::from(371),U256::from_dec_str("1000000000000000000").unwrap())),
//cavapoo 9
(Address::from_str("456d8f0d25a4e787ee60c401f8b963a465148f70").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//star-foxx 18
(Address::from_str("31D457E7bcFf5Bc9A5Ef86E6a5eA1DB5b5C3BFB0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//bella-protocol 18
(Address::from_str("a91ac63d040deb1b7a5e4d4134ad23eb0ba07e14").unwrap(), (U256::from(13508),U256::from_dec_str("1000000000000000000").unwrap())),
//boombaby-io 9
(Address::from_str("82b89e0f9c0695639eb88659d0c306dbc242af96").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//sakhalin-husky 9
(Address::from_str("2b1fe2cea92436e8c34b7c215af66aaa2932a8b2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//ofc-coin 9
(Address::from_str("b3b975fc904e67858ecfee48a49d7269b3e0b949").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//godl 18
(Address::from_str("7f509465c38b66bdecec2cfdc842e11809cc8357").unwrap(), (U256::from(2),U256::from_dec_str("1000000000000000000").unwrap())),
//tardigrades-finance-eth 9
(Address::from_str("92a42db88ed0f02c71d439e55962ca7cab0168b5").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//dark-matter 18
(Address::from_str("79126d32a86e6663f3aaac4527732d0701c1ae6c").unwrap(), (U256::from(377400),U256::from_dec_str("1000000000000000000").unwrap())),
//polkalokr 18
(Address::from_str("80ce3027a70e0a928d9268994e9b85d03bd4cdcf").unwrap(), (U256::from(1054),U256::from_dec_str("1000000000000000000").unwrap())),
//digies-coin 9
(Address::from_str("7333cbf5b0b843b4129e234f791b0058347f671a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//projekt-diamond 9
(Address::from_str("53109fe9e044f2c324d00ad85bfb0b13ce379480").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//tenshi 9
(Address::from_str("9358e3a79d428c7708da22a5bd085159f6818d12").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//night-life-crypto 8
(Address::from_str("1951ab088141e69a3713a351b0d55ba3acda192c").unwrap(), (U256::from(8432),U256::from_dec_str("100000000").unwrap())),
//taiyo 9
(Address::from_str("13db9034c9ca6cb739887288fce790544a476f8c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//savebritney 18
(Address::from_str("606ce698aea1dca5a2627a4583da13a340667f09").unwrap(), (U256::from(23),U256::from_dec_str("1000000000000000000").unwrap())),
//dinox 18
(Address::from_str("20a8cec5fffea65be7122bcab2ffe32ed4ebf03a").unwrap(), (U256::from(2422),U256::from_dec_str("1000000000000000000").unwrap())),
//gambler-shiba 18
(Address::from_str("b892249939adbf6d7851864ca9a5c7d2d537af97").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//the-tokenized-bitcoin 8
(Address::from_str("3212b29E33587A00FB1C83346f5dBFA69A458923").unwrap(), (U256::from(396160000),U256::from_dec_str("100000000").unwrap())),
//robo-token 18
(Address::from_str("6fc2f1044a3b9bb3e43a43ec8f840843ed753061").unwrap(), (U256::from(265),U256::from_dec_str("1000000000000000000").unwrap())),
//ethereum-chain-token 9
(Address::from_str("59d71082d8a5b18ebc6b653ae422ac4383cd2597").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//meteorite-network 9
(Address::from_str("765baefcb5418fa9f7dddacb1ccc07bd0e890e4e").unwrap(), (U256::from(119300),U256::from_dec_str("1000000000").unwrap())),
//key 18
(Address::from_str("4cd988afbad37289baaf53c13e98e2bd46aaea8c").unwrap(), (U256::from(2),U256::from_dec_str("1000000000000000000").unwrap())),
//smoothy 18
(Address::from_str("bF776e4FCa664D791C4Ee3A71e2722990E003283").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//antique-zombie-shards 18
(Address::from_str("78175901e9B04090Bf3B3D3cB7f91CA986fb1aF6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//picaartmoney 0
(Address::from_str("A7E0719a65128b2f6cDbc86096753Ff7d5962106").unwrap(), (U256::from(112),U256::from_dec_str("1").unwrap())),
//puppies-network 9
(Address::from_str("95f49ae439537e50CED0374c1B52C42AA899741C").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//mirrored-facebook 18
(Address::from_str("0e99cC0535BB6251F6679Fa6E65d6d3b430e840B").unwrap(), (U256::from(3494980),U256::from_dec_str("1000000000000000000").unwrap())),
//global-defi 18
(Address::from_str("b5e88b229b18e748e3aa16a1c2bfefdfc8a5560d").unwrap(), (U256::from(15300),U256::from_dec_str("1000000000000000000").unwrap())),
//polylauncher 18
(Address::from_str("6c7b97c7e09e790d161769a52f155125fac6d5a1").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//aelf 18
(Address::from_str("bf2179859fc6D5BEE9Bf9158632Dc51678a4100e").unwrap(), (U256::from(4051),U256::from_dec_str("1000000000000000000").unwrap())),
//afterback 18
(Address::from_str("0eaca6ec24e461f76c4da385571336f954c9652a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//banketh 18
(Address::from_str("be0c826f17680d8da620855be89dd6544c034ca1").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//zin-finance 18
(Address::from_str("033e223870f766644f7f7a4B7dc2E91573707d06").unwrap(), (U256::from(8),U256::from_dec_str("1000000000000000000").unwrap())),
//snap-token 9
(Address::from_str("4c5813b8c6fbbac76caa148aaf8910f236b56fdf").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//adventure-token 18
(Address::from_str("a2ef2757d2ed560c9e3758d1946d7bcccbd5a7fe").unwrap(), (U256::from(429),U256::from_dec_str("1000000000000000000").unwrap())),
//wrapped-fct 8
(Address::from_str("415acc3c6636211e67e248dc28400b452acefa68").unwrap(), (U256::from(0),U256::from_dec_str("100000000").unwrap())),
//polkaparty 18
(Address::from_str("48592de8cded16f6bb56c896fe1affc37630889c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//flurry 18
(Address::from_str("60f63b76e2fc1649e57a3489162732a90acf59fe").unwrap(), (U256::from(11),U256::from_dec_str("1000000000000000000").unwrap())),
//whalestreet-shrimp-token 18
(Address::from_str("9077f9e1efe0ea72867ac89046b2a6264cbcaef5").unwrap(), (U256::from(266),U256::from_dec_str("1000000000000000000").unwrap())),
//ledgity 18
(Address::from_str("85Ffb35957203dfD12061eAeCD708dB623Bd567C").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//contribute-dao 18
(Address::from_str("8e84ee8b28ddbe2b1d5e204e674460835d298815").unwrap(), (U256::from(1083311),U256::from_dec_str("1000000000000000000").unwrap())),
//identity 18
(Address::from_str("6fB1E018f107d3352506c23777e4cd62e063584a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//carboneco 9
(Address::from_str("bb3c2a170fbb8988cdb41c04344f9863b0f71c20").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//bscstarter 18
(Address::from_str("1d7Ca62F6Af49ec66f6680b8606E634E55Ef22C1").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//tenup 18
(Address::from_str("7714f320Adca62B149df2579361AfEC729c5FE6A").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//bond-appetite-usd 18
(Address::from_str("9a1997c130f4b2997166975d9aff92797d5134c2").unwrap(), (U256::from(9810),U256::from_dec_str("1000000000000000000").unwrap())),
//bondappetit-governance-token 18
(Address::from_str("28A06c02287e657ec3F8e151A13C36A1D43814b0").unwrap(), (U256::from(810),U256::from_dec_str("1000000000000000000").unwrap())),
//despace-protocol 18
(Address::from_str("634239cfa331df0291653139d1a6083b9cf705e3").unwrap(), (U256::from(1266),U256::from_dec_str("1000000000000000000").unwrap())),
//dds-store 9
(Address::from_str("25e4579f028e2629ed15c70a378d82209cfb5e7d").unwrap(), (U256::from(9188),U256::from_dec_str("1000000000").unwrap())),
//matrixetf 18
(Address::from_str("1a57367c6194199e5d9aea1ce027431682dfb411").unwrap(), (U256::from(199),U256::from_dec_str("1000000000000000000").unwrap())),
//pasv 6
(Address::from_str("1cea6313400ddbcb503c23f5a4facd3014f29872").unwrap(), (U256::from(0),U256::from_dec_str("1000000").unwrap())),
//ethereum-pro-new 18
(Address::from_str("AB6E163cBEB3959b68b90beC722F5a9EEf82bA72").unwrap(), (U256::from(2),U256::from_dec_str("1000000000000000000").unwrap())),
//dfbtc 18
(Address::from_str("060924fb947e37eee230d0b1a71d9618aec269fc").unwrap(), (U256::from(6808),U256::from_dec_str("1000000000000000000").unwrap())),
//picipo 18
(Address::from_str("1e05f68B29b286FB3BbAd3c688D7e2ABda549b80").unwrap(), (U256::from(366),U256::from_dec_str("1000000000000000000").unwrap())),
//centurion-inu 9
(Address::from_str("9f91d9f9070b0478abb5a9918c79b5dd533f672c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//two-two 18
(Address::from_str("41045282901E90BDa7578D628e479E5421D1cDD5").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//crypto-phoenix 18
(Address::from_str("8689d850cdf3b74a1f6a5eb60302c785b71c2fc7").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//beach-token 9
(Address::from_str("bd15c4c8cd28a08e43846e3155c01a1f648d8d42").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//0xcert 18
(Address::from_str("83e2be8d114f9661221384b3a50d24b96a5653f5").unwrap(), (U256::from(22),U256::from_dec_str("1000000000000000000").unwrap())),
//happy-fans 18
(Address::from_str("3079F61704E9eFa2BcF1db412f735d8d4cFa26f4").unwrap(), (U256::from(1),U256::from_dec_str("1000000000000000000").unwrap())),
//inu-token 9
(Address::from_str("00f29171d7bcdc464a0758cf3217fe83173772b9").unwrap(), (U256::from(39),U256::from_dec_str("1000000000").unwrap())),
//haildraconis 18
(Address::from_str("3b08c03fa8278cf81b9043b228183760376fcdbb").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//internet-of-energy-network 18
(Address::from_str("1e4E46b7BF03ECE908c88FF7cC4975560010893A").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//protector-roge 9
(Address::from_str("282d0ad1fa03dfbdb88243b958e77349c73737d1").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//cryptopunt 18
(Address::from_str("31903E333809897eE57Af57567f4377a1a78756c").unwrap(), (U256::from(111),U256::from_dec_str("1000000000000000000").unwrap())),
//gogeta-inu 9
(Address::from_str("636484a1c41e88e3fc7c99248ca0b3c3a844ab86").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//billion-token 18
(Address::from_str("065cc8636a00c007276ed9cb874cd59b89e6609b").unwrap(), (U256::from(4),U256::from_dec_str("1000000000000000000").unwrap())),
//ichigo-inu 9
(Address::from_str("8254c1c134436f74047f79eaaea97e3324ef78b5").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//inubis 9
(Address::from_str("ab917b34b57f1c01c5df8ddc0f75828e3914fce6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//charizard-inu 9
(Address::from_str("727e8260877f8507f8d61917e9778b6af8491e63").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//peanuts 18
(Address::from_str("9f41da75ab2b8c6f0dcef7173c4bf66bd4f6b36a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//mason-token 18
(Address::from_str("3d2c03b2504e4e593169fac757788aac9d303a4e").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//akamaru-inu 9
(Address::from_str("4abac7a6acf3ce84f1c2fa07d91e72cdd6081cd3").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//eiichiro-oda-inu 9
(Address::from_str("04dc37b220a055c5f93680815f670babcd912c2c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//jpaw-inu 9
(Address::from_str("2740641bb774a4f41f814d969ba1967155e3470a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//yukon 9
(Address::from_str("724a4dbc096e8553120ec99d975ca62c1e4f9f51").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//my-shiba-academia 9
(Address::from_str("93a20a5f1709659005e1610d1a022d5f1e2d0df7").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//chilliswap 18
(Address::from_str("12b54baA8FFcFd6679CcF1AE618ca3006cFcc2aC").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//cats-claw 9
(Address::from_str("02eddbbf40f7ab1b6fd1a87bf263d4be967d0552").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//blocks 18
(Address::from_str("8a6d4c8735371ebaf8874fbd518b56edd66024eb").unwrap(), (U256::from(285),U256::from_dec_str("1000000000000000000").unwrap())),
//metashib-token 9
(Address::from_str("181c94a45ed257baf2211d4ff7e1f49a5964134a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//megacosm 9
(Address::from_str("15fc9f4efdd40f0f8a62f2a2ee7bbc79679540e8").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//oobit 18
(Address::from_str("07f9702ce093db82dfdc92c2c6e578d6ea8d5e22").unwrap(), (U256::from(3138),U256::from_dec_str("1000000000000000000").unwrap())),
//shokky 9
(Address::from_str("b02db7bd0cbc93a31f3c92349b4a206368174fc0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//shibamon 9
(Address::from_str("36b00c4c6ce3653a091c7940fc98c3acb0043871").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//scoobi-doge 18
(Address::from_str("06a87f6afec4a739c367bef69eefe383d27106bd").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//dogus 18
(Address::from_str("903904cb39bac33d4983ead3b3f573d720c7965e").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//mega-shiba-inu 9
(Address::from_str("1c23f0f3e06fa0e07c5e661353612a2d63323bc6").unwrap(), (U256::from(65),U256::from_dec_str("1000000000").unwrap())),
//nezuko-inu 9
(Address::from_str("bc298dfaa2edda095b924f1390cc38fb7c5f6250").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//berserk-inu 9
(Address::from_str("55ae8e43172e91fab2a9e97636023f4c87b4c470").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//babelfish 9
(Address::from_str("014d9a527fe5d11c178d70248921db2b735d6e41").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//mashima-inu 9
(Address::from_str("b2f8a70b09db0f7795a5f079b5021eb84aa59e28").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//balls 9
(Address::from_str("174ed6e64a5903b59ca7910081e1e3a2c551afc6").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//momento 9
(Address::from_str("0ae8b74cd2d566853715800c9927f879d6b76a37").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//togashi-inu 9
(Address::from_str("5daa0cbe290e082dbfd6f595e2e53b678895f322").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//x-ae-a-12 9
(Address::from_str("1902882a8f6c7fb1402f83c434ea8e064b35bab3").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//rbx 18
(Address::from_str("8254e26e453eb5abd29b3c37ac9e8da32e5d3299").unwrap(), (U256::from(909),U256::from_dec_str("1000000000000000000").unwrap())),
//gm-coin 9
(Address::from_str("73b8726618f53f84eeb860fd50ab217fdf30dea0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//mewtwo-inu 9
(Address::from_str("4F2AB9D03ce5b8D0d3BcA09259c78005D2775E08").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//mishka-token 9
(Address::from_str("976091738973b520a514ea206acdd008a09649de").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//first-inu 9
(Address::from_str("1bdc5e5aa2749b4934c33441e050b8854b77a331").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//entropyfi 18
(Address::from_str("0a0e3bfD5a8cE610E735D4469Bc1b3b130402267").unwrap(), (U256::from(334),U256::from_dec_str("1000000000000000000").unwrap())),
//polkainu 9
(Address::from_str("aabcecd071ab4ace5496f6ff3e1c4c3ee8116f75").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//olympus-inu-dao 9
(Address::from_str("98F817765f69c802a7b188A3165a3267aD2d1123").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//robin-inu 9
(Address::from_str("10b6bd5e0abab280ec1c5313ee04ccbe91a2ebae").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//psyduck-inu 9
(Address::from_str("99342b1a141aa3a02e04afb496562037fdf8e655").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//phantom-protocol 18
(Address::from_str("3f9bec82c776c47405bcb38070d2395fd18f89d3").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//tetsu-inu 9
(Address::from_str("1e9dae82fa136796d306695b8be1e151bc5365e8").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//cross-chain-bridge-token 18
(Address::from_str("92868a5255c628da08f550a858a802f5351c5223").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//garfield-token 9
(Address::from_str("7b392dd9bdef6e17c3d1ba62d1a6c7dcc99d839b").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//spidey-inu 9
(Address::from_str("6ff952aef0c0f7c7e20cc396b798daddf6561f18").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//goldinu 9
(Address::from_str("b2ed199b46630e789e8740fb83b1611acf018516").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//have-fun-staying-poor 9
(Address::from_str("7343581f55146951b0f678dc6cfa8fd360e2f353").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//baby-cat-girl 9
(Address::from_str("06E04bBfA6a53c57EbfC17e1AEed8E2686640eCd").unwrap(), (U256::from(7),U256::from_dec_str("1000000000").unwrap())),
//consensus-cell-network 2
(Address::from_str("9b62ec1453cea5dde760aaf662048ca6eeb66e7f").unwrap(), (U256::from(80),U256::from_dec_str("100").unwrap())),
//no-face-inu 18
(Address::from_str("3093003005fd7c9c077e85c15ff47bcfcf0397e0").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//naruto-inu 9
(Address::from_str("bfce0e06dedcbea3e170ba4df2a6793334cac5ef").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//ether-terrestrial 9
(Address::from_str("316f17a75978575e9fedc839ba393395a9d83877").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//shinedao 18
(Address::from_str("1c7ede23b1361acc098a1e357c9085d131b34a01").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//axus-coin-project 18
(Address::from_str("872d63d889d4b445c89a0887dcdbcc179b026432").unwrap(), (U256::from(282),U256::from_dec_str("1000000000000000000").unwrap())),
//spacelink 9
(Address::from_str("56a41eef4aba11292c58b39f61dabc82ed22c79b").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//media-eye 18
(Address::from_str("9a257c90fa239fba07771ef7da2d554d148c2e89").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//ludena-protocol 18
(Address::from_str("b29663Aa4E2e81e425294193616c1B102B70a158").unwrap(), (U256::from(17800),U256::from_dec_str("1000000000000000000").unwrap())),
//angle-protocol 18
(Address::from_str("1a7e4e63778b4f12a199c062f3efdd288afcbce8").unwrap(), (U256::from(11500),U256::from_dec_str("1000000000000000000").unwrap())),
//zaddy-inu-token 18
(Address::from_str("4fff29d95a8953ad28847278dd6aa11f4c695a24").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//dinnersready 9
(Address::from_str("160c280fa54e9e8ee22e4f9a71ec96cc2a40f793").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//junior-shiba 18
(Address::from_str("73ee71cb9f0276f093f113c94c084a7a58ffd1e9").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//angel-inu 9
(Address::from_str("2373c5dc96238a64ce4062e74000fd3dacfd3bf7").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//vari-stable-capital 9
(Address::from_str("99bfe582a97f0ded07ee6fb5c1e5b6f1ff082243").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//dumpbuster 9
(Address::from_str("a0A9C16856C96D5E9d80a8696eEA5E02B2Dc3398").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//multigencapital 9
(Address::from_str("3ed5a70a149f3c758231a2d592c5b5b5aee86e35").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//planet-inu 9
(Address::from_str("a461258c192cb6057ad8729589b0d18b08ccace8").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//gobble-gobble 18
(Address::from_str("1ec1b3fffd5072d97b27110a667c35025c96d5c5").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//big-brain-capital-dao 9
(Address::from_str("270719e21852e0e817c4663cc9f1567441d6eaac").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//sportsicon 18
(Address::from_str("3f68e7b44e9bcb486c2feadb7a2289d9cdfc9088").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//studio-shibli 9
(Address::from_str("B1A88c33091490218965787919fcc9862C1798eE").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//supermegahyperdoge 9
(Address::from_str("5644bb2b594fcf6f74384d2ad26c68f02a47981c").unwrap(), (U256::from(1),U256::from_dec_str("1000000000").unwrap())),
//arcane-universe 9
(Address::from_str("58530a272bf650827ae05fadee76f36271089f7f").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//superbrain-capital-dao 9
(Address::from_str("2f02bE0C4021022b59E9436f335d69DF95E5222a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//freemoon-eth 9
(Address::from_str("31f0bc450c12eb62b4c617d4c876f7a66470fcb3").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//cage-io 18
(Address::from_str("8987a07ba83607a66c7351266e771fb865c9ca6c").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//bxmi-token 18
(Address::from_str("a0f5505dC06eBE8Ee8CbdC2059eaDE0b9F35cbC2").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//trava-finance 18
(Address::from_str("186d0ba3dfc3386c464eecd96a61fbb1e2da00bf").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//no-bull 9
(Address::from_str("20be82943e8d9c682580e11d424ec15db95b4a24").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//green-eyed-monster 9
(Address::from_str("a22d31228699efffe79b5403da9e7b4009732d6a").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//crafting-finance 18
(Address::from_str("508df5aa4746be37b5b6a69684dfd8bdc322219d").unwrap(), (U256::from(0),U256::from_dec_str("1000000000000000000").unwrap())),
//cobragoose 9
(Address::from_str("20dc897a85a204dac089ee1dc1998268a9b17fc1").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//roboshib 9
(Address::from_str("0b48a744669767a3478293fd4eecb8fdc5d33cda").unwrap(), (U256::from(0),U256::from_dec_str("1000000000").unwrap())),
//governance-ohm 18
(Address::from_str("0ab87046fBb341D058F17CBC4c1133F25a20a52f").unwrap(), (U256::from(164124800),U256::from_dec_str("1000000000000000000").unwrap())),

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