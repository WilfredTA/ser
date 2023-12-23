// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract SimpleToken {
    mapping(address => uint256) public balances;

    constructor() {
        balances[msg.sender] = 100;
    }

    function give(address recipient, uint256 amt) public {
        balances[recipient] += amt;
    }

}
