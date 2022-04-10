#[derive(Debug)]
#[allow(dead_code)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u128,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u128) -> Transaction {
        Self { from, to, amount }
    }
}
