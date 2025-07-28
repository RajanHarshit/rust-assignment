#[derive(Debug)]
pub enum LoanError {
    AgeError(String),
    IncomeError(String),
}