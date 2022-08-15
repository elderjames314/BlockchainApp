#[macro_use]
extern  crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod my_blockchain_app;

fn main() {
    
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    //get the miner address
    print!("input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_address);
    //get difficulty
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);

    //ensure difficulty is passed as integer
    let diff = difficulty.trim().parse::<u32>().expect("we need an integer");

    //generate genesis block
    println!("generating genesis block! ");
    let mut chain = my_blockchain_app::Chain::new(miner_address.trim().to_string(), diff);

    //create action items
    loop {
        println!("Menu");
        println!("1) New transaction");
        println!("2) Mine block");
        println!("3) Change difficulty");
        println!("4) Change reward");
        println!("0) Exit");
        print!("Enter your choice");

        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        //find out which choice was selected
        match choice.trim().parse().unwrap() {

            0 => {
                println!("exiting...");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut recipient = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);

                print!("Enter the recipient address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut recipient);

                print!("Enter the amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                //now that we have all the params for transactions
                //let create new transction and put into result variable

                let result = chain.new_transaction(sender.trim().to_string(), recipient.trim().to_string(), amount.trim().parse().unwrap());

                //check if result is successfull or not
                match result {
                    true => println!("transaction added successfully"),
                    false => println!("transaction failed, please try again")
                }

            },
            2 => {
                println!("Generating block");
                let result = chain.generate_new_block();
                match result {
                    true => println!("Block generated successfully"),
                    false => println!("The system could not generate new block this time")
                }
            },
            3 => {
                //updating difficulty
                let mut new_difficulty = String::new();
                print!("enter new difficulty: ");

                io::stdout().flush();
                io::stdin().read_line(&mut new_difficulty);
                let result = chain.update_difficulty(new_difficulty.trim().parse().unwrap());

                match result {
                    true => println!("Difficulty was updated successfully"),
                    false => println!("the operation failed to update difficulty")
                }

            },
            4 => {
                //update reward
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let result = chain.update_reward(new_reward.trim().parse().unwrap());
                
                match result {
                    true => println!("reward updated successfully"),
                    false => println!("failed to update reward")
                }
            
            }

            _ => println!("\tinvalid option passed, please retry \t")

        }
    }

}