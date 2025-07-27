mod file_logger;
mod logger;

/// Simulate the main logging behavior
fn run_logger(mut logger: impl logger::Logger) {
    logger.log("Starting application...");
    logger.log("Performing operations...");

    // Simulate something critical
    logger.log("Simulating a critical log entry...");

    println!("All logs completed successfully.");
}

fn main() {
    println!("--- Logging with ConsoleLogger ---");
    let console_logger = logger::ConsoleLogger;
    run_logger(console_logger);

    println!("\n--- Logging with FileLogger ---");

    match file_logger::FileLogger::new("log.txt") {
        Ok(file_logger) => {
            // Wrap with std::panic::catch_unwind to handle unwinding (if configured)
            let result = std::panic::catch_unwind(|| {
                run_logger(file_logger);
            });

            match result {
                Ok(_) => println!("Logging to file succeeded."),
                Err(_) => println!("Logger panicked (caught via unwind)."),
            }
        }
        Err(e) => {
            eprintln!("Failed to create FileLogger: {}", e);
        }
    }
}
