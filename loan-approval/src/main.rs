mod error;
mod applicant;
use applicant::Applicant;

/// Check loan approval based on income and age
fn process_loan(Applicant: &Applicant) -> Result<(), error::LoanError> {
    // First check primary applicant::Applicant
    Applicant.check_eligibility()?;

    // If applicant::Applicant has a co-applicant::Applicant, validate them too (optional logic)
    if let Some(co) = &Applicant.co_applicant {
        // Only print if co-applicant::Applicant is valid
        match co.check_eligibility() {
            Ok(_) => println!("Co-applicant::Applicant {} is eligible.", co.name),
            Err(e) => println!("Warning: Co-applicant::Applicant not eligible: {:?}", e),
        }
    }

    // Evaluate total income for loan amount
    let total_income = Applicant.total_income();
    let threshold = Applicant.loan_amount / 2; // arbitrary rule: income must be >= half of loan

    if total_income >= threshold {
        println!(
            "Loan approved for {}! Total income: ${}, Loan amount: ${}",
            Applicant.name, total_income, Applicant.loan_amount
        );
        Ok(())
    } else {
        Err(error::LoanError::IncomeError(format!(
            "Combined income too low (${}) for requested loan amount (${})",
            total_income, Applicant.loan_amount
        )))
    }
}

fn main() {
    let Applicants = vec![
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

    let mut iter = Applicants.into_iter();

    // Process each applicant::Applicant using while let and match
    while let Some(Applicant) = iter.next() {
        println!("\nProcessing application for {}:", Applicant.name);

        match process_loan(&Applicant) {
            Ok(_) => println!("Status: ✅ Approved"),
            Err(e) => match e {
                error::LoanError::AgeError(msg) => println!("Status: ❌ Age error - {}", msg),
                error::LoanError::IncomeError(msg) => println!("Status: ❌ Income error - {}", msg),
            },
        }
    }
}
