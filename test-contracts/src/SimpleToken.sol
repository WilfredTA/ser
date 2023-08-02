// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract SimpleToken {
    mapping(address => uint256) public balances;

    constructor() {
        balances[msg.sender] = 100;
    }

    function transfer(address recipient, uint256 amt) public {
        require(balances[msg.sender] >= amt);
        balances[msg.sender] -= amt;
        balances[recipient] += amt;
    }

    function getBalance(address owner) external view returns (uint256) {
        return balances[owner];
    }
}
