use crate::error;
/// Struct representing a loan applicant
pub struct Applicant {
    pub name: String,
    pub age: u8,
    pub income: u32,
    pub loan_amount: u32,
    pub co_applicant: Option<Box<Applicant>>, // optional co-applicant
}

impl Applicant {
    pub fn new(name: &str, age: u8, income: u32, loan_amount: u32, co_applicant: Option<Applicant>) -> Self {
        Applicant {
            name: name.to_string(),
            age,
            income,
            loan_amount,
            co_applicant: co_applicant.map(Box::new),
        }
    }

    /// Check if this applicant is eligible
    pub fn check_eligibility(&self) -> Result<(), error::LoanError> {
        if self.age < 21 || self.age > 65 {
            return Err(error::LoanError::AgeError(format!(
                "{} is not in eligible age range: {}",
                self.name, self.age
            )));
        }

        if self.income < 30_000 {
            return Err(error::LoanError::IncomeError(format!(
                "{} has insufficient income: ${}",
                self.name, self.income
            )));
        }

        Ok(())
    }

    /// Calculate total income with co-applicant (if any)
    pub fn total_income(&self) -> u32 {
        match &self.co_applicant {
            Some(co) => self.income + co.income,
            None => self.income,
        }
    }
}
