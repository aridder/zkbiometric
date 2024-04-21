// Copyright 2023 RISC Zero, Inc.
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

//! Generated crate containing the image ID and ELF binary of the build guest.

use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/methods.rs"));

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

#[derive(Serialize, Deserialize, Debug)]
pub struct BiometricRoot {
    #[serde(rename = "biometricIssuer")]
    biometric_issuer: PublicKeyHolder,
    #[serde(rename = "biometricOnboardingCredential")]
    biometric_onboarding_credential: Credential,
    #[serde(rename = "biometricChallengeCredential")]
    biometric_challenge_credential: Credential,
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

#[cfg(test)]
mod tests {
    use std::fs;

    use risc0_zkvm::{default_executor, default_prover, ExecutorEnv};

    use crate::{BiometricRoot, FieldValue, Predicate, Root};
    use crate::Condition::GT;

    #[test]
    fn proves_older_than() {
        let data = fs::read_to_string("./data.json").expect("Unable to read file");

        let root: Root = serde_json::from_str(&data).expect("JSON was not well-formatted");

        let person_credential = root.person_credential;

        let predicate = Predicate {
            field: String::from("date_of_birth"),
            condition: GT,
            value: FieldValue::Int(19791001),
            return_value: String::from("Subject is older than 40 years old"),
        };

        let predicate_list = vec![predicate];

        let public_key_eid = root.eid_issuer.public_key;

        let input = (person_credential.proof.jwt, public_key_eid, predicate_list);

        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session_info = default_executor()
            .execute(env, super::PREDICATE_VERIFIER_ELF)
            .unwrap();
        let result_list: Vec<String> = session_info.journal.decode().unwrap();

        // println!("Issuer: {}", issuer_address);
        println!("Result list: {:?}", result_list);
    }

    #[test]
    fn prove_biometry() {
        let biometric_mock = fs::read_to_string("./biometric_mock.json").expect("Unable to read file");
        let biometric_mock_root: BiometricRoot = serde_json::from_str(&biometric_mock).expect("JSON was not well-formatted");

        let onboarding_biometric_credential = biometric_mock_root.biometric_onboarding_credential;
        let challenge_biometric_credential = biometric_mock_root.biometric_challenge_credential;

        let biometric_issuer = biometric_mock_root.biometric_issuer;

        let input = (onboarding_biometric_credential.proof.jwt, challenge_biometric_credential.proof.jwt, biometric_issuer.public_key);

        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .unwrap();


        let session_info = default_executor()
            .execute(env, super::BIOMETRIC_VERIFIER_ELF)
            .unwrap();
    }

    #[test]
    fn proves_nationality() {
        use crate::Condition::EQ;

        let data = fs::read_to_string("./data.json").expect("Unable to read file");

        let root: Root = serde_json::from_str(&data).expect("JSON was not well-formatted");

        let person_credential = root.person_credential;

        let predicate = Predicate {
            field: String::from("nationality"),
            condition: EQ,
            value: FieldValue::Text(String::from("NO")),
            return_value: String::from("NO"),
        };

        let predicate_list = vec![predicate];

        let public_key_eid = root.eid_issuer.public_key;

        let input = (person_credential.proof.jwt, public_key_eid, predicate_list);

        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session_info = default_executor()
            .execute(env, super::PREDICATE_VERIFIER_ELF)
            .unwrap();
        let issuer: String = session_info.journal.decode().unwrap();

        // println!("Issuer address: {}", issuer_address);
    }

}
