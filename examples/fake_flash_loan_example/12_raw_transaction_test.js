
let fs = require("fs");
const Web3 = require('web3')
const web3 = new Web3("http://localhost:8545")
const privKey = '4d5db4107d237df6a3d58ee5f70ae63d73d7658d4026f2eefd2f204c81682cb7'; // Genesis private key
const addressFrom = '0x00a329c0648769a73afac7f9381e08fb43dbea72';
const privKeyTo = '376a0e785c3b3f2b254d312bbc87ad87102081672594400363a5aceb4e0915e3';
const addressTo = '0x15218E401162e4a093806345D4F31a7A250637Fd';
const flashloankey = "ad0ad85b628caae0aa45653da3e9910166376e0dd94b30696b5fa8327786c735";
const flashloanaddress = "0x1d00652d5E40173ddaCdd24FD8Cdb12228992755";

// Create transaction
// const normalTx = async (nonce_, val_) => {
//    console.log(
//       `Attempting to make transaction from ${addressFrom} to ${addressTo}`
//    );
//    const createTransaction = await web3.eth.accounts.signTransaction(
//       {
//          from: addressFrom,
//          to: flashloanaddress,
//          value: val_,
//          gas: '21000',
//          nonce: nonce_
//       },
//       privKey
//    );
//    const createReceipt = await web3.eth.sendSignedTransaction(
//       createTransaction.rawTransaction
//    );
//    console.log(
//       `Transaction successful with hash: ${createReceipt.transactionHash}`
//    );
// };
// normalTx(0, 1);
// normalTx(1, 1);
// normalTx(2, 2);
// normalTx(3, 3);
// normalTx(4, 4);
// normalTx(5, 5);
// normalTx(6, 6);
// normalTx(7, 7);
// web3.eth.getBalance(flashloanaddress).then(console.log);


// // Read the compiled contract code
// // Compile with
// // solc erc.sol --combined-json abi,asm,ast,bin,bin-runtime,compact-format,devdoc,hashes,interface,metadata,opcodes,srcmap,srcmap-runtime,storage-layout,userdoc --overwrite -o out_erc
// let source = fs.readFileSync("contracts/out_erc/combined.json");
// let contracts = JSON.parse(source)["contracts"]['erc.sol:ERC20Basic'];

// // ABI description as JSON structure
// let abi = JSON.parse(contracts.abi);

// // Smart contract EVM bytecode as hex
// let code = '0x' + contracts.bin;

// Deploy contract
// const deployContract = async () => {
//    console.log('Attempting to deploy from account:', addressFrom);

//    var new_nonce = 0;

//    await web3.eth.getTransactionCount(addressFrom, function(error, txCount) {
//       new_nonce = txCount;
//    }); 
//    console.log("now nonce is :", new_nonce);
//    const createTransaction = await web3.eth.accounts.signTransaction(
//       {
//          from: addressFrom,
//          to: '0x62494b3ed9663334e57f23532155ea0575c487c5',
//          data: '0x3a27652300000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000180000000000000000000000000acd43e627e64355f1861cec6d3a6688b31a6f9520000000000000000000000000000000000000000001be0bdf10f9d4200000000000000000000000000000000000000000000000000000000000079df4e6660000000000000000000000000000000000000000000004cf0a8462078f8000000000000000000000000000000000000000000000000000000000000177df9542f790000000000000000000000000000000000000000000000000000ac35b9dd455c000000000000000000000000000000000000000000000000000000000000000500000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000',
//          gas: '5000000',
//          nonce: new_nonce,
//       },
//       privKey
//    );
//    const createReceipt = await web3.eth.sendSignedTransaction(
//       createTransaction.rawTransaction
//    );
//    console.log('Contract deployed receipt', createReceipt);
// };
// deployContract();
const deployContract = async () => {
   const createReceipt = await web3.eth.getCode("0x5fe6ea3cde244c04ef2b02df66b16e66df58c552");
   console.log('Contract deployed receipt', createReceipt);
}
deployContract();