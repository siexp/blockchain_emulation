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
pub struct Header {
    pub mint_timestamp: u64,

    pub nonce: u128,
    pub hash: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(mint_timestamp: u64, nonce: u128, hash: String) -> Block {
        Self {
            header: Header {
                mint_timestamp: mint_timestamp,

                nonce: nonce,
                hash: hash,
            },
            transactions: vec![],
        }
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
            blocks: LinkedList::from([Block::new(timestamp, 0u128, digest(timestamp.to_string()))]),
            mem_pool: vec![],
        }
    }

    pub fn on_transaction(&mut self, tx: Transaction) {
        self.mem_pool.push(tx);
    }

    pub fn mint(&mut self) {
        let transactions: Vec<Transaction> = self
            .mem_pool
            .drain(0..std::cmp::min(self.mem_pool.len(), 3))
            .collect();

        let (hash, nonce) = {
            let prev_hash = &self.blocks.back().expect("blockchain is empty").header.hash;
            let tx_hash = transactions
                .iter()
                .map(|tx| format!("{:?}", tx))
                .collect::<String>();

            let mut nonce = 0u128;
            let mut hash = digest(tx_hash.clone() + &nonce.to_string() + &prev_hash);
            while hash.matches("1").count() != 8 {
                nonce += 1;
                hash = digest(tx_hash.clone() + &nonce.to_string() + &prev_hash);
            }

            (hash, nonce)
        };

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("failed to get timestamp")
            .as_secs();

        self.blocks.push_back(Block {
            header: Header {
                mint_timestamp: timestamp,
                nonce: nonce,
                hash: hash,
            },
            transactions: transactions,
        });
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
        assert_eq!(
            bc.blocks.len(),
            1,
            "blockchain should contain genesis block"
        );
        assert_eq!(bc.mem_pool.len(), 0, "blockchain mem_pool should be empty");
    }

    #[test]
    fn mint_empty_block() {
        let timestamp = 0u64;

        let mut bc = Blockchain::new(timestamp);
        bc.mint();
        assert_eq!(bc.blocks.len(), 2, "blockchain should contain two blocks");
        assert_eq!(
            bc.blocks.back().unwrap().transactions.len(),
            0,
            "new block should be empty"
        );
        const EXPECTED_HASH: &str = "0411b5d9de8a3255512a01596b62f6b51ec11ade2a9398b8b40a6e332f6a351d";
        assert_eq!(
            bc.blocks.back().unwrap().header.hash,
            EXPECTED_HASH,
            "block hash should be {}",
            EXPECTED_HASH
        );
        assert_eq!(
            bc.blocks.back().unwrap().header.hash.matches("1").count(),
            8,
            "block hash should contain exact eight '1'"
        );
    }

    #[test]
    fn mint_block_with_1_tx_in_mem_pool() {
        let timestamp = 0u64;

        let mut bc = Blockchain::new(timestamp);
        bc.on_transaction(Transaction::new(
            String::from("alice"),
            String::from("bob"),
            100u128,
        ));
        bc.mint();
        assert_eq!(
            bc.blocks.back().unwrap().transactions.len(),
            1,
            "new block should contain one transaction"
        );
        assert_eq!(
            bc.mem_pool.len(),
            0,
            "blockchain mem_pool should be empty after mint"
        );
    }

    #[test]
    fn mint_block_with_4_tx_in_mem_pool() {
        let timestamp = 0u64;

        let mut bc = Blockchain::new(timestamp);
        bc.on_transaction(Transaction::new(
            String::from("alice"),
            String::from("bob"),
            100u128,
        ));
        bc.on_transaction(Transaction::new(
            String::from("bob"),
            String::from("alice"),
            25u128,
        ));
        bc.on_transaction(Transaction::new(
            String::from("bob"),
            String::from("alice"),
            25u128,
        ));
        bc.on_transaction(Transaction::new(
            String::from("bob"),
            String::from("alice"),
            50u128,
        ));
        bc.mint();
        assert_eq!(
            bc.blocks.back().unwrap().transactions.len(),
            3,
            "new block should contain three transaction"
        );
        assert_eq!(
            bc.mem_pool.len(),
            1,
            "blockchain mem_pool should contain one transaction after mint"
        );
    }
}
