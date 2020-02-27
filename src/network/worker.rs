use crossbeam::channel;
use log::{debug, warn};
use std::thread;
use std::sync::{Arc, Mutex};

use super::message::Message;
use super::peer;
use crate::network::server::Handle as ServerHandle;
use crate::blockchain::Blockchain;

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
                Message::NewBlockHashes(_) => {
                    debug!("NewBlockHashes");
                }
                Message::GetBlocks(_) => {
                    debug!("GetBlocks");
                }
                Message::Blocks(_) => {
                    debug!("Blocks");
                }
            }
        }
    }
}
