use std::cell::RefCell;
use std::rc::Rc;

/// Struct representing cash storage in the ATM.
struct Cash {
    amount: u32,
}

impl Cash {
    fn new(amount: u32) -> Self {
        Cash { amount }
    }

    /// Attempt to withdraw an amount of cash.
    fn withdraw(&mut self, amount: u32) -> Result<u32, String> {
        if self.amount >= amount {
            self.amount -= amount;
            println!("Withdrew: ${}", amount);
            Ok(amount)
        } else {
            Err(format!("Insufficient funds: Requested ${}, Available ${}", amount, self.amount))
        }
    }

    fn balance(&self) -> u32 {
        self.amount
    }
}

/// The ATM struct holds the cash and handles withdrawals.
/// Uses RAII: When ATM is dropped, the cash is cleaned up automatically.
struct ATM {
    cash: Rc<RefCell<Cash>>, // Shared ownership with interior mutability
}

impl ATM {
    fn new(initial_amount: u32) -> Self {
        let cash = Rc::new(RefCell::new(Cash::new(initial_amount)));
        ATM { cash }
    }

    fn withdraw_cash(&self, amount: u32) {
        let mut cash = self.cash.borrow_mut();
        match cash.withdraw(amount) {
            Ok(_) => println!("Withdrawal successful."),
            Err(e) => println!("Withdrawal failed: {}", e),
        }
    }

    fn check_balance(&self) {
        let cash = self.cash.borrow();
        println!("Current ATM Balance: ${}", cash.balance());
    }
}

// RAII demonstrated here – cleanup is automatic
impl Drop for ATM {
    fn drop(&mut self) {
        println!("ATM is shutting down. Final balance: ${}", self.cash.borrow().balance());
        println!("Releasing resources.");
    }
}

fn main() {
    {
        let atm = ATM::new(500); // ATM starts with $500
        atm.check_balance();

        atm.withdraw_cash(100);
        atm.withdraw_cash(200);
        atm.withdraw_cash(300); // Should fail

        atm.check_balance();
    } // ATM goes out of scope, drop is called automatically here

    println!("ATM session ended.");
}
