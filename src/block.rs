use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Header {
    pub mint_timestamp: u64,

    pub nonce: u128,
    pub hash: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(mint_timestamp: u64, nonce: u128, hash: String) -> Block {
        Self {
            header: Header {
                mint_timestamp,

                nonce,
                hash,
            },
            transactions: vec![],
        }
    }
}
