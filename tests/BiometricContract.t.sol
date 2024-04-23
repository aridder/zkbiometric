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
    VerifierContract public biometricVerifier;

    string public IMAGE_ID = "";
    string public SEAL = "";
    string public POST_STATE_DIGEST = "";
    string public JOURNAL = "did:key:z6MkiMYTi9pqYrYbLGN6Drjyj3DsdhVTubJ9uygTNQfjmcpb";

//    string public biometricOnboardCredentialJwt = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiQmlvbWV0cmljT25ib2FyZGluZ0NyZWRlbnRpYWwiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiZmluZ2VycHJpbnQiOiJwb2FzZGxia2gxMiszOTEwMmFzZGFzZGxraiJ9fSwic3ViIjoiZGlkOmtleTp6Nk1raU1ZVGk5cHFZclliTEdONkRyanlqM0RzZGhWVHViSjl1eWdUTlFmam1jcGIiLCJuYmYiOjE3MTM3MDk5MjgsImlzcyI6ImRpZDprZXk6ejZNa21IcWNhTFBQdFJEUW41Q2NZOGoxZ2lqaFdOZlc2N0Y4OVpUcXRmQnN2aXZqIn0.jCbEWFsSCo7lSXP16TSYevGvb6drwXqzTJL5egvPaqQckjtIw6apCbGHv_vSMDSil5YI66Q0A1CMOyeVNDHoAA";
//    string public biometricChallengeJwt = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiQmlvbWV0cmljQ2hhbGxlbmdlQ3JlZGVudGlhbCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJmaW5nZXJwcmludCI6InBvYXNkbGJraDEyKzM5MTAyYXNkYXNkbGtqIn19LCJzdWIiOiJkaWQ6a2V5Ono2TWtpTVlUaTlwcVlyWWJMR042RHJqeWozRHNkaFZUdWJKOXV5Z1ROUWZqbWNwYiIsIm5iZiI6MTcxMzcwOTkyOCwiaXNzIjoiZGlkOmtleTp6Nk1rbUhxY2FMUFB0UkRRbjVDY1k4ajFnaWpoV05mVzY3Rjg5WlRxdGZCc3ZpdmoifQ.X6v9RqghqiW9tbyFHD1OlDuwvX1Zchy51s8IzhAnPGN-B6U_B_bOyyMgL7UFFx9nQtWN_6pTjgiLAcj-khx6DQ";
//    string public biometricIssuerPublicKey = "6597f4e63517f4f9b8e5c98a0823b7b6b2c0341383da5836771a11b51948ff98";
//    string public userDid = "did:key:z6MkiMYTi9pqYrYbLGN6Drjyj3DsdhVTubJ9uygTNQfjmcpb";

    function setUp() public {
        IRiscZeroVerifier verifier = deployRiscZeroVerifier();
        biometricVerifier = new VerifierContract(verifier);

//        assertEq(biometricVerifier.get(), 10);
    }

    function test_checkBiometric() public {
//        bytes memory inputData = abi.encode(
//            biometricOnboardCredentialJwt,
//            biometricChallengeJwt,
//            biometricIssuerPublicKey
//        );
//
//        console.logBytes(inputData);
//
//
//        (bytes memory journal, bytes32 post_state_digest, bytes memory seal) = prove(Elf.BIOMETRIC_VERIFIER_PATH, inputData);
//
//        assertEq(biometricVerifier.get(), 10);


//        evenNumber.set(abi.decode(journal, (uint256)), post_state_digest, seal);
//        assertEq(evenNumber.get(), number);
    }

}
