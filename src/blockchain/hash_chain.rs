use crate::user::User;
use rsa::sha2::{Digest, Sha256};
use sha2::digest::consts::U256;

use super::transaction::{self, Transaction};

type BlockRef = Option<Box<Block>>;

pub struct HashChain {
    root: BlockRef,
    height: u32,
    current_block: BlockRef
}

#[derive(Clone, Debug)]
pub struct Block {
    transaction: Transaction,
    previois_block: u64,
    height: u32,
    creator: User,
    block_reward: u8,
    proof_of_work: u64,
    reward_reciever: Option<User>
}

pub struct BlockHash {
    transaction: Transaction,
    height: u32,
}

impl HashChain {
    fn new() -> Self {
        let genesis = Block::new_genesis_block();
        HashChain { root: genesis.clone(), height: 0, current_block: genesis.clone() }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        let hashable = self.current_block.as_ref().unwrap().get_hashable();
        let mut hasher = Sha256::new();
        hasher.update(hashable.as_bytes());
        self.current_block = Block::new_block(transaction, u64::from_be_bytes(hasher.finalize().as_slice().try_into().unwrap()), self.height);
        self.height += 1
    }
}

impl Block {
    fn new_genesis_block() -> BlockRef {
        Some(Box::new(Block{ transaction: todo!(), previois_block: todo!(), height: todo!(), creator: todo!(), block_reward: todo!(), proof_of_work: todo!(), reward_reciever: todo!() }))
    }

    fn new_block(transaction: Transaction, prev: u64, height: u32) -> BlockRef {
        Some(Box::new(Block{ 
            transaction: transaction, 
            previois_block: prev, 
            height: height + 1, 
            creator: todo!("get node username"), 
            block_reward: todo!("calculate somehow"), 
            proof_of_work: 0, 
            reward_reciever: None, 
        }))
    }
    
    fn new_block_with_transaction(transaction: Transaction) -> BlockRef {
        todo!()
    }
    
    pub fn get_hashable(&self) -> String {
        let mut data: String = "".to_owned();
        let th = self.transaction.to_hashable();
        data.push_str(&th.sender);
        data.push_str(&th.recipient);
        data.push_str(&th.amount.to_string());
        data.push_str(&self.height.to_string());
        return data
    }

}