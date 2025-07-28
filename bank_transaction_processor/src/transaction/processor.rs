use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::types::Transaction;

pub trait Processor: Send {
    fn process(&mut self, txn: Transaction);
}

#[derive(Clone)]
pub struct Bank {
    state: Arc<Mutex<HashMap<u64, i64>>>,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Processor for Bank {
    fn process(&mut self, txn: Transaction) {
        let mut state = self.state.lock().unwrap();
        match txn {
            Transaction::Deposit { account_id, amount } => {
                *state.entry(account_id).or_insert(0) += amount as i64;
                println!("[Deposit] Account {}: +{}", account_id, amount);
            }
            Transaction::Withdraw { account_id, amount } => {
                *state.entry(account_id).or_insert(0) -= amount as i64;
                println!("[Withdraw] Account {}: -{}", account_id, amount);
            }
            Transaction::Transfer { from, to, amount } => {
                *state.entry(from).or_insert(0) -= amount as i64;
                *state.entry(to).or_insert(0) += amount as i64;
                println!("[Transfer] {} → {}: {}", from, to, amount);
            }
        }
        println!("Current State: {:?}", *state);
    }
}
