let fs = require("fs");
const Web3 = require('web3');
const web3 = new Web3("http://localhost:8558");
const privKeyTo = '376a0e785c3b3f2b254d312bbc87ad87102081672594400363a5aceb4e0915e3';
//web3.eth.getBalance(flashloanaddress).then(console.log);
var addressFr = '0x8308cace1cf1d9c03b565b96eec5eb92758cfc1b';
var contractAddr = '0x4f92e5038e400383d5827681ef9951e7f3beb33d';
const addressfake = '0x15218E401162e4a093806345D4F31a7A250637Fd';
web3.eth.getBlockNumber().then(console.log);
//web3.eth.getBlock(12476307).then(console.log);
// const callContract = async () => {
//     var new_nonce = 0;
//     await web3.eth.getTransactionCount(addressFr, function(error, txCount) {
//        new_nonce = txCount;
//     }); 
//     console.log("now nonce is :", new_nonce);
//     const callTransaction = await web3.eth.accounts.signTransaction(
//        {
//           from: addressFr,
//           to: contractAddr,
//           data: "0xcf94b15d00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000020000000000000000000000001e0447b19bb6ecfdae1e4ae1694b0c3659614e4e000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000028d6cb0f9b21862400000000000000000000000000000000000000000000000028d6cb0f9b21862400000000000000000000000000000000000000000000000000fffb3ba60401f40000000000000000000000000000000000000000000000000000000000000140000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000020000000000000000000000004304ae5fd14cec2299caee4e9a4afbedd046d6120000000000000000000000004304ae5fd14cec2299caee4e9a4afbedd046d612000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000080fb784b7ed66730e8b1dbd9820afd29931aab030000000000000000000000000000000000000000000000000000000000000002000000000000000000000000f11da347012b9752931202a40ed1328982551f5c000000000000000000000000f11da347012b9752931202a40ed1328982551f5c00000000000000000000000080fb784b7ed66730e8b1dbd9820afd29931aab03000000000000000000000000514910771af9ca656af840dff83e8264ecf986ca00000000000000000000000000000000000000000000000000000000000000010000000000000000000000007a250d5630b4cf539739df2c5dacb4c659f2488d000000000000000000000000a2107fa5b38d9bbd2c461d6edf11b11a50f6b974000000000000000000000000514910771af9ca656af840dff83e8264ecf986ca000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
//           gas: '900000',
//           nonce: new_nonce,
//           gasPrice: 0,
//        },
//        privKeyTo
//     );
//     const CallReceipt = await web3.eth.sendSignedTransaction(
//        callTransaction.rawTransaction
//     );
//     console.log('Contract call receipt: ', CallReceipt);
//  };
 //callContract();
// addressFr = '0xf7c73741f9bc91e043edcb3c2305f15b5919632b';
// contractAddr = '0x7a250d5630b4cf539739df2c5dacb4c659f2488d';
//  const callContract2 = async () => {
//    var new_nonce = 0;
//    await web3.eth.getTransactionCount(addressFr, function(error, txCount) {
//       new_nonce = txCount;
//    }); 
//    console.log("now nonce is :", new_nonce);
//    const callTransaction = await web3.eth.accounts.signTransaction(
//       {
//          from: addressFr,
//          to: contractAddr,
//          data: "0x38ed17390000000000000000000000000000000000000000000000981abb5f771dd327bb000000000000000000000000000000000000000000000000be0979bac956480000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f7c73741f9bc91e043edcb3c2305f15b5919632b000000000000000000000000000000000000000000000000000000005fdf3a1900000000000000000000000000000000000000000000000000000000000000020000000000000000000000000b38210ea11411557c13457d4da7dc6ea731b88a000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
//          gas: '900000',
//          nonce: new_nonce+1,
//          gasPrice: 0,
//       },
//       privKeyTo
//    );
//    const CallReceipt = await web3.eth.sendSignedTransaction(
//       callTransaction.rawTransaction
//    );
//    console.log('Contract call receipt: ', CallReceipt);
// };
// callContract2();

// addressFr = '0xf7a8f04c7fe7c8a6ed692bdf5ee1658559cbe7dc';
// contractAddr = '0x8438d64da58772e9f7fceaa1506ba300f935abbd';
//  const callContract3 = async () => {
//    var new_nonce = 0;
//    await web3.eth.getTransactionCount(addressFr, function(error, txCount) {
//       new_nonce = txCount;
//    }); 
//    console.log("now nonce is :", new_nonce);
//    const callTransaction = await web3.eth.accounts.signTransaction(
//       {
//          from: addressFr,
//          to: contractAddr,
//          data: "0x362a3fad000000000000000000000000f7a8f04c7fe7c8a6ed692bdf5ee1658559cbe7dc",
//          gas: '900000',
//          nonce: new_nonce,
//          gasPrice: 0,
//       },
//       privKeyTo
//    );
//    const CallReceipt = await web3.eth.sendSignedTransaction(
//       callTransaction.rawTransaction
//    );
//    console.log('Contract call receipt: ', CallReceipt);
// };
// callContract3();

// //https://etherscan.io/tx/0x7fc5ebc647e656812a98ef595c80a2a164141216eee0669ba8222c4c3707b7f0
// addressFr = '0x05656f7544c080e728c70fd91fc574e6f842501e';
// contractAddr = '0x7a250d5630b4cf539739df2c5dacb4c659f2488d';
//  const callContract3 = async () => {
//    var new_nonce = 0;
//    await web3.eth.getTransactionCount(addressFr, function(error, txCount) {
//       new_nonce = txCount;
//    }); 
//    console.log("now nonce is :", new_nonce);
//    const callTransaction = await web3.eth.accounts.signTransaction(
//       {
//          from: addressFr,
//          to: contractAddr,
//          data: "0x7ff36ab5000000000000000000000000000000000000000000000000499bdb541cfa94dc000000000000000000000000000000000000000000000000000000000000008000000000000000000000000005656f7544c080e728c70fd91fc574e6f842501e000000000000000000000000000000000000000000000000000000005fdf3c480000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000004a7adcb083fe5e3d6b58edc3d260e2e61668e7a2",
//          gas: '900000',
//          nonce: new_nonce,
//          gasPrice: 0,
//          value: '4000000000000000000',
//       },
//       privKeyTo
//    );
//    const CallReceipt = await web3.eth.sendSignedTransaction(
//       callTransaction.rawTransaction
//    );
//    console.log('Contract call receipt: ', CallReceipt);
// };
// callContract3();
// //https://etherscan.io/tx/0xdb73481e6d1eee04f5548b7217d7df6ed9397910d664e4fbc04a4d011d211b90
// addressFr = '0xbf5ae133b9a0fc1a07952a7df2afa21f7f69ef58';
// contractAddr = '0x7a250d5630b4cf539739df2c5dacb4c659f2488d';
//  const callContract3 = async () => {
//    var new_nonce = 0;
//    await web3.eth.getTransactionCount(addressFr, function(error, txCount) {
//       new_nonce = txCount;
//    }); 
//    console.log("now nonce is :", new_nonce);
//    const callTransaction = await web3.eth.accounts.signTransaction(
//       {
//          from: addressFr,
//          to: contractAddr,
//          data: "0x38ed1739000000000000000000000000000000000000000000000033635140fd9966000000000000000000000000000000000000000000000000000000000000dfdf696800000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000bf5ae133b9a0fc1a07952a7df2afa21f7f69ef580000000000000000000000000000000000000000000000000000000070a760e70000000000000000000000000000000000000000000000000000000000000002000000000000000000000000ee9801669c6138e84bd50deb500827b776777d28000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7",
//          gas: '900000',
//          nonce: new_nonce,
//          gasPrice: 0,
//       },
//       privKeyTo
//    );
//    const CallReceipt = await web3.eth.sendSignedTransaction(
//       callTransaction.rawTransaction
//    );
//    console.log('Contract call receipt: ', CallReceipt);
// };
// callContract3();
// //https://etherscan.io/tx/0xaa2c991d233e6787bbcab7862748f6ef07ebb42f045789e34889db34df0dd905 NOT WORK!
// addressFr = '0x8308cace1cf1d9c03b565b96eec5eb92758cfc1b';
// contractAddr = '0x4f92e5038e400383d5827681ef9951e7f3beb33d';
// web3.eth.getBalance(addressFr).then(console.log);
//  const callContract3 = async () => {
//    var new_nonce = 0;
//    await web3.eth.getTransactionCount(addressFr, function(error, txCount) {
//       new_nonce = txCount;
//    }); 
//    console.log("now nonce is :", new_nonce);
//    const callTransaction = await web3.eth.accounts.signTransaction(
//       {
//          from: addressFr,
//          to: contractAddr,
//          data: "0xcf94b15d00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000020000000000000000000000001e0447b19bb6ecfdae1e4ae1694b0c3659614e4e000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000028d6cb0f9b21862400000000000000000000000000000000000000000000000028d6cb0f9b21862400000000000000000000000000000000000000000000000000fffb3ba60401f40000000000000000000000000000000000000000000000000000000000000140000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000020000000000000000000000004304ae5fd14cec2299caee4e9a4afbedd046d6120000000000000000000000004304ae5fd14cec2299caee4e9a4afbedd046d612000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000080fb784b7ed66730e8b1dbd9820afd29931aab030000000000000000000000000000000000000000000000000000000000000002000000000000000000000000f11da347012b9752931202a40ed1328982551f5c000000000000000000000000f11da347012b9752931202a40ed1328982551f5c00000000000000000000000080fb784b7ed66730e8b1dbd9820afd29931aab03000000000000000000000000514910771af9ca656af840dff83e8264ecf986ca00000000000000000000000000000000000000000000000000000000000000010000000000000000000000007a250d5630b4cf539739df2c5dacb4c659f2488d000000000000000000000000a2107fa5b38d9bbd2c461d6edf11b11a50f6b974000000000000000000000000514910771af9ca656af840dff83e8264ecf986ca000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
//          gas: '900000',
//          nonce: new_nonce,
//          gasPrice: 0,
//       },
//       privKeyTo
//    );
//    const CallReceipt = await web3.eth.sendSignedTransaction(
//       callTransaction.rawTransaction
//    );
//    console.log('Contract call receipt: ', CallReceipt);
// };
// callContract3();
//https://etherscan.io/tx/0xb15eb49bab0fdc69d7418da1cb0697112c74c35e57fa8e8c5a004fff417ecde8 NOT WORK
// addressFr = '0xf034f41cbe726666d62fddb7293c62fecaa78eee';
// contractAddr = '0x1deeee4a6078a776e7122ed8024abfabbc4a8bf4';
// web3.eth.getBalance(addressFr).then(console.log);
//  const callContract3 = async () => {
//    var new_nonce = 0;
//    await web3.eth.getTransactionCount(addressFr, function(error, txCount) {
//       new_nonce = txCount;
//    }); 
//    console.log("now nonce is :", new_nonce);
//    const callTransaction = await web3.eth.accounts.signTransaction(
//       {
//          from: addressFr,
//          to: contractAddr,
//          data: "0x1cff79cd000000000000000000000000f15f41f63c377461a718293e125ffceb3eedee1900000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000064329b8f590000000000000000000000006b175474e89094c44da98b954eedeac495271d0f0000000000000000000000006b175474e89094c44da98b954eedeac495271d0f0000000000000000000000000000000000000000000000006043bf53bd19db7800000000000000000000000000000000000000000000000000000000",
//          gas: '900000',
//          nonce: new_nonce,
//          gasPrice: 0,
//       },
//       privKeyTo
//    );
//    const CallReceipt = await web3.eth.sendSignedTransaction(
//       callTransaction.rawTransaction
//    );
//    console.log('Contract call receipt: ', CallReceipt);
// };
// callContract3();
//https://etherscan.io/tx/0x1345b1d4cfb105034d8040baf9d6b8d4ea3858d17ba8b24d762238fd65b57c8b
addressFr = '0x39277f3fec62330c6cded4bb2ad8aeafa8f659b5';
contractAddr = '0x5c4ea81583a6183dd2517307f4377f23ff4d4176';
web3.eth.getBalance(addressFr).then(console.log);
 const callContract3 = async () => {
   var new_nonce = 0;
   await web3.eth.getTransactionCount(addressFr, function(error, txCount) {
      new_nonce = txCount;
   }); 
   console.log("now nonce is :", new_nonce);
   const callTransaction = await web3.eth.accounts.signTransaction(
      {
         from: addressFr,
         to: contractAddr,
         data: "0x28967cdc0000000000000000000000000000000000000000000000a2a15d09519be00000",
         gas: '900000',
         nonce: new_nonce,
         gasPrice: 0,
      },
      privKeyTo
   );
   const CallReceipt = await web3.eth.sendSignedTransaction(
      callTransaction.rawTransaction
   );
   console.log('Contract call receipt: ', CallReceipt);
};
callContract3();