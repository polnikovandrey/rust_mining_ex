use crypto_hash::{hex_digest, Algorithm};
use histogram::Histogram;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread::spawn;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    id: String,
    timestamp: u64,
    payload: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    index: u64,
    timestamp: u64,
    pub proof: u64,
    transactions: Vec<Transaction>,
    previous_block_hash: String,
}

impl Block {
    pub fn genesis() -> Block {
        let transaction = Transaction {
            id: String::from("b3c973e2-db05-4eb5-9668-3e81c7389a6d"),
            timestamp: 0,
            payload: String::from("I am Andrey Polnikov"),
        };
        Block {
            index: 1,
            timestamp: 0,
            proof: 1917336,
            transactions: vec![transaction],
            previous_block_hash: String::from("0"),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn hash(block: &Block) -> String {
        hex_digest(Algorithm::SHA256, block.to_json().as_bytes())
    }

    pub fn valid(hash: &str, prefix: &str) -> bool {
        hash.starts_with(prefix)
    }

    pub fn new(timestamp: u64, transactions: Vec<Transaction>, previous_block: &Block) -> Block {
        Block {
            index: previous_block.index + 1,
            timestamp,
            proof: 0,
            transactions,
            previous_block_hash: Self::hash(previous_block),
        }
    }

    pub fn mine_single_threaded_mutably(block: &mut Block, prefix: &str) {
        while !Self::valid(&Self::hash(block), prefix) {
            block.proof += 1;
        }
    }

    pub fn mine_with_iterator(block_candidate: &Block, prefix: &str) -> Block {
        (0..)
            .map(|proof| Block {
                index: block_candidate.index,
                timestamp: block_candidate.timestamp,
                proof,
                transactions: block_candidate.transactions.clone(),
                previous_block_hash: block_candidate.previous_block_hash.clone(),
            })
            .find(|b| Self::valid(&Self::hash(b), prefix))
            .unwrap()
    }

    pub fn mine_with_parallel_iterator_find_first(block_candidate: &Block, prefix: &str) -> Block {
        (0..u64::MAX)
            .into_par_iter()
            .map(|proof| Block {
                index: block_candidate.index,
                timestamp: block_candidate.timestamp,
                proof,
                transactions: block_candidate.transactions.clone(),
                previous_block_hash: block_candidate.previous_block_hash.clone(),
            })
            .find_first(|b| Self::valid(&Self::hash(b), prefix))
            .unwrap()
    }

    pub fn mine_with_parallel_iterator_find_any(block_candidate: &Block, prefix: &str) -> Block {
        (0..u64::MAX)
            .into_par_iter()
            .map(|proof| Block {
                index: block_candidate.index,
                timestamp: block_candidate.timestamp,
                proof,
                transactions: block_candidate.transactions.clone(),
                previous_block_hash: block_candidate.previous_block_hash.clone(),
            })
            .find_any(|b| Self::valid(&Self::hash(b), prefix))
            .unwrap()
    }

    pub fn mine_with_channels(block_candidate: &Block, prefix: &str) -> Block {
        let num_threads: usize = 4;
        let keep_running = Arc::new(AtomicBool::new(true));
        let (sender, receiver) = channel();
        let mut handles = Vec::with_capacity(num_threads);
        for thread_id in 0..num_threads {
            let keep_running_ref = keep_running.clone();
            let mut block = block_candidate.clone();
            let prefix = prefix.to_string();
            block.proof = thread_id as u64;
            let sender = sender.clone();
            let handle = spawn(move || {
                while keep_running_ref.load(Ordering::SeqCst)
                    && !Self::valid(&Self::hash(&block), &prefix)
                {
                    block.proof += num_threads as u64;
                }
                sender.send(block.clone()).unwrap();
                ()
            });
            handles.push(handle);
        }
        let block = receiver.recv().unwrap();
        keep_running.store(false, Ordering::SeqCst);
        for handle in handles {
            handle.join().unwrap();
        }
        block
    }
}

pub fn measure<F>(label: &str, closure: F)
where
    F: Fn() -> String,
{
    let iters = 3;
    let mut histogram = Histogram::new();
    println!("{}:", label);
    for _ in 0..iters {
        let start = SystemTime::now();
        let mut s = closure();
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        let millis: u64 = duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000;
        s.clear();
        println!("{} ms", millis);
        histogram.increment(millis).unwrap();
    }
    let mean = histogram.mean().unwrap();
    let median = histogram.percentile(50f64).unwrap();
    let min = histogram.minimum().unwrap();
    let max = histogram.maximum().unwrap();
    let std_dev = histogram.stddev().unwrap();
    println!("mean:\t{} ms/iter", mean);
    println!("median:\t{} ms/iter", median);
    println!("min:\t{} ms/iter", min);
    println!("max:\t{} ms/iter", max);
    println!("std_dev:\t{} ms/iter", std_dev);
}
