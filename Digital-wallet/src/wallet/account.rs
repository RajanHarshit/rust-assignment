use crate::wallet::error::WalletError;
use crate::wallet::logger::Logger;

#[derive(Debug)]
pub struct Wallet {
    username: String,
    password: String,
    balance: u64,
    history: Vec<String>,
}

impl Wallet {
    pub fn new(username: &str, password: &str, logger: &mut dyn Logger) -> Self {
        logger.log("Created new wallet");
        Self {
            username: username.to_string(),
            password: password.to_string(),
            balance: 0,
            history: vec![],
        }
    }

    pub fn authenticate(&self, input: &str) -> Result<(), WalletError> {
        if self.password == input {
            Ok(())
        } else {
            Err(WalletError::IncorrectPassword)
        }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
        self.history.push(format!("Deposited: {}", amount));
    }

    pub fn transfer(&mut self, amount: u64, to: &str, logger: &mut dyn Logger) -> Result<(), WalletError> {
        if self.balance < amount {
            return Err(WalletError::InsufficientFunds);
        }
        self.balance -= amount;
        let msg = format!("Transferred {} to {}", amount, to);
        logger.log(&msg);
        self.history.push(msg);
        Ok(())
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn get_transaction_history(&self) -> &[String] {
        &self.history
    }
}
