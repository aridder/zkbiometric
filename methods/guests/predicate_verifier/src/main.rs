#![no_main]

use ed25519_dalek::VerifyingKey;
use jwt_compact::{alg::*, prelude::*, Token, UntrustedToken};
// If you want to try std support, also update the guests Cargo.toml file
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use alloy_primitives::Address;


risc0_zkvm::guest::entry!(main);

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

fn check_predicate(claims: &CredentialClaims, predicate: &Predicate) -> bool {
    let credential_subject = &claims.vc.credential_subject;
    let field_value = &credential_subject[&predicate.field];

    match &predicate.value {
        FieldValue::Int(int_value) => {
            if let Some(credential_value) = field_value.as_u64() {
                let credential_value = credential_value as u32;
                match predicate.condition {
                    Condition::LT => credential_value < *int_value,
                    Condition::GT => credential_value > *int_value,
                    Condition::EQ => credential_value == *int_value,
                    Condition::NEQ => credential_value != *int_value,
                }
            } else {
                false // handle case where the value is not an integer as expected
            }
        }
        FieldValue::Text(text_value) => {
            if let Some(credential_value) = field_value.as_str() {
                match predicate.condition {
                    Condition::EQ => credential_value == text_value,
                    Condition::NEQ => credential_value != text_value,
                    // Less than and greater than comparisons do not typically apply to strings,
                    // but you could implement lexicographical comparison if needed.
                    _ => false, // Returning false or handle differently if needed
                }
            } else {
                false // handle case where the value is not a string as expected
            }
        }
    }
}

pub fn main() {
    let (jwt_credential, public_key_issuer, predicate_list): (String, String, Vec<Predicate>) = env::read();

    let bytes_pk_eid_issuer: &[u8; 32] = &hex::decode(&public_key_issuer).unwrap().try_into().unwrap();

    // Verify the signature, panicking if verification fails.
    let verifying_key_eid_issuer = VerifyingKey::from_bytes(&bytes_pk_eid_issuer).unwrap();

    let untrusted_token_credential = UntrustedToken::new(&jwt_credential).unwrap();

    let credential_claims: Token<CredentialClaims> = Ed25519.validator(&verifying_key_eid_issuer).validate(&untrusted_token_credential).unwrap();

    let claims = &credential_claims.claims().custom;

    for predicate in &predicate_list {
        let is_valid = &check_predicate(claims, &predicate);
        assert!(is_valid);
    }

    env::commit(&(
        &predicate_list.into_iter().map(|x| x.return_value).collect::<Vec<String>>()
    ))
}
