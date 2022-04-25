use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviusHash,
    InvalidGenisisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransction,
}

pub struct Blockchain{
    pub blocks: Vec<Block>,
    unspend_outputs: HashSet<Hash>
}

impl Blockchain{
    pub fn update_with_block (&mut self, block: Block) -> Result<(), BlockValidationErr>{

        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        }
        else if !block::check_difficulty(&block.hash(), block.difficulty){
            return Err(BlockValidationErr::InvalidHash);
        }
        else if i != 0 {
            // not genisis block
            let prev_block = &self.blocks[i-1];

            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            }
            else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviusHash);
            }
        }
        else {
            if block.prev_block_hash != vec![0; 32]{
                return Err(BlockValidationErr::InvalidGenisisBlockFormat);
            }
        }

        if let Some((coinbase, transaction)) = block.transactions.split_first(){

            if !coinbase.is_coinbase(){
                return Err(BlockValidationErr::InvalidCoinbaseTransction)
            }
            let block_spend: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0.0;

            for transac in transaction {
                let input_hashes = transac.input_hashes();

                if !(&input_hashes - &self.unspend_outputs).is_empty()
                    || !(&input_hashes & &block_spend).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transac.input_value();
                let output_value = transac.output_value();

                if output_value > input_value{
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee as f64;

                }

                if (coinbase.output_value() as f64) < total_fee{
                    return Err(BlockValidationErr::InvalidCoinbaseTransction);
                }else {
                    block_created.extend(coinbase.output_hashes());
                }
    
                self.unspend_outputs.retain(|output|!block_spend.contains(output));
                self.unspend_outputs.extend(block_created);

                
            }

            self.blocks.push(block);
            Ok(())
        }

        pub fn new () -> Self {
            Blockchain {
                blocks: vec![],
                unspend_outputs: HashSet::new(),
            }
        }

    }

