// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

contract test {

    struct Test {
        string tag;
        uint256 value1;
        uint256 value2;
    }

    function testing_match(Test memory test_param) private pure returns (uint256) {
        if (keccak256(abi.encode(test_param.tag)) == keccak256("Foo")) {
            return test_param.value1 + test_param.value2;
        } else if (keccak256(abi.encode(test_param.tag)) == keccak256("Bar")) {
            return test_param.value1 - test_param.value2;
        } else {
            return 0;
        }
    }

    function loop(uint256 l) public pure returns (uint8) {
        Test memory foo = Test({tag: "Foo", value1: 10, value2: 5});
        Test memory bar = Test({tag: "Bar", value1: 10, value2: 5});

        for (uint256 i = 0; i < l; i++) {
            if (i % 2 == 0) {
                testing_match(foo);
            } else {
                testing_match(bar);
            }
        }

        return 1;
    }
}
