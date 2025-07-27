/// Struct representing cash storage in the ATM.
pub struct Cash {
    amount: u32,
}

impl Cash {
    pub fn new(amount: u32) -> Self {
        Cash { amount }
    }

    /// Attempt to withdraw an amount of cash.
    pub fn withdraw(&mut self, amount: u32) -> Result<u32, String> {
        if self.amount >= amount {
            self.amount -= amount;
            println!("Withdrew: ${}", amount);
            Ok(amount)
        } else {
            Err(format!("Insufficient funds: Requested ${}, Available ${}", amount, self.amount))
        }
    }

    pub fn balance(&self) -> u32 {
        self.amount
    }
}