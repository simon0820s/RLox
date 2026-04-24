use colored::Colorize;

pub struct RLoxError;

impl RLoxError {
    pub fn new(line: usize, location: &str, message: &str) -> Self {
        let error: Self = Self;
        error.report(line, location, message);
        error
    }

    fn report(&self, line: usize, location: &str, message: &str) {
        let prefix: String = format!("[line {}] Error{}", line, location);

        eprintln!("{}: {}", prefix.red().bold(), message.white());
    }
}