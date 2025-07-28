mod wallet;

use wallet::account::Wallet;
use wallet::error::WalletError;
use wallet::logger::{ConsoleLogger, Logger};

fn main() {
    let mut logger: Box<dyn Logger> = Box::new(ConsoleLogger);
    let mut wallet = Wallet::new("harshit", "1234", &mut *logger);

    // Auth
    match wallet.authenticate("1234") {
        Ok(_) => {
            println!("Authenticated.");

            wallet.deposit(500);
            wallet.transfer(200, "friend", &mut *logger).unwrap_or_else(|e| println!("{:?}", e));

            println!("Balance: {}", wallet.get_balance());
            println!("History: {:?}", wallet.get_transaction_history());
        }
        Err(e) => println!("Auth failed: {:?}", e),
    }
}
