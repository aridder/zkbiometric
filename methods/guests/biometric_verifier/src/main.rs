#![no_main]

use ed25519_dalek::VerifyingKey;
use jwt_compact::{alg::*, prelude::*, Token, UntrustedToken};
// If you want to try std support, also update the guests Cargo.toml file
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

risc0_zkvm::guest::entry!(main);

#[derive(Serialize, Deserialize, Debug)]
struct PublicKeyHolder {
    public_key: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct GenericCredential {
    #[serde(rename = "credentialSubject")]
    credential_subject: serde_json::Value,
    // Flexible subject
    issuer: Issuer,
    #[serde(rename = "type")]
    types: Vec<String>,
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(rename = "issuanceDate")]
    issuance_date: String,
    proof: Proof,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Issuer {
    id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Proof {
    #[serde(rename = "type")]
    proof_type: String,
    jwt: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CredentialClaims {
    vc: CredentialSubjectClaims,
    sub: String,
    iss: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct CredentialSubjectClaims {
    #[serde(rename = "credentialSubject")]
    credential_subject: serde_json::Value,
    #[serde(rename = "type")]
    types: Vec<String>,
    #[serde(rename = "@context")]
    context: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiometricRoot {
    #[serde(rename = "biometricIssuer")]
    biometric_issuer: PublicKeyHolder,
    #[serde(rename = "biometricOnboardingCredential")]
    biometric_onboarding_credential: GenericCredential,
    #[serde(rename = "biometricChallengeCredential")]
    biometric_challenge_credential: GenericCredential,
}


pub fn main() {
    let (onboarding_biometric_credential_jwt, challenge_biometric_credential_jwt, onboarding_issuer_public_key): (String, String, String) = env::read();

    let bytes_pk_eid_issuer: &[u8; 32] = &hex::decode(&onboarding_issuer_public_key).unwrap().try_into().unwrap();

    let verifying_key_eid_issuer = VerifyingKey::from_bytes(&bytes_pk_eid_issuer).unwrap();

    let onboarding_biometric_credential = UntrustedToken::new(&onboarding_biometric_credential_jwt).unwrap();
    let challenge_biometric_credential = UntrustedToken::new(&challenge_biometric_credential_jwt).unwrap();

    let onboarding_claims: Token<CredentialClaims> = Ed25519.validator(&verifying_key_eid_issuer).validate(&onboarding_biometric_credential).unwrap();
    let challenge_claims: Token<CredentialClaims> = Ed25519.validator(&verifying_key_eid_issuer).validate(&challenge_biometric_credential).unwrap();

    let challenge_subject = &challenge_claims.claims().custom.sub;
    let onboarding_subject = &onboarding_claims.claims().custom.sub;

    let claims = &onboarding_claims.claims().custom;
    let challenge_claims = &challenge_claims.claims().custom;

    let onboarding_biometric_fingerprint = &claims.vc.credential_subject["fingerprint"];
    let challenge_biometric_fingerprint = &challenge_claims.vc.credential_subject["fingerprint"];

    assert!(onboarding_biometric_fingerprint == challenge_biometric_fingerprint);
    assert!(onboarding_subject == challenge_subject);

    env::commit(&onboarding_subject)
}
