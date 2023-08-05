// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.3;

contract SimpleStorage {
    mapping (uint => uint) public ids;

    function set(uint key, uint val) public {
        ids[key] = val;
    }
}