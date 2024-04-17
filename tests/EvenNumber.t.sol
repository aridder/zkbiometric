// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {RiscZeroCheats} from "risc0/RiscZeroCheats.sol";
import {console2} from "forge-std/console2.sol";
import {Test} from "forge-std/Test.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {VerifierContract} from "../contracts/VerifierContract.sol";
import {Elf} from "./Elf.sol";
import "forge-std/console.sol";

contract EvenNumberTest is RiscZeroCheats, Test {
    VerifierContract public evenNumber;

    function setUp() public {
        IRiscZeroVerifier verifier = deployRiscZeroVerifier();
        evenNumber = new VerifierContract(verifier);

        assertEq(evenNumber.get(), 10);
    }

    function test_SetEven() public {
        uint256 number = 10;
        console.log("address of %s is %s", "evenNumber", address(evenNumber));
        console.logAddress(address(evenNumber));
        assertEq(evenNumber.get(), number);
        (bytes memory journal, bytes32 post_state_digest, bytes memory seal) =
            prove(Elf.PREDICATE_VERIFIER_PATH, abi.encode(number));

//        evenNumber.set(abi.decode(journal, (uint256)), post_state_digest, seal);
//        assertEq(evenNumber.get(), number);
    }

    function test_SetZero() public {
        uint256 number = 10;
        assertEq(evenNumber.get(), number);
//        (bytes memory journal, bytes32 post_state_digest, bytes memory seal) =
//            prove(Elf.PREDICATE_VERIFIER_PATH, abi.encode(number));
//
//        evenNumber.set(abi.decode(journal, (uint256)), post_state_digest, seal);
//        assertEq(evenNumber.get(), number);
    }
}
