use sha256::digest;
use std::collections::LinkedList;
use std::time::SystemTime;

#[derive(Debug)]
#[allow(dead_code)]
struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u128,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u128) -> Transaction {
        Self { from, to, amount }
    }
}

#[derive(Debug)]
struct Block {
    pub prev_hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(prev_hash: String) -> Block {
        Self {
            prev_hash,
            transactions: vec![],
        }
    }

    pub fn hash(&self) -> String {
        let source = self
            .transactions
            .iter()
            .map(|transaction| format!("{:?}", transaction))
            .collect::<String>()
            + &self.prev_hash;

        digest(source)
    }
}

#[derive(Debug)]
struct Blockchain {
    pub blocks: LinkedList<Block>,
    pub mem_pool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(timestamp: u64) -> Blockchain {
        Self {
            blocks: LinkedList::from([Block::new(digest(timestamp.to_string()))]),
            mem_pool: vec![],
        }
    }

    pub fn on_transaction(&mut self, tx: Transaction) {
        self.mem_pool.push(tx);
    }

    pub fn mint(&mut self) {
        let prev_hash = self.blocks.back().expect("empty blockchain").hash();
        let mut new_block = Block::new(prev_hash);
        if let Some(tx) = self.mem_pool.pop() {
            new_block.transactions.push(tx)
        }

        self.blocks.push_back(new_block);
    }
}

fn main() {
    let timestamp = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("failed to get timestamp")
    .as_secs();

    let mut bc = Blockchain::new(timestamp);
    println!("{:?}", bc);

    bc.on_transaction(Transaction::new(
        String::from("alice"),
        String::from("bob"),
        100u128,
    ));
    bc.mint();
    println!("{:?}", bc);

    bc.mint();
    println!("{:?}", bc);
}

#[cfg(test)]
mod tests {
    use crate::{Blockchain, Transaction};

    #[test]
    fn genesis_test() {
        let timestamp = 0u64;

        let bc = Blockchain::new(timestamp);
        assert_eq!(bc.blocks.len(), 1, "blockchain should contain genesis block");
        assert_eq!(bc.mem_pool.len(), 0, "blockchain mem_pool should be empty");
    }

    #[test]
    fn mint_empty_block() {
        let timestamp = 0u64;

        let mut bc = Blockchain::new(timestamp);
        bc.mint();
        assert_eq!(bc.blocks.len(), 2, "blockchain should contain two blocks");
        assert_eq!(bc.blocks.back().unwrap().transactions.len(), 0, "new block should be empty");
    }

    #[test]
    fn mint_block() {
        let timestamp = 0u64;

        let mut bc = Blockchain::new(timestamp);
        bc.on_transaction(Transaction::new(
            String::from("alice"),
            String::from("bob"),
            100u128,
        ));
        bc.mint();
        assert_eq!(bc.blocks.len(), 2, "blockchain should contain two blocks");
        assert_eq!(bc.blocks.back().unwrap().transactions.len(), 1, "new block should contain one transaction");
        assert_eq!(bc.mem_pool.len(), 0, "blockchain mem_pool should be empty after mint");
    }
}