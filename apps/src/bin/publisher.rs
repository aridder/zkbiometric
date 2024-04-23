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

// This application demonstrates how to send an off-chain proof request
// to the Bonsai proving service and publish the received proofs directly
// to your deployed app contract.

use alloy_primitives::U256;
use alloy_sol_types::{sol};
use anyhow::Result;
use clap::Parser;

use apps::{BonsaiProver};
use methods::{BIOMETRIC_VERIFIER_ELF};

// `IEvenNumber` interface automatically generated via the alloy `sol!` macro.
sol! {
    interface IVerifierContract {
       function set(string[] result_list, bytes32 post_state_digest, bytes calldata seal);
    }
}

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ethereum chain ID
    #[clap(long)]
    chain_id: u64,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    eth_wallet_private_key: String,

    /// Ethereum Node endpoint.
    #[clap(long)]
    rpc_url: String,

    /// Application's contract address on Ethereum
    #[clap(long)]
    contract: String,

    /// The input to provide to the guest binary
    #[clap(short, long)]
    input: U256,
}

fn main() -> Result<()> {
    env_logger::init();
    // let args = Args::parse();
    //
    // // Create a new `TxSender`.
    // let tx_sender = TxSender::new(
    //     args.chain_id,
    //     &args.rpc_url,
    //     &args.eth_wallet_private_key,
    //     &args.contract,
    // )?;


//    string public biometricOnboardCredentialJwt = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiQmlvbWV0cmljT25ib2FyZGluZ0NyZWRlbnRpYWwiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiZmluZ2VycHJpbnQiOiJwb2FzZGxia2gxMiszOTEwMmFzZGFzZGxraiJ9fSwic3ViIjoiZGlkOmtleTp6Nk1raU1ZVGk5cHFZclliTEdONkRyanlqM0RzZGhWVHViSjl1eWdUTlFmam1jcGIiLCJuYmYiOjE3MTM3MDk5MjgsImlzcyI6ImRpZDprZXk6ejZNa21IcWNhTFBQdFJEUW41Q2NZOGoxZ2lqaFdOZlc2N0Y4OVpUcXRmQnN2aXZqIn0.jCbEWFsSCo7lSXP16TSYevGvb6drwXqzTJL5egvPaqQckjtIw6apCbGHv_vSMDSil5YI66Q0A1CMOyeVNDHoAA";
//    string public biometricChallengeJwt = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiQmlvbWV0cmljQ2hhbGxlbmdlQ3JlZGVudGlhbCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJmaW5nZXJwcmludCI6InBvYXNkbGJraDEyKzM5MTAyYXNkYXNkbGtqIn19LCJzdWIiOiJkaWQ6a2V5Ono2TWtpTVlUaTlwcVlyWWJMR042RHJqeWozRHNkaFZUdWJKOXV5Z1ROUWZqbWNwYiIsIm5iZiI6MTcxMzcwOTkyOCwiaXNzIjoiZGlkOmtleTp6Nk1rbUhxY2FMUFB0UkRRbjVDY1k4ajFnaWpoV05mVzY3Rjg5WlRxdGZCc3ZpdmoifQ.X6v9RqghqiW9tbyFHD1OlDuwvX1Zchy51s8IzhAnPGN-B6U_B_bOyyMgL7UFFx9nQtWN_6pTjgiLAcj-khx6DQ";
//    string public biometricIssuerPublicKey = "6597f4e63517f4f9b8e5c98a0823b7b6b2c0341383da5836771a11b51948ff98";
//    string public userDid = "did:key:z6MkiMYTi9pqYrYbLGN6Drjyj3DsdhVTubJ9uygTNQfjmcpb";


    let onboarding_biometric_credential = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiQmlvbWV0cmljT25ib2FyZGluZ0NyZWRlbnRpYWwiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiZmluZ2VycHJpbnQiOiJwb2FzZGxia2gxMiszOTEwMmFzZGFzZGxraiJ9fSwic3ViIjoiZGlkOmtleTp6Nk1raU1ZVGk5cHFZclliTEdONkRyanlqM0RzZGhWVHViSjl1eWdUTlFmam1jcGIiLCJuYmYiOjE3MTM3MDk5MjgsImlzcyI6ImRpZDprZXk6ejZNa21IcWNhTFBQdFJEUW41Q2NZOGoxZ2lqaFdOZlc2N0Y4OVpUcXRmQnN2aXZqIn0.jCbEWFsSCo7lSXP16TSYevGvb6drwXqzTJL5egvPaqQckjtIw6apCbGHv_vSMDSil5YI66Q0A1CMOyeVNDHoAA";
    let challenge_biometric_credential = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiQmlvbWV0cmljQ2hhbGxlbmdlQ3JlZGVudGlhbCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJmaW5nZXJwcmludCI6InBvYXNkbGliaW1tYXJ";

    let biometric_issuer = "6597f4e63517f4f9b8e5c98a0823b7b6b2c0341383da5836771a11b51948ff98";

    // let input = args.input.abi_encode();
    // let input = (onboarding_biometric_credential.proof.jwt, challenge_biometric_credential.proof.jwt, biometric_issuer.public_key);
    let input = (onboarding_biometric_credential, challenge_biometric_credential, biometric_issuer);

    // Send an off-chain proof request to the Bonsai proving service.
    let (journal, post_state_digest, seal) = BonsaiProver::prove(BIOMETRIC_VERIFIER_ELF, &input)?;

    println!("Journal: {:?}", journal);
    println!("Post state digest: {:?}", post_state_digest);
    println!("Seal: {:?}", seal);

    // let result_list = journal
    //     .into_iter()
    //     .map(|b| (b as char).to_string())
    //     .collect();
    //
    // // Decode the journal. Must match what was written in the guest with
    // // `env::commit_slice`.
    //
    // // Encode the function call for `IEvenNumber.set(x)`.
    // let calldata = IVerifierContract::IVerifierContractCalls::set(IVerifierContract::setCall {
    //     result_list: result_list,
    //     post_state_digest,
    //     seal,
    // })
    // .abi_encode();
    //
    // // Send the calldata to Ethereum.
    // let runtime = tokio::runtime::Runtime::new()?;
    // runtime.block_on(tx_sender.send(calldata))?;
    //
    Ok(())
}
