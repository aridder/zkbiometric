# zkBiometric

Securely verify onboarded biometric data using a real-time challenge with a zkVM. Use zero-knowledge proofs to ensure robust identity confirmation while maintaining user privacy.

## Overview

As part of the European Union’s initiative to issue digital credentials to all citizens by 2027, zkBiometric provides a secure and privacy-preserving solution for biometric identity verification. Designed for high assurance and adherence to strict privacy standards, our system enables service providers to verify identities without accessing sensitive biometric data.

### How It Works

- **Biometric Data Capture and Credential Issuance**: Utilizing [Mobai's technology](https://www.mobai.bio/), we capture a digital representation of a user's facial biometrics. These are then encapsulated in a BiometricOnboardingCredential and issued to the user’s digital wallet.
  
- **Biometric Verification for Service Access**: Service providers request users to verify their identity by reproducing their biometric "fingerprint," which is used to generate a BiometricChallengeCredential.
  
- **Zero-Knowledge Proof of Identity Verification**: Our system utilizes RISC Zero's zkVM to verify the identity by confirming that the BiometricChallengeCredential matches the BiometricOnboardingCredential through a zero-knowledge proof.

- **Secure Data Transmission**: Verified identities enable secure transmission of relevant user data like account numbers or and Ethereum address to service providers.

### System Benefits

- **Enhanced Security and Privacy**: Leveraging zero-knowledge proofs to keep biometric data private and secure.
  
- **High Assurance Identity Verification**: Robust biometric mechanisms provide high confidence in identity verification, crucial for sensitive transactions.
  
- **Scalability and EU Compliance**: Designed to scale across the EU, supporting the digital credential rollout and EU regulatory frameworks.

- **Potential**: Can be expanded to financial services for KYC processes, secure voting systems, and any digital platform requiring reliable user authentication.


## Technology Stack

- **Verifiable Credentials (eIDAS 2.0)**: [EU Digital Identity Wallet](https://github.com/eu-digital-identity-wallet)
  
- **RISC Zero zkVM**: [Learn More](https://www.risczero.com/)
  
- **Foundry Template from RISC Zero zkVM**

## How It's Made

Currently set up with mock data for demonstration purposes:
- Public key of biometric credential issuer
- BiometricOnboardingCredential - mock biometric data for the onboarded user
- BiometricChallengeCredential - mock data created upon request by the service provider.
