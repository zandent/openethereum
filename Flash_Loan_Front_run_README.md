# OpenEthereum Flash Loan Front Run Project

The project is to present a new type of attack at OpenEthereum client end: When assembling next block for Ethereum, flash loan transactions are detected and the beneficiaries are calculated based on ERC20 token and ETH transfers in the transaction. Once the transaction is beneficial for senders, the transaction flash loan actions are copied and signed by front run address as well as a new contract deployment by the address. Miners connected to the client will mine the block containing the front run transactions instead of the original flash loan one.

## 1. Build and Run
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
`crates/ethcore/src/miner/miner.rs: prepare_block` defines picking best transactions, executing them and pushing them into a new block.

`crates/ethcore/src/block.rs: push_transaction` defines creating flash loan detection overlay for the transaction and executing it. Once detecting flash loan transaction and beneficiaries, new contract deployment transaction and flash loan transaction
the block state is reverted to the start point.