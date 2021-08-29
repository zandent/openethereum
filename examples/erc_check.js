
let fs = require("fs");
const Web3 = require('web3')
const web3 = new Web3("http://localhost:8545")
const privKey = '4d5db4107d237df6a3d58ee5f70ae63d73d7658d4026f2eefd2f204c81682cb7'; // Genesis private key
const addressFrom = '0x00a329c0648769a73afac7f9381e08fb43dbea72';
const privKeyTo = '376a0e785c3b3f2b254d312bbc87ad87102081672594400363a5aceb4e0915e3'
const addressTo = '0x15218E401162e4a093806345D4F31a7A250637Fd';
const contractAddr = '0x3f85D0b6119B38b7E6B119F7550290fec4BE0e3c'
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
let source = fs.readFileSync("contracts/out_erc/combined.json");
let contracts = JSON.parse(source)["contracts"]['erc.sol:ERC20Basic'];

// ABI description as JSON structure
let abi = JSON.parse(contracts.abi);

// Smart contract EVM bytecode as hex
let code = '0x' + contracts.bin;


// Deploy contract
const callContract = async () => {
const incrementerToCall = new web3.eth.Contract(abi, contractAddr);
incrementerToCall.methods.symbol().call({from: addressFrom, gas: '1000000'}).then(console.log);

 };
 callContract();