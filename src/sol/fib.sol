// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

contract Fib {
    function fib(uint8 n) private pure returns (uint256) {
        if (n == 0) {
          return 1;
        } else if (n == 1) {
          return 1;
        } else {
          return fib(n-1) + fib(n-2);
        }
    }

    function test(uint8 n) public pure returns (uint256) {
      return fib(n);
    }
}
