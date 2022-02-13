
let fs = require("fs");
const Web3 = require('web3')
const web3 = new Web3("http://localhost:8545")
const privKey = '4d5db4107d237df6a3d58ee5f70ae63d73d7658d4026f2eefd2f204c81682cb7'; // Genesis private key
const addressFrom = '0x00a329c0648769a73afac7f9381e08fb43dbea72';
const privKeyTo = '376a0e785c3b3f2b254d312bbc87ad87102081672594400363a5aceb4e0915e3';
const addressTo = '0x15218E401162e4a093806345D4F31a7A250637Fd';

//the parameters for deploy
const token_a = "0xb4c79daB8f259C7Aee6E5b2Aa729821864227e84";
const amt_a = 100;
const token_b = "0xee35211C4D9126D520bBfeaf3cFee5FE7B86F221";
const amt_b = 200;
const ratio_a_to_b = 2;

// Create transaction
const normalTx = async () => {
   console.log(
      `Attempting to make transaction from ${addressFrom} to ${addressTo}`
   );
   const createTransaction = await web3.eth.accounts.signTransaction(
      {
         from: addressFrom,
         to: addressTo,
         value: web3.utils.toWei('10', 'ether'),
         gas: '21000',
      },
      privKey
   );
   const createReceipt = await web3.eth.sendSignedTransaction(
      createTransaction.rawTransaction
   );
   console.log(
      `Transaction successful with hash: ${createReceipt.transactionHash}`
   );
};
normalTx();
web3.eth.getBalance(addressTo).then(console.log);


// Read the compiled contract code
// Compile with
// solc erc.sol --combined-json abi,asm,ast,bin,bin-runtime,compact-format,devdoc,hashes,interface,metadata,opcodes,srcmap,srcmap-runtime,storage-layout,userdoc --overwrite -o out_erc
let source = fs.readFileSync("contracts/out_fake_swap/combined.json");
let contracts = JSON.parse(source)["contracts"]['fake_swap.sol:fakeSwap'];

// ABI description as JSON structure
let abi = JSON.parse(contracts.abi);

// Smart contract EVM bytecode as hex
let code = '0x' + contracts.bin;

// Deploy contract
const deployContract = async () => {
   console.log('Attempting to deploy from account:', addressFrom);
   // Create Contract proxy class
   const contract = new web3.eth.Contract(abi);
   const contractTx = contract.deploy({
      data: code,
      arguments: [token_a, amt_a, token_b, amt_b, ratio_a_to_b],
   });
   var new_nonce = 0;
   await web3.eth.getTransactionCount(addressFrom, function(error, txCount) {
      new_nonce = txCount;
   }); 
   console.log("now nonce is :", new_nonce);
   const createTransaction = await web3.eth.accounts.signTransaction(
      {
         from: addressFrom,
         data: contractTx.encodeABI(),
         gas: '5000000',
         nonce: new_nonce + 1,
      },
      privKey
   );
   const createReceipt = await web3.eth.sendSignedTransaction(
      createTransaction.rawTransaction
   );
   console.log('Contract deployed receipt', createReceipt);
};
deployContract();