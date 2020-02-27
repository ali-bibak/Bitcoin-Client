use crossbeam::channel;
use log::{debug, warn};
use std::thread;
use std::sync::{Arc, Mutex};

use super::message::Message;
use super::peer;
use crate::network::server::Handle as ServerHandle;
use crate::blockchain::Blockchain;
use crate::block::Block;
use crate::crypto::hash::H256;
use log::error;

#[derive(Clone)]
pub struct Context {
    msg_chan: channel::Receiver<(Vec<u8>, peer::Handle)>,
    num_worker: usize,
    server: ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>,
}

pub fn new(
    num_worker: usize,
    msg_src: channel::Receiver<(Vec<u8>, peer::Handle)>,
    server: &ServerHandle,
    blockchain: &Arc<Mutex<Blockchain>>
) -> Context {
    Context {
        msg_chan: msg_src,
        num_worker,
        server: server.clone(),
        blockchain: Arc::clone(blockchain),
    }
}

impl Context {
    pub fn start(self) {
        let num_worker = self.num_worker;
        for i in 0..num_worker {
            let cloned = self.clone();
            thread::spawn(move || {
                cloned.worker_loop();
                warn!("Worker thread {} exited", i);
            });
        }
    }

    fn worker_loop(&self) {
        loop {
            let bc = Arc::clone(&self.blockchain);
            let msg = self.msg_chan.recv().unwrap();
            let (msg, peer) = msg;
            let msg: Message = bincode::deserialize(&msg).unwrap();
            match msg {
                Message::Ping(nonce) => {
                    debug!("Ping: {}", nonce);
                    peer.write(Message::Pong(nonce.to_string()));
                }
                Message::Pong(nonce) => {
                    debug!("Pong: {}", nonce);
                }
                Message::NewBlockHashes(block_hashes) => {
                    debug!("NewBlockHashes: {:?}", block_hashes);
                    let blockchain = (*bc).lock().unwrap();
                    let mut vec: Vec<H256> = Vec::new();
                    for block_hash in &block_hashes {
                        if !blockchain.find(&block_hash) {
                            vec.push(block_hash.clone());
                        }
                    }
                    debug!("Asking for blocks: {:?}", vec);
                    peer.write(Message::GetBlocks(vec));
                }
                Message::GetBlocks(block_hashes) => {
                    debug!("GetBlocks: {:?}", block_hashes);
                    let blockchain = (*bc).lock().unwrap();
                    let mut vec: Vec<Block> = Vec::new();
                    for block_hash in &block_hashes {
                       if !blockchain.find(&block_hash) {
                           error!("Error finding the block {:?}", block_hash);
                       } else {
                           vec.push(blockchain.get(&block_hash));
                       }
                    }
                    debug!("Sending the blocks: {:?}", vec);
                    peer.write(Message::Blocks(vec));
                }
                Message::Blocks(blocks) => {
                    debug!("Blocks: {:?}", blocks);
                    let mut blockchain = (*bc).lock().unwrap();
                    for block in &blocks {
                        blockchain.insert(&block);
                    }
                }
            }
        }
    }
}
