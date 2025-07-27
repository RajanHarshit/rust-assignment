use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Trait defining generalized logging behavior
trait Logger {
    fn log(&mut self, message: &str);
}

/// Console logger (stdout)
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, message: &str) {
        println!("[Console] {}", message);
    }
}

/// File-based logger
struct FileLogger {
    file: File,
}

impl FileLogger {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;

        Ok(FileLogger { file })
    }
}

impl Logger for FileLogger {
    fn log(&mut self, message: &str) {
        if let Err(e) = writeln!(self.file, "[File] {}", message) {
            // Simulate critical failure that should panic
            panic!("❌ Critical logging failure: {}", e);
        }
    }
}

/// Simulate the main logging behavior
fn run_logger(mut logger: impl Logger) {
    logger.log("Starting application...");
    logger.log("Performing operations...");

    // Simulate something critical
    logger.log("Simulating a critical log entry...");

    println!("✅ All logs completed successfully.");
}

fn main() {
    println!("--- Logging with ConsoleLogger ---");
    let console_logger = ConsoleLogger;
    run_logger(console_logger);

    println!("\n--- Logging with FileLogger ---");

    match FileLogger::new("log.txt") {
        Ok(file_logger) => {
            // Wrap with std::panic::catch_unwind to handle unwinding (if configured)
            let result = std::panic::catch_unwind(|| {
                run_logger(file_logger);
            });

            match result {
                Ok(_) => println!("✅ Logging to file succeeded."),
                Err(_) => println!("❗ Logger panicked (caught via unwind)."),
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create FileLogger: {}", e);
        }
    }
}
