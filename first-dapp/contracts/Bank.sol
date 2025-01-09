// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/interfaces/IERC20.sol";

contract Bank {
    mapping(address => uint) public deposited;

    address public immutable token;

    constructor(address _token) {
        token = _token;
    }

    function myBalance() public view  returns(uint balance) {
        balance = deposited[msg.sender] / (10 ** 18);
    }

    function deposit(uint amount) public {
        amount = amount * 10 ** 18;
        require(IERC20(token).transferFrom(msg.sender, address(this), amount), "transfer error");
        deposited[msg.sender] += amount;
    }
}