# OpenEthereum Flash Loan Front Run Project

The project is to present a new type of attack at OpenEthereum client end: When assembling next block for Ethereum, flash loan transactions are detected and the beneficiaries are calculated based on ERC20 token and ETH transfers in the transaction. Once the transaction is beneficial for senders, the transaction flash loan actions are copied and signed by front run address as well as a new contract deployment by the address. Miners connected to the client will mine the block containing the front run transactions instead of the original flash loan one.

## 1. Build & Run
- After install cargo 1.49. To build
```bash
$ cargo build --release
```
- Run full node

Get an account as a miner.
```bash
$ ./target/release/openethereum account new --base-path ./run/
```
Use the miner to start a full node. Here use `0x5297491909011f6c17639167bff80cb938e5e721` as an example.
```bash
$ ./target/release/openethereum --base-path ./run/ --author 0x5297491909011f6c17639167bff80cb938e5e721 --jsonrpc-hosts=127.0.0.1
```
- Run archive node for testing
```bash
$ ./target/release/openethereum --base-path ./archive_run/ --pruning='archive'
```
## 2. Code Structure
`crates/ethcore/src/miner/miner.rs: prepare_block`: defines picking best transactions from transaction pool, executing them and pushing them into a new block.

`crates/ethcore/src/block.rs: push_transaction`: creates flash loan detection overlay for the transaction and execute it. Once detecting flash loan transaction and beneficiaries, new contract deployment transaction and flash loan transaction signed by `FRONTRUN_ADDRESS` are created. 

1. Create a state checkpoint.
2. Execute original transaction and monitor token transfer actions during executing.
3. Summaries all of the beneficiaries and victims based on the transfers.
4. If it is beneficial, assemble contract deployment transaction and flash loan transaction denoting `a` and `b`.
5. The block state is reverted to the checkpoint.
6. Execute `a` and `b`. Analysis all token transfers and check whether one of the beneficiaries belongs to `FRONTRUN_ADDRESS`.
7. If failing to execute or check beneficiary, assemble initialization function call transaction `c`.
8. Revert state. Execute `a`, `b` and `c` and check beneficiary.
9. If failed, ignore the state changes. If succeed, drop original one and keep front run transactions in the block. Insert all of the receipts into the block.

`crates/ethcore/src/state/adversaryaccount.rs`: defines struct `AdversaryAccount` which is one of the fields in state. The struct records all of token transfers in current transaction, analysis overall victims and beneficiaries and assemble front run transactions. In addition, there are several helper functions store and read contract deployment transaction and initialization function call transaction information using [sled](https://github.com/spacejam/sled) database.

`crates/ethcore/src/state/frontrunmacro.rs`: defines constant global variables:

- ERC20 token information
- Flash loan provider addresses
- Front run address and its secret key
- The signatures of ERC20 typical events such as `transfer`, `deposit`, etc.
- Fake address used to burn/mint tokens

`Other files under crates/ethcore/src`: during executing opcode instructions of the transactions, all of the ETH transfers are recorded in object `AdversaryAccount` when the beginning of execution and `CALL` is invoked. Also, when opcode `LOG*` is invoked, check whether the token event belongs to token transfer action and store it if so.

## 3. Experiment results
Pass:

- bZx: logic bug, Pump and Arbitrage https://etherscan.io/tx/0xb5c8bd9430b6cc87a0e2fe110ece6bf527fa4f170a4bc8cd032f768fc5219838
- bZx: oracle manipulation https://etherscan.io/tx/0x762881b07feb63c436dee38edd4ff1f7a74c33091e534af56c9f7d49b5ecac15
- Balancer Pool(STA) https://etherscan.io/tx/0x013be97768b702fe8eccef1a40544d5ecb3c1961ad5f87fee4d16fdc08c78106
- Balancer Pool(STONK) https://etherscan.io/tx/0xeb008786a7d230180dbd890c76d6a7735430e836d55729a3ff6e22e254121192
- Eminence https://etherscan.io/tx/0x3503253131644dd9f52802d071de74e456570374d586ddd640159cf6fb9b8ad8
- Harvest Finance(USDC) https://etherscan.io/tx/0x35f8d2f572fceaac9288e5d462117850ef2694786992a8c3f6d02612277b0877
- Value DeFi https://etherscan.io/tx/0x46a03488247425f845e444b9c10b52ba3c14927c687d38287c0faddc7471150a
- Cheese Bank https://etherscan.io/tx/0x600a869aa3a259158310a233b815ff67ca41eab8961a49918c2031297a02f1cc
- Origin Dollar(OUSD) https://etherscan.io/tx/0xe1c76241dda7c5fcf1988454c621142495640e708e3f8377982f55f8cf2a8401
- Warp Finance https://etherscan.io/tx/0x8bb8dc5c7c830bac85fa48acad2505e9300a91c3ff239c9517d0cae33b595090
- Yearn Finance https://etherscan.io/tx/0xf6022012b73770e7e2177129e648980a82aab555f9ac88b8a9cda3ec44b30779

Fail:

- Harvest Finance(USDT) https://etherscan.io/tx/0x0fc6d2ca064fc841bc9b1c1fad1fbb97bcea5c9a1b2b66ef837f1227e06519a6

    Reason: It did multiple initialization function calls before doing flash loan