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
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

use serde::{Deserialize, Serialize};

use credential_verifier::{GenericCredential, PublicKeyHolder};

#[derive(Serialize, Deserialize, Debug)]
pub struct BiometricRoot {
    #[serde(rename = "biometricIssuer")]
    biometric_issuer: PublicKeyHolder,
    #[serde(rename = "biometricOnboardingCredential")]
    biometric_onboarding_credential: GenericCredential,
    #[serde(rename = "biometricChallengeCredential")]
    biometric_challenge_credential: GenericCredential,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use risc0_zkvm::{default_executor, ExecutorEnv};

    use crate::{BiometricRoot};

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


        let subject_did: String = session_info.journal.decode().unwrap();

        print!("Subject DID: {}", subject_did);

        assert_eq!(subject_did, "did:key:z6MkiMYTi9pqYrYbLGN6Drjyj3DsdhVTubJ9uygTNQfjmcpb")
    }
}

