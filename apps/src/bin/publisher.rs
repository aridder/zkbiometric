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
use alloy_sol_types::{sol, SolInterface, SolValue};
use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

use apps::{BonsaiProver, TxSender};
use methods::BIOMETRIC_VERIFIER_ELF;

#[derive(Serialize, Deserialize, Debug)]
struct PublicKeyHolder {
    public_key: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Proof {
    #[serde(rename = "type")]
    proof_type: String,
    jwt: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Credential {
    #[serde(rename = "credentialSubject")]
    credential_subject: serde_json::Value,
    issuer: serde_json::Value,
    #[serde(rename = "type")]
    types: Vec<String>,
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(rename = "issuanceDate")]
    issuance_date: String,
    proof: Proof,
}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    #[serde(rename = "bidSize")]
    bid_size: u32,
    #[serde(rename = "eidIssuer")]
    eid_issuer: PublicKeyHolder,
    bank: PublicKeyHolder,
    person: PublicKeyHolder,
    #[serde(rename = "personCredential")]
    person_credential: Credential,
    #[serde(rename = "houseLoanCredential")]
    house_loan_credential: Credential,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Condition {
    LT,
    GT,
    EQ,
    NEQ,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum FieldValue {
    Int(u32),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Predicate {
    field: String,
    condition: Condition,
    value: FieldValue,
    return_value: String,
}

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
    let args = Args::parse();

    // Create a new `TxSender`.
    let tx_sender = TxSender::new(
        args.chain_id,
        &args.rpc_url,
        &args.eth_wallet_private_key,
        &args.contract,
    )?;

    // ABI encode the input for the guest binary, to match what the `is_even` guest
    let input = args.input.abi_encode();

    // Send an off-chain proof request to the Bonsai proving service.
    let (journal, post_state_digest, seal) = BonsaiProver::prove(BIOMETRIC_VERIFIER_ELF, &input)?;

    let result_list = journal
        .into_iter()
        .map(|b| (b as char).to_string())
        .collect();

    // Decode the journal. Must match what was written in the guest with
    // `env::commit_slice`.

    // Encode the function call for `IEvenNumber.set(x)`.
    let calldata = IVerifierContract::IVerifierContractCalls::set(IVerifierContract::setCall {
        result_list: result_list,
        post_state_digest,
        seal,
    })
    .abi_encode();

    // Send the calldata to Ethereum.
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(tx_sender.send(calldata))?;

    Ok(())
}
