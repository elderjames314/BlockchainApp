# Blockchain app

nput a miner address: James
Difficulty: 2
generating genesis block! 
Block hahs 00ae3a1c9e43fad2e952572ed3394b2d6f7e3430fbbe09f49677c6a060
Block {
    header: BlockHeader {
        timestamp: 1660553377760,
        nonce: 45629,
        pre_hash: "0000000000000000000000000000000000000000000000000000000000000000",
        merkle: "b24c58bbab7cc4ccd03cebdd0abf8c029989a4b4a2fa1675d14235dac4fb",
        difficultiy: 2,
    },
    count: 1,
    transactions: [
        Transaction {
            sender: "root",
            recipient: "James",
            amount: 100.0,
        },
    ],
}
Menu
1) New transaction
2) Mine block
3) Change difficulty
4) Change reward
0) Exit
Enter your choice: 1

Enter sender address: Oladimeji
Enter the recipient address: dejo
Enter the amount: 20
transaction added successfully
Menu
1) New transaction
2) Mine block
3) Change difficulty
4) Change reward
0) Exit
Enter your choice: 2

Generating block
Block hahs 006bff19c9a2ee4c9e298baafcfdc2df18c4f627181253a2523be0e44fa3
Block {
    header: BlockHeader {
        timestamp: 1660553401647,
        nonce: 129971,
        pre_hash: "00ae3a1c9e43fad2e952572ed3394b2d6f7e3430fbbe09f49677c6a060",
        merkle: "ff66677566a3bb4e935c48cd774b8beb62a1132d7121c5c6802df82d782c497",
        difficultiy: 2,
    },
    count: 2,
    transactions: [
        Transaction {
            sender: "root",
            recipient: "James",
            amount: 100.0,
        },
        Transaction {
            sender: "Oladimeji",
            recipient: "dejo",
            amount: 20.0,
        },
    ],
}
Block generated successfully
Menu
1) New transaction
2) Mine block
3) Change difficulty
4) Change reward
0) Exit
Enter your choice: 3

enter new difficulty: 1
Difficulty was updated successfully
Menu
1) New transaction
2) Mine block
3) Change difficulty
4) Change reward
0) Exit
Enter your choice: 4

Enter new reward: 150.34
reward updated successfully
Menu
1) New transaction
2) Mine block
3) Change difficulty
4) Change reward
0) Exit
Enter your choice: 2

Generating block
Block hahs 0972dac6d2c362fa2b2c8f259f746c7fd231ebc82b51f9e7e484a0601558
Block {
    header: BlockHeader {
        timestamp: 1660553429046,
        nonce: 267,
        pre_hash: "006bff19c9a2ee4c9e298baafcfdc2df18c4f627181253a2523be0e44fa3",
        merkle: "4eaeafee97d096ae3d96251288e715a963688195cb9a3bcb7773d6e5c992d94",
        difficultiy: 1,
    },
    count: 1,
    transactions: [
        Transaction {
            sender: "root",
            recipient: "James",
            amount: 150.34,
        },
    ],
}
Block generated successfully
Menu
1) New transaction
2) Mine block
3) Change difficulty
4) Change reward
0) Exit
Enter your choice: 