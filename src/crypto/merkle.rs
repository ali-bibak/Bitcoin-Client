use super::hash::{Hashable, H256};
use std::ptr;
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, Clone)]
pub struct MerkleNode{
    key: H256,
    left_child: *const MerkleNode,
    right_child: *const MerkleNode,
}

/*
impl Default for *const MerkleNode {
    fn default() -> Self{
        return ptr::null();
    }
}
*/

/// A Merkle tree.
#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: *const MerkleNode,
}

fn build(leaves: Vec<MerkleNode>, leaf_size: usize) -> MerkleNode {
    let mut n = leaf_size;
    if n == 1 {
        let root = leaves[0].clone();
        return root;
    }
    let mut new_leaves = leaves.clone();
    if n % 2 == 1 {
        let elem = new_leaves[n - 1].clone();
        new_leaves.push(elem);
        n += 1;
    }
    n = n / 2;
    for i in 0..n {
        let mut elem1: MerkleNode = new_leaves[2 * i].clone();
        let mut elem2: MerkleNode = new_leaves[2 * (i + 1)].clone();
        let hash1 = (elem1.key).as_ref();
        let hash2 = (elem2.key).as_ref();
        let concat = H256::from([hash1, hash2].concat());
        let concat_hash= H256::hash(concat);
        let mut par: MerkleNode = MerkleNode {
            key: concat_hash,
            left_child: &elem1,
            right_child: &elem2,
        };
        new_leaves[i] = par;
    }
    let root = build(new_leaves, n);
    return root;
}

impl MerkleTree {
    pub fn new<T>(data: &[T]) -> Self where T: Hashable, {
        let mut Tree: MerkleTree = MerkleTree {
            root: ptr::null(),
        };
        let leaf_size = data.len();
        let mut leaves: Vec<MerkleNode> = Vec::new();
        for i in 0..leaf_size {
            let dt = data[i].clone();
            let hashed = H256::hash(dt);
            let mut elem: MerkleNode = MerkleNode {
                key: hashed,
                left_child: ptr::null(),
                right_child: ptr::null(),
            };
            leaves.push(elem);
        }
        let root = build(leaves, leaf_size);
        Tree.root = &root;
        return Tree;
    }

    pub fn root(&self) -> H256 {
        return self.root.key;
    }

    /// Returns the Merkle Proof of data at index i
    pub fn proof(&self, index: usize) -> Vec<H256> {
        unimplemented!()
    }
}

/// Verify that the datum hash with a vector of proofs will produce the Merkle root. Also need the
/// index of datum and `leaf_size`, the total number of leaves.
pub fn verify(root: &H256, datum: &H256, proof: &[H256], index: usize, leaf_size: usize) -> bool {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::crypto::hash::H256;
    use super::*;

    macro_rules! gen_merkle_tree_data {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }

    #[test]
    fn root() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let root = merkle_tree.root();
        assert_eq!(
            root,
            (hex!("6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920")).into()
        );
        // "b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0" is the hash of
        // "0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d"
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
        // "6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920" is the hash of
        // the concatenation of these two hashes "b69..." and "965..."
        // notice that the order of these two matters
    }

    #[test]
    fn proof() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert_eq!(proof,
                   vec![hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f").into()]
        );
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
    }

    #[test]
    fn verifying() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert!(verify(&merkle_tree.root(), &input_data[0].hash(), &proof, 0, input_data.len()));
    }
}
