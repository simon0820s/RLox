use colored::Colorize;

pub struct RLoxError {
    pub line: usize,
    pub location: String,
    pub message: String,
}

impl RLoxError {
    pub fn new(line: usize, location: String, message: String) -> Self {
        Self {
            line,
            location,
            message,
        }
    }

    pub fn report(&self) {
        let prefix: String = format!("[line {}] Error{}", self.line, self.location);

        eprintln!(
            "{}: {}",
            prefix.red().bold(),
            self.message.white()
        );
    }
}
