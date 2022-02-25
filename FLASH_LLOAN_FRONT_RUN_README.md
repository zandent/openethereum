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
4. If it is beneficial, assemble contract deployment transaction and flash loan transaction denoting `a` and `b`
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