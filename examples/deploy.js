
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
web3.eth.getBalance("0xB90168C8CBcd351D069ffFdA7B71cd846924d551").then(console.log);


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
    console.log('Attempting to deploy from account:', addressFrom);

 const incrementer = new web3.eth.Contract(abi);
 const incrementerTx = incrementer.deploy({
       data: code,
       arguments: [5],
    });

var new_nonce = 0;

await web3.eth.getTransactionCount(addressFrom, function(error, txCount) {
        new_nonce = txCount;
     }); 
console.log("now nonce is :", new_nonce);
 const createTransaction = await web3.eth.accounts.signTransaction(
       {
          from: addressFrom,
          data: incrementerTx.encodeABI(),
          gas: '1000000',
          nonce: new_nonce + 1,
          value: '100',
       },
       privKey
    );
 const createReceipt = await web3.eth.sendSignedTransaction(
       createTransaction.rawTransaction
    );
    console.log('Contract deployed at address', createReceipt.contractAddress);

 };
deployContract();