use std::time::Instant;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    version: u8,
    previous_block_hash: String,
    merkle_root: String,
    timestamp: u128,
}

impl BlockHeader {
    pub fn new(version: u8, previous_block_hash: String, merkle_root: String) -> Self {
        let start = Instant::now();
        let elapsed = start.elapsed();
        BlockHeader {
            version,
            previous_block_hash,
            merkle_root,
            timestamp: elapsed.as_millis(),
        }
    }
    // fn id(&self) -> 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<String>,
    block_hash: String
}

impl Block {
    pub fn init_blockchain() -> Self {
        Block {
            header: BlockHeader::new(1, String::from("0x0"), String::from("0x0")),
            transactions: Vec::new(),

        }
    }
}