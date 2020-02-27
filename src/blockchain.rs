use std::collections::HashMap;

use crate::block::Block;
use crate::crypto::merkle::MerkleTree;
use crate::transaction::Transaction;
use crate::crypto::hash::{H256, Hashable};

pub struct Blockchain {
    ledger: HashMap<H256, Block>,
    heights: HashMap<H256, u32>,
    tip_hash: H256,
}

impl Blockchain {
    /// Create a new blockchain, only containing the genesis block
    pub fn new() -> Self {
        let parent = H256::from([0; 32]);

        let difficulty: H256 = Blockchain::get_difficulty().into();

        let mut transactions: Vec<Transaction> = Vec::new();
        let transaction = Transaction::new("genesis in".to_string(), "genesis out".to_string());
        transactions.push(transaction);

        let merkle_tree = MerkleTree::new(&transactions);
        let merkle_root = merkle_tree.root();

        let genesis_block: Block = Block::new(parent, difficulty, transactions, merkle_root);

        let mut chain: HashMap<H256, Block> = HashMap::new();
        let hashed = genesis_block.hash();
        chain.insert(hashed, genesis_block);
        let mut heights: HashMap<H256, u32> = HashMap::new();
        heights.insert(hashed, 0);
        let blockchain = Blockchain {
            ledger: chain,
            heights,
            tip_hash: hashed,
        };
        return blockchain;
    }

    pub fn get_difficulty() -> H256 {
        let mut bytes32 = [255u8;32];
        bytes32[0] = 0;
        bytes32[1] = 0;
        bytes32[4] = 6;
        bytes32[10] = 13;
        bytes32[23] = 7;
        bytes32[28] = 45;
        let difficulty: H256 = bytes32.into();
        return difficulty;
    }

    /// Insert a block into blockchain
    pub fn insert(&mut self, block: &Block) {
        let bl: Block = block.clone();
        let parent_hash = bl.get_parent();
        let parent_height: u32 = self.heights.get(&parent_hash).unwrap().clone();
        let hashed = bl.hash();
        let h = parent_height + 1;
        if h > self.heights.get(&self.tip_hash).unwrap().clone() {
            self.tip_hash = hashed;
        }
        self.ledger.insert(hashed, bl);
        self.heights.insert(hashed, h);
    }

    /// Get the last block's hash of the longest chain
    pub fn tip(&self) -> H256 {
        return self.tip_hash;
    }

    /// Get the hash of all blocks in the longest chain
    #[cfg(any(test, test_utilities))]
    pub fn all_blocks_in_longest_chain(&self) -> Vec<H256> {
        let mut current: H256 = self.tip();
        let mut path: Vec<H256> = Vec::new();
        let mut h = self.heights.get(&current).unwrap().clone();
        path.push(current);
        while h > 0 {
            current = self.ledger.get(&current).unwrap().get_parent();
            path.push(current);
            h = h - 1;
        }
        return path;
    }
}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::block::test::generate_random_block;
    use crate::crypto::hash::Hashable;

    #[test]
    fn insert_one() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let block = generate_random_block(&genesis_hash);
        blockchain.insert(&block);
        assert_eq!(blockchain.tip(), block.hash());
    }

    /*
    #[test]
    fn insert_more() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        println!("genesis: {}", genesis_hash);
        let block1 = generate_random_block(&genesis_hash);
        blockchain.insert(&block1);
        println!("#1 {}", block1.hash());
        let block2 = generate_random_block(&genesis_hash);
        blockchain.insert(&block2);
        println!("#2 {}", block2.hash());
        let block3 = generate_random_block(&block2.hash());
        blockchain.insert(&block3);
        println!("#3 {}", block3.hash());
        let block4 = generate_random_block(&block2.hash());
        blockchain.insert(&block4);
        println!("#4 {}", block4.hash());
        let block5 = generate_random_block(&block4.hash());
        blockchain.insert(&block5);
        println!("#5 {}", block5.hash());
        let block6 = generate_random_block(&block1.hash());
        blockchain.insert(&block6);
        println!("#6 {}", block6.hash());
        let tip_hash = blockchain.tip();
        println!("tip is {}", tip_hash);
        let xx = blockchain.all_blocks_in_longest_chain();
        println!("{:?}", xx);
        assert_eq!(blockchain.tip(), block5.hash());
    }
    */
}
