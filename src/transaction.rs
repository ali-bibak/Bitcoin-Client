extern crate untrusted;

use serde::{Serialize,Deserialize};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair, VerificationAlgorithm, EdDSAParameters};
use ring::digest::{SHA256, digest};
use crate::crypto::hash::H256;
use ring::error::Unspecified;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    input: String,
    output: String,
    signature: Vec<u8>,
}

/// Create digital signature of a transaction
pub fn sign(t: &Transaction, key: &Ed25519KeyPair) -> Signature {
    let serialized = bincode::serialize(&t).unwrap();
    let hashed = digest(&SHA256, &serialized);
    let signature = key.sign(hashed.as_ref());
    return signature;
}

/// Verify digital signature of a transaction, using public key instead of secret key
pub fn verify(t: &Transaction, public_key: &<Ed25519KeyPair as KeyPair>::PublicKey, signature: &Signature) -> bool {
    let serialized = bincode::serialize(&t).unwrap();
    let hashed = digest(&SHA256, &serialized);
    let public_key = untrusted::Input::from(public_key.as_ref());
    let msg = untrusted::Input::from(hashed.as_ref());
    let sgn = untrusted::Input::from(signature.as_ref());
    let verification = VerificationAlgorithm::verify(&EdDSAParameters, public_key, msg, sgn);
    let mut status;
    match verification {
        Ok(_) => {status = true},
        Err(_) => {status = false},
    }
    return status;
}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::crypto::key_pair;

    pub fn generate_random_transaction() -> Transaction {
        Default::default()
        //unimplemented!()
    }

    #[test]
    fn sign_verify() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, &(key.public_key()), &signature));
    }
}
