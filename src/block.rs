use serde::{Serialize, Deserialize};
use ring::digest::{SHA256, digest};
use std::time::{SystemTime};
use crate::crypto::hash::{H256, Hashable};
use crate::crypto::merkle::{MerkleNode, MerkleTree};
use crate::transaction::{Transaction};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    header: Header,
    content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    parent: Box<Block>,
    parent_hash: H256,
    nonce: u32,
    difficulty: H256,
    timestamp: SystemTime,
    merkle_root: MerkleNode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    transactions: Vec<Transaction>,
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
        unimplemented!()
    }
}
