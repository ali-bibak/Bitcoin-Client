extern crate untrusted;

use serde::{Serialize, Deserialize};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair, VerificationAlgorithm, EdDSAParameters};
use ring::digest::{SHA256, digest};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MySignature {
    value: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    input: String,
    output: String,
    signature: Option<MySignature>,
}

impl Transaction {
    fn set_signature(&mut self, signature: &Signature) {
        if self.is_signed() {
            eprintln!("Ignored attempt to sign the already signed transaction");
            return ;
        }
        let my_signature = MySignature {
            value: signature.as_ref().to_vec(),
        };
        self.signature = Option::from(my_signature);
    }

    fn is_signed(&self) -> bool {
        return match self.signature {
            Some(_) => true,
            None => false,
        };
    }
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
    let status= match verification {
        Ok(_) => true,
        Err(_) => false,
    };
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
