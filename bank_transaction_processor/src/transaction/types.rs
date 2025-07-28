#[derive(Debug)]
pub enum Transaction {
    Deposit { account_id: u64, amount: u64 },
    Withdraw { account_id: u64, amount: u64 },
    Transfer { from: u64, to: u64, amount: u64 },
}
