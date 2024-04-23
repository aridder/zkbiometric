
use ed25519_dalek::VerifyingKey;
use jwt_compact::{alg::*, prelude::*, Token, UntrustedToken};
// If you want to try std support, also update the guest Cargo.toml file
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKeyHolder {
    pub public_key: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Issuer {
    id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Proof {
    #[serde(rename = "type")]
    proof_type: String,
    pub jwt: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GenericCredential {
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
    pub proof: Proof,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialClaims {
    vc: CredentialSubjectClaims,
    sub: String,
    iss: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CredentialSubjectClaims {
    #[serde(rename = "credentialSubject")]
    credential_subject: serde_json::Value,
    #[serde(rename = "type")]
    types: Vec<String>,
    #[serde(rename = "@context")]
    context: Vec<String>,
}

pub struct CredentialVerifier;

impl CredentialVerifier {
    pub fn verify(
        credential_one_jwt: &str,
        credential_two_jwt: &str,
        issuer_pk: &str,
        field_to_verify: &str,
    ) -> String {
        let bytes_issuer_pk: &[u8; 32] = &hex::decode(&issuer_pk).unwrap().try_into().unwrap();

        let verifying_issuer_key = VerifyingKey::from_bytes(&bytes_issuer_pk).unwrap();

        let credential_one = UntrustedToken::new(&credential_one_jwt).unwrap();
        let credential_two = UntrustedToken::new(&credential_two_jwt).unwrap();

        let claims_one: Token<CredentialClaims> = Ed25519.validator(&verifying_issuer_key).validate(&credential_one).unwrap();
        let claims_two: Token<CredentialClaims> = Ed25519.validator(&verifying_issuer_key).validate(&credential_two).unwrap();

        let subject_one = &claims_one.claims().custom.sub;
        let subject_two = &claims_two.claims().custom.sub;

        let claims = &claims_one.claims().custom;
        let claims_two = &claims_two.claims().custom;

        let field_one = &claims.vc.credential_subject[field_to_verify];
        let field_two = &claims_two.vc.credential_subject[field_to_verify];

        assert_eq!(field_one, field_two);
        assert_eq!(subject_one, subject_two);

        subject_one.to_string()
    }
}
