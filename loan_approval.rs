#[derive(Debug)]
enum LoanError {
    AgeError(String),
    IncomeError(String),
}

/// Struct representing a loan applicant
struct Applicant {
    name: String,
    age: u8,
    income: u32,
    loan_amount: u32,
    co_applicant: Option<Box<Applicant>>, // optional co-applicant
}

impl Applicant {
    fn new(name: &str, age: u8, income: u32, loan_amount: u32, co_applicant: Option<Applicant>) -> Self {
        Applicant {
            name: name.to_string(),
            age,
            income,
            loan_amount,
            co_applicant: co_applicant.map(Box::new),
        }
    }

    /// Check if this applicant is eligible
    fn check_eligibility(&self) -> Result<(), LoanError> {
        if self.age < 21 || self.age > 65 {
            return Err(LoanError::AgeError(format!(
                "{} is not in eligible age range: {}",
                self.name, self.age
            )));
        }

        if self.income < 30_000 {
            return Err(LoanError::IncomeError(format!(
                "{} has insufficient income: ${}",
                self.name, self.income
            )));
        }

        Ok(())
    }

    /// Calculate total income with co-applicant (if any)
    fn total_income(&self) -> u32 {
        match &self.co_applicant {
            Some(co) => self.income + co.income,
            None => self.income,
        }
    }
}

/// Check loan approval based on income and age
fn process_loan(applicant: &Applicant) -> Result<(), LoanError> {
    // First check primary applicant
    applicant.check_eligibility()?;

    // If applicant has a co-applicant, validate them too (optional logic)
    if let Some(co) = &applicant.co_applicant {
        // Only print if co-applicant is valid
        match co.check_eligibility() {
            Ok(_) => println!("Co-applicant {} is eligible.", co.name),
            Err(e) => println!("Warning: Co-applicant not eligible: {:?}", e),
        }
    }

    // Evaluate total income for loan amount
    let total_income = applicant.total_income();
    let threshold = applicant.loan_amount / 2; // arbitrary rule: income must be >= half of loan

    if total_income >= threshold {
        println!(
            "Loan approved for {}! Total income: ${}, Loan amount: ${}",
            applicant.name, total_income, applicant.loan_amount
        );
        Ok(())
    } else {
        Err(LoanError::IncomeError(format!(
            "Combined income too low (${}) for requested loan amount (${})",
            total_income, applicant.loan_amount
        )))
    }
}

fn main() {
    let applicants = vec![
        Applicant::new("Alice", 30, 40_000, 60_000, None),
        Applicant::new("Bob", 20, 50_000, 50_000, None), // too young
        Applicant::new(
            "Charlie",
            45,
            25_000,
            70_000,
            Some(Applicant::new("Dana", 42, 30_000, 0, None)),
        ),
        Applicant::new("Eve", 34, 80_000, 90_000, None),
    ];

    let mut iter = applicants.into_iter();

    // Process each applicant using while let and match
    while let Some(applicant) = iter.next() {
        println!("\nProcessing application for {}:", applicant.name);

        match process_loan(&applicant) {
            Ok(_) => println!("Status: ✅ Approved"),
            Err(e) => match e {
                LoanError::AgeError(msg) => println!("Status: ❌ Age error - {}", msg),
                LoanError::IncomeError(msg) => println!("Status: ❌ Income error - {}", msg),
            },
        }
    }
}
