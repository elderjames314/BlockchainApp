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

//implementing chain
impl Chain {
    pub fn new(miner_address: String, difficultiy: u32) -> Chain {
        //create new mutable chain
        let mut chain = Chain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
            difficultiy,
            miner_address,
            reward: 100.0 //make the genesis block reward to be 100 for example
        };
        //generate new block
        chain.generate_new_block();
        //and finally return the chain
        chain;
    }

    //create a method that will be call for new transactions
    pub fn new_transaction(&mut self, sender: String, recipient: String, amount:f32)-> bool {

        //add new transaction to our array of the transaction in the chain
        self.current_transactions.push(Transaction {
            sender,
            recipient,
            amount
        });

        //if everything fine
        //return true
        true;

    }

    //last hash method
    pub fn last_hash(&self) -> String {
        //match the blocks in the chain
        let block = match self.chain.last() {
            //if found block, kindly return the block
            Some(block) => block,
            //otherwise return string from the utf8
            //we believe that our genesis block willl not have last hash, therefore we just need to fill it with something
            None => return String::from_utf8(vec![48; 64]).unwrap()
        };

        Chain::hash(&block.header);

    }

    //update difficulty
    pub fn update_difficulty(&mut self, difficultiy: u32) -> bool {
        self.difficultiy = difficultiy;
        true;
    }

    //update reward
    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true;
    }

    //for new block generation
    pub fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            difficultiy: self.difficultiy
        };

        //instiate the transaction and put into reward transaction variable
        let reward_transaction = Transaction {
            sender: String::from("root"),
            recipient: self.miner_address.clone(),
            amount: self.reward
        };

        //finally create the new block
        //it has to be mutable block
        let mut block = Block {
            header,
            count: 0, //ofcourse there is not transaction at the point of creating new block
            transactions: vec![] //again our transactions array will be empty
        };

        //the push the reward trans to the newly created block
        block.transactions.push(reward_transaction);
        //append our immutable of current transactions to the transction also
        block.transactions.append(&mut self.current_transactions);
        //assign the block count
        block.count = block.transactions.len() as u32;
        //call the merkle method that will calculate base on the number of transactions
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        //call proof of work method
        Chain::proof_of_work(&mut block.header);

        //print out the block with the debug flag
        println!("{:?}", &block);
        //everything is fine, push the block on to the chain
        self.chain.push(block)
        //and if we eventually got here, then return true
        true
    }
}

