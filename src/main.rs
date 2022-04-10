use std::time::SystemTime;

mod block;
mod blockchain;
mod transaction;

use blockchain::Blockchain;
use transaction::Transaction;

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