mod cash;
mod atm;

fn main() {
    {
        let atm = atm::ATM::new(500); // ATM starts with $500
        atm.check_balance();

        atm.withdraw_cash(100);
        atm.withdraw_cash(200);
        atm.withdraw_cash(300); // Should fail

        atm.check_balance();
    } // ATM goes out of scope, drop is called automatically here

    println!("ATM session ended.");
}
