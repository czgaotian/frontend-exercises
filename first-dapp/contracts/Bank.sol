// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/interfaces/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract Bank {
    mapping(address => uint256) public deposited;

    address public immutable token;

    constructor(address _token) {
        token = _token;
    }

    modifier requireBalance(uint256 amount) {
        amount = amount * 10**18;
        uint256 balance = deposited[msg.sender];
        require(amount <= balance, "the amount more than bank of balance");
        _;
    }

    function myBalance() public view returns (uint256 balance) {
        balance = deposited[msg.sender] / (10**18);
    }

    function deposit(uint256 amount) public {
        amount = amount * 10**18;
        require(
            IERC20(token).transferFrom(msg.sender, address(this), amount),
            "transfer error"
        );
        deposited[msg.sender] += amount;
    }

    function withdraw(uint256 amount) external requireBalance(amount) {
        amount = amount * 10**18;
        // require(IERC20(token).transfer(msg.sender, amount), "transfer error");
        SafeERC20.safeTransfer(IERC20(token), msg.sender, amount);
        deposited[msg.sender] -= amount;
    }

    function transfer(address to, uint256 amount)
        public
        requireBalance(amount)
    {
        amount = amount * 10**18;
        deposited[msg.sender] -= amount;
        deposited[to] += amount;
    }
}
