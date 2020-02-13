extern crate rand;

use serde::{Serialize, Deserialize};
use ring::digest::{SHA256, digest};
use std::time::{SystemTime};
use crate::crypto::hash::{H256, Hashable};
use crate::crypto::merkle::{MerkleTree};
use crate::transaction::{Transaction};
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    header: Header,
    content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    parent: H256,
    nonce: u32,
    difficulty: H256,
    timestamp: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    transactions: Vec<Transaction>,
    merkle_root: H256,
}

impl Hashable for Header {
    fn hash(&self) -> H256 {
        let serialized = bincode::serialize(&self).unwrap();
        let hashed = digest(&SHA256, &serialized);
        let hashed256 = H256::from(hashed);
        return hashed256;
    }
}

impl Hashable for Block {
    fn hash(&self) -> H256 {
        return self.header.hash();
    }
}

#[cfg(any(test, test_utilities))]
pub mod test {
    use super::*;
    use crate::crypto::hash::H256;

    pub fn generate_random_block(parent: &H256) -> Block {
        let mut rng = rand::thread_rng();
        let nonce: u32 = rng.gen();

        let mut bytes32 = [255u8;32];
        bytes32[0]=0;
        bytes32[1]=0;
        bytes32[4]=6;
        bytes32[10]=13;
        bytes32[18]=66;
        bytes32[23]=23;
        bytes32[27]=41;
        let difficulty: H256 = bytes32.into();

        let timestamp = SystemTime::now();
        let header = Header {
            parent: parent.clone(),
            nonce,
            difficulty,
            timestamp,
        };
        let transactions: Vec<Transaction> = Vec::new();
        let merkle_tree = MerkleTree::new(&transactions);
        let merkle_root = merkle_tree.root();
        let content = Content {
            transactions,
            merkle_root,
        };
        let block = Block {
            header,
            content,
        };
        return block;
    }
}
