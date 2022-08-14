//bring external crates that will help us accomplishing the goal
extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

//specifically bring sha256 standard libray
use sha2::{sha256, Digest};
use std::fmt::Write;

//create the transactions struct
#[derive(Debug, Clone, Serialize)]
struct Transaction {

    sender:String,
    recipient:String,
    amount: f32

}

//create A block header struct
#[ derive(Serialize, Debug)]
pub struct BlockHeader {

    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficultiy: u32

}

// create block struct
#[ derive(Serialize, Debug)]
pub struct Block {
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>
}

//create a chain block that will chain each of the block
pub struct Chain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    difficultiy: u32,
    miner_address: String,
    reward: f32

}

