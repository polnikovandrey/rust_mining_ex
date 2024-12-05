use std::time::SystemTime;
use crypto_hash::{hex_digest, Algorithm};
use histogram::Histogram;
use serde::{Deserialize, Serialize};

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
        (0..).map(|proof| Block {
            index: block_candidate.index,
            timestamp: block_candidate.timestamp,
            proof,
            transactions: block_candidate.transactions.clone(),
            previous_block_hash: block_candidate.previous_block_hash.clone(),
        }).find(|b| Self::valid(&Self::hash(b), prefix)).unwrap()
    }
}

pub fn measure<F>(label: &str, closure: F) where F: Fn() -> String {
    let iters = 10;
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