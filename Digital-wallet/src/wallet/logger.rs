pub trait Logger {
    fn log(&mut self, message: &str);
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, message: &str) {
        println!("[LOG]: {}", message);
    }
}
