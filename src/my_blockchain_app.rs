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
        chain
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
        true

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

        Chain::hash(&block.header)

    }

    //update difficulty
    pub fn update_difficulty(&mut self, difficultiy: u32) -> bool {
        self.difficultiy = difficultiy;
        true
    }

    //update reward
    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    //for new block generation
    pub fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String,
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

    //get merkle_impl
    pub fn get_merkle(current_transactions: Vec<Transaction>) ->String {
        let mut merkle = Vec::new();

        //iterate the transaction, the hash each of the transaction and push to the merkel vector
        for transaction in &current_transactions {
            let hash = Chain::hash(transaction);
            merkle.push(hash);
        }

        //just incase our length of the merkle is old
        //then we just clone the last hash and push to merkle vector
        //to get even number
        if merkle.len() % 2 == 1 {
            let last = merkle.last().clone().unwrap();
            merkle.push(last);
        }

        //we need to have a single line of merkle
        while(merkle.len() > 1) {
            //the first hash
            let mut h1 = merkle.remove(0);
            //the second hash
            let mut h2 = merkle.remove(0);
            //merge the two hash to gether
            h1.push_str(&mut h2);
            //then hash it again
            let nh = Chain::hash(&h1);
            //push it back to the merkle
            merkle.push(nh);
            //this will continuw until we have a single length in the merkle
        }
        //once we have the single length of hash in the merkle, then pop it out
        merkle.pop().unwrap()
    }

    //implementing proof of work
    pub fn proof_of_work(header: &mut BlockHeader) {
        //make a loop block
        //it will keep looping until we have a proper nonce
        //the number of looping is determine by the difficulty
        //for instance if difficulty is 2, then our hash has to start with 2 leading zero eg 00894434344332343223232233
        //if the difficulty goes, then the looping will increase as well.
        loop {
            //create hash from the header
            let hash = Chain::hash(header);
            //make a slice with header of our hash as per difficulty
            let slice = &hash[..header.difficulty as u32];
            match slice.parse::<u32>() {
                //if everything is fine
                Ok(val) => {
                    if val  != 0 {
                        header.nonce  += 1;
                    }else {
                        println!("Block hahs {}", hash);
                        break;
                    }
                },
                //something went wrong
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }

    //implementing the hash using the serde library
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let result = hasher.result();
        let vec_result = result.to_vec();
        //return the chain version of our vector

        Chain::hex_to_string(vec_result.as_slice())
    }

    pub fn hex_to_string(vec_result: &[u8]) -> String {
        let mut s = String::new();
        //loop through the vector and write each byte to string
        for b in vec_result {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        //return the string
        s
    }
}

