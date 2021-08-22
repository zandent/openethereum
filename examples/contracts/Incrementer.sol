pragma solidity >=0.6.0;

contract Incrementer {
    uint256 public number;
    uint256 public balance;

    constructor(uint256 _initialNumber) public payable {
        number = _initialNumber;
        balance = msg.value;
    }

    function increment(uint256 _value) public {
        number = number + _value;
    }

    function set() public payable{
        balance = balance + msg.value;
    }
    function acquire() public payable {
        require(balance >= 30, "no balance in the contract");
        balance = balance - 30;
        payable(msg.sender).transfer(9);
        payable(msg.sender).transfer(10);
        payable(msg.sender).transfer(11);
    }
}