use log::info;
use crossbeam::channel::{unbounded, Receiver, Sender, TryRecvError};
use std::time;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::network::server::Handle as ServerHandle;
use crate::blockchain::Blockchain;
use crate::block::Block;
use crate::transaction::Transaction;
use crate::crypto::merkle::MerkleTree;
use crate::crypto::hash::{H256, Hashable};
use crate::network::message::Message;

enum ControlSignal {
    Start(u64), // the number controls the lambda of interval between block generation
    Exit,
}

enum OperatingState {
    Paused,
    Run(u64),
    ShutDown,
}

pub struct Context {
    /// Channel for receiving control signal
    control_chan: Receiver<ControlSignal>,
    operating_state: OperatingState,
    server: ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>,
}

#[derive(Clone)]
pub struct Handle {
    /// Channel for sending signal to the miner thread
    control_chan: Sender<ControlSignal>,
}

pub fn new(
    server: &ServerHandle, blockchain: &Arc<Mutex<Blockchain>>
) -> (Context, Handle) {
    let (signal_chan_sender, signal_chan_receiver) = unbounded();

    let ctx = Context {
        control_chan: signal_chan_receiver,
        operating_state: OperatingState::Paused,
        server: server.clone(),
        blockchain: Arc::clone(&blockchain),
    };

    let handle = Handle {
        control_chan: signal_chan_sender,
    };

    (ctx, handle)
}

impl Handle {
    pub fn exit(&self) {
        self.control_chan.send(ControlSignal::Exit).unwrap();
    }

    pub fn start(&self, lambda: u64) {
        self.control_chan
            .send(ControlSignal::Start(lambda))
            .unwrap();
    }

}

impl Context {
    pub fn start(mut self) {
        thread::Builder::new()
            .name("miner".to_string())
            .spawn(move || {
                self.miner_loop();
            })
            .unwrap();
        info!("Miner initialized into paused mode");
    }

    fn handle_control_signal(&mut self, signal: ControlSignal) {
        match signal {
            ControlSignal::Exit => {
                info!("Miner shutting down");
                self.operating_state = OperatingState::ShutDown;
            }
            ControlSignal::Start(i) => {
                info!("Miner starting in continuous mode with lambda {}", i);
                self.operating_state = OperatingState::Run(i);
            }
        }
    }

    fn miner_loop(&mut self) {
        // main mining loop
        let mut num_mined = 0;
        loop {
            let bc = Arc::clone(&self.blockchain);

            // check and react to control signals
            match self.operating_state {
                OperatingState::Paused => {
                    let signal = self.control_chan.recv().unwrap();
                    self.handle_control_signal(signal);
                    continue;
                }
                OperatingState::ShutDown => {
                    return;
                }
                _ => match self.control_chan.try_recv() {
                    Ok(signal) => {
                        self.handle_control_signal(signal);
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => panic!("Miner control channel detached"),
                },
            }
            if let OperatingState::ShutDown = self.operating_state {
                return;
            }

            let mut block: Block;
            let mut blockchain = (*bc).lock().unwrap();
            while {
                let parent_hash = blockchain.tip();
                let parent = blockchain.get(&parent_hash);
                let difficulty = parent.get_difficulty();
                let mut transactions: Vec<Transaction> = Vec::new();
                let transaction = Transaction::new("new block input!".to_string(), "new block output!".to_string());
                transactions.push(transaction);
                let merkle_tree = MerkleTree::new(&transactions);
                let merkle_root = merkle_tree.root();
                block = Block::new(parent_hash.clone(), difficulty, transactions, merkle_root);

                block.hash() > difficulty
            } {}

            num_mined += 1;
            info!("Successfully mined block #{}: {}", num_mined, block.hash());

            blockchain.insert(&block);
            let mut vec: Vec<H256> = Vec::new();
            vec.push(block.hash());
            self.server.broadcast(Message::NewBlockHashes(vec));

            if let OperatingState::Run(i) = self.operating_state {
                if i != 0 {
                    let interval = time::Duration::from_micros(i as u64);
                    thread::sleep(interval);
                }
            }
        }
    }
}
