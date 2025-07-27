use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// ----------------------------
// Account Struct
// ----------------------------
#[derive(Debug)]
struct Account {
    id: u32,
    balance: f64,
}

impl Account {
    fn new(id: u32, balance: f64) -> Self {
        Self { id, balance }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err(format!("Insufficient funds in account {}", self.id))
        }
    }
}

// ----------------------------
// Transaction Enum
// ----------------------------
#[derive(Debug)]
enum Transaction {
    Deposit { account_id: u32, amount: f64 },
    Withdraw { account_id: u32, amount: f64 },
    Transfer { from_id: u32, to_id: u32, amount: f64 },
}

// ----------------------------
// Trait Processor
// ----------------------------
trait Processor: Send {
    fn process(&mut self, txn: Transaction);
}

// ----------------------------
// Transaction Processor Struct
// ----------------------------
struct Bank {
    accounts: Arc<Mutex<Vec<Account>>>,
}

impl Bank {
    fn find_account_mut(accounts: &mut [Account], id: u32) -> Option<&mut Account> {
        accounts.iter_mut().find(|acc| acc.id == id)
    }
}

impl Processor for Bank {
    fn process(&mut self, txn: Transaction) {
        let mut accounts = self.accounts.lock().unwrap();

        match txn {
            Transaction::Deposit { account_id, amount } => {
                if let Some(acc) = Bank::find_account_mut(&mut accounts, account_id) {
                    acc.deposit(amount);
                    println!("💰 Deposit: ${:.2} to Account {}. Balance: ${:.2}", amount, account_id, acc.balance);
                } else {
                    println!("❌ Deposit failed: Account {} not found", account_id);
                }
            }

            Transaction::Withdraw { account_id, amount } => {
                if let Some(acc) = Bank::find_account_mut(&mut accounts, account_id) {
                    match acc.withdraw(amount) {
                        Ok(_) => println!("💸 Withdraw: ${:.2} from Account {}. Balance: ${:.2}", amount, account_id, acc.balance),
                        Err(e) => println!("❌ Withdraw failed: {}", e),
                    }
                } else {
                    println!("❌ Withdraw failed: Account {} not found", account_id);
                }
            }

            Transaction::Transfer { from_id, to_id, amount } => {
                let (from_opt, to_opt) = {
                    let mut from = None;
                    let mut to = None;

                    for acc in &mut *accounts {
                        if acc.id == from_id {
                            from = Some(acc as *mut Account);
                        }
                        if acc.id == to_id {
                            to = Some(acc as *mut Account);
                        }
                    }

                    (from, to)
                };

                unsafe {
                    match (from_opt, to_opt) {
                        (Some(from_ptr), Some(to_ptr)) => {
                            let from_acc = &mut *from_ptr;
                            let to_acc = &mut *to_ptr;

                            match from_acc.withdraw(amount) {
                                Ok(_) => {
                                    to_acc.deposit(amount);
                                    println!(
                                        "🔁 Transfer: ${:.2} from {} to {}. New balances: from ${:.2}, to ${:.2}",
                                        amount, from_id, to_id, from_acc.balance, to_acc.balance
                                    );
                                }
                                Err(e) => println!("❌ Transfer failed: {}", e),
                            }
                        }
                        _ => println!("❌ Transfer failed: Account(s) not found"),
                    }
                }
            }
        }
    }
}

// ----------------------------
// Main Program
// ----------------------------
fn main() {
    let accounts = vec![
        Account::new(1, 1000.0),
        Account::new(2, 500.0),
        Account::new(3, 200.0),
    ];

    let shared_accounts = Arc::new(Mutex::new(accounts));

    // Create Bank processor
    let processor: Box<dyn Processor> = Box::new(Bank {
        accounts: Arc::clone(&shared_accounts),
    });

    // Channel for transaction queue
    let (tx, rx) = mpsc::channel::<Transaction>();

    // Move processor into a background thread
    let handle = thread::spawn(move || {
        let mut processor = processor;
        for txn in rx {
            processor.process(txn);
            thread::sleep(Duration::from_millis(500)); // simulate processing delay
        }
    });

    // Enqueue transactions
    tx.send(Transaction::Deposit {
        account_id: 1,
        amount: 200.0,
    }).unwrap();

    tx.send(Transaction::Withdraw {
        account_id: 2,
        amount: 100.0,
    }).unwrap();

    tx.send(Transaction::Transfer {
        from_id: 1,
        to_id: 3,
        amount: 300.0,
    }).unwrap();

    tx.send(Transaction::Withdraw {
        account_id: 3,
        amount: 1000.0,
    }).unwrap(); // Should fail due to insufficient funds

    drop(tx); // Close channel so background thread exits
    handle.join().unwrap();

    // Final balances
    println!("\n🏁 Final account balances:");
    let accounts = shared_accounts.lock().unwrap();
    for acc in accounts.iter() {
        println!("Account {}: ${:.2}", acc.id, acc.balance);
    }
}
