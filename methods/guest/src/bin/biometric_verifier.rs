use credential_verifier::{CredentialVerifier};
use risc0_zkvm::guest::env;

pub fn main() {
    let (onboarding_biometric_credential_jwt, challenge_biometric_credential_jwt, onboarding_issuer_public_key): (String, String, String) = env::read();

    let subject = CredentialVerifier::verify(&onboarding_biometric_credential_jwt, &challenge_biometric_credential_jwt, &onboarding_issuer_public_key, &String::from("fingerprint"));

    env::commit(&subject);
}
