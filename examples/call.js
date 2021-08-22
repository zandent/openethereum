
let fs = require("fs");
const Web3 = require('web3')
const rpcURL = '' // Your RCkP URL goes here
const web3 = new Web3("http://localhost:8545")
web3.eth.getBlockNumber(function (error, result) {console.log(result)});
web3.eth.getAccounts(function (error, result) {console.log(result)});  
web3.eth.getGasPrice().then(console.log);
//web3 = new Web3(new Web3.providers.HttpProvider("http://localhost:8546"));
//web3.eth.getAccounts().then(console.log);
//const address = '0x0000000000000000000000000000000000000000' // Your account address goes here
//web3.eth.getBalance(address, (err, wei) => { balance = web3.utils.fromWei(wei, 'ether') })
// Variables definition
const privKey =
 '4d5db4107d237df6a3d58ee5f70ae63d73d7658d4026f2eefd2f204c81682cb7'; // Genesis private key
const addressFrom = '0x00a329c0648769a73afac7f9381e08fb43dbea72';
const addressTo = '0xB90168C8CBcd351D069ffFdA7B71cd846924d551';
const contractAddr = "0xb4c79daB8f259C7Aee6E5b2Aa729821864227e84";
// Create transaction
const deploy = async () => {
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

// Deploy transaction
const createReceipt = await web3.eth.sendSignedTransaction(
    createTransaction.rawTransaction
 );
 console.log(
    `Transaction successful with hash: ${createReceipt.transactionHash}`
 );
};

deploy();


// Read the compiled contract code
// Compile with
// solc Incrementer.sol --combined-json abi,asm,ast,bin,bin-runtime,compact-format,devdoc,hashes,interface,metadata,opcodes,srcmap,srcmap-runtime,storage-layout,userdoc --overwrite -o out
let source = fs.readFileSync("contracts/out/combined.json");
let contracts = JSON.parse(source)["contracts"]['Incrementer.sol:Incrementer'];

// ABI description as JSON structure
let abi = JSON.parse(contracts.abi);

// Smart contract EVM bytecode as hex
let code = '0x' + contracts.bin;

// Create Contract proxy class
let contract = new web3.eth.Contract(abi);


// Deploy contract
const deployContract = async () => {
web3.eth.getBalance(addressFrom).then(console.log);
var new_nonce = 0;
const incrementerToCall = new web3.eth.Contract(abi, contractAddr);

// var data = await incrementerToCall.methods
//       .balance()
//       .call({ from: addressFrom });
//    console.log(`The current balance is: ${data}`);
//    await new Promise(r => setTimeout(r, 2000));
var _value = 3;
const encoded = incrementerToCall.methods.acquire().encodeABI();
console.log(`Calling the increment by ${_value} function in contract at address ${contractAddr}`);
await web3.eth.getTransactionCount(addressFrom, function(error, txCount) {
   new_nonce = txCount;
}); 
console.log("now nonce is :", new_nonce);
   const callTransaction = await web3.eth.accounts.signTransaction(
      {
         from: addressFrom,
         to: contractAddr,
         data: encoded,
         gas: '1000000',
         nonce: new_nonce + 1,
      },
      privKey
   );
   const CallReceipt = await web3.eth.sendSignedTransaction(
      callTransaction.rawTransaction
   );
   console.log('Contract call receipt: ', CallReceipt); 
   web3.eth.getBalance(addressFrom).then(console.log);
//    await new Promise(r => setTimeout(r, 2000));
//    data = await incrementerToCall.methods
//    .balance()
//    .call({ from: addressFrom });
// console.log(`The current balance is: ${data}`);
 };
deployContract();