use std::{
    io::{self, Write},
    process,
};
pub mod error;
pub mod token_type;
pub mod token;
use crate::error::RLoxError;

pub struct RLox {
    had_error: bool,
}

impl RLox {
    pub fn new() -> Self {
        RLox { had_error: false }
    }
    pub fn run_file(&mut self, path: &str) {
        let source: String = std::fs::read_to_string(path).expect("Failed to read file");
        self.run(&source);
        if self.had_error {
            process::exit(65)
        };
    }

    pub fn run_prompt(&mut self) {
        let stdin: io::Stdin = io::stdin();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line: String = String::new();

            if stdin.read_line(&mut line).unwrap() == 0 {
                break;
            }

            self.run(&line);
        }
    }

    fn run(&mut self, source: &str) {
        self.error(0, "Not implemented yet");
        for ch in source.chars() {
            print!("{}", ch);
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        RLoxError::new(line, "", message);
        self.had_error = true;
    }
}
