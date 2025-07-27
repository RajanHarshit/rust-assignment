use crate::file_logger;
use std::io::Write;
/// Trait defining generalized logging behavior
pub trait Logger {
    fn log(&mut self, message: &str);
}

/// Console logger (stdout)
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, message: &str) {
        println!("[Console] {}", message);
    }
}

impl Logger for file_logger::FileLogger {
    fn log(&mut self, message: &str) {
        if let Err(e) = writeln!(self.file, "[File] {}", message) {
            // Simulate critical failure that should panic
            panic!("Critical logging failure: {}", e);
        }
    }
}