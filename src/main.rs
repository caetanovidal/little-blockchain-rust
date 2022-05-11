use blockchainlib::*;
fn main () {

    let difficulty = 0x000ffffffffffffffffffffffffffff;

    let mut genisis_block = Block::new(0, now(), vec![0; 32], 0, vec![
        Transaction {
            inputs : vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output{
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }
    ], difficulty
);

    //println!("{:?}", &block);

    //let h = block.hash();

    //println!("{:?}", &h);

    //block.hash = h;

    //println!("{:?}", &block);

    genisis_block.mine();
    println!("{:?}", &genisis_block);

    let mut last_hash = genisis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genisis_block).expect("Fail to add a genesis block");



    for i in 1..11 {
        let mut block = Block::new(i, now(), last_hash, 0, vec![
            Transaction {
                inputs : vec![ ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 50,
                    },
                    transaction::Output{
                        to_addr: "Bob".to_owned(),
                        value: 7,
                    },
                ],
            }
        ], difficulty
     );

    block.mine();

    last_hash = block.hash.clone();

    println!("{:?}", &block);

    blockchain.update_with_block(block).expect("Fail to add a block");


    }

    
}
