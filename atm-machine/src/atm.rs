use std::cell::RefCell;
use std::rc::Rc;
use crate::cash::Cash;
// The ATM struct holds the cash and handles withdrawals.
/// Uses RAII: When ATM is dropped, the cash is cleaned up automatically.
pub struct ATM {
    cash: Rc<RefCell<Cash>>, // Shared ownership with interior mutability
}

impl ATM {
    pub fn new(initial_amount: u32) -> Self {
        let cash = Rc::new(RefCell::new(Cash::new(initial_amount)));
        ATM { cash }
    }

    pub fn withdraw_cash(&self, amount: u32) {
        let mut cash = self.cash.borrow_mut();
        match cash.withdraw(amount) {
            Ok(_) => println!("Withdrawal successful."),
            Err(e) => println!("Withdrawal failed: {}", e),
        }
    }

    pub fn check_balance(&self) {
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